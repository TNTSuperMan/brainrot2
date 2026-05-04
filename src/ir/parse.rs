use std::{cmp::max, ops::Range};

use crate::ir::{error::SyntaxError, ir::{IR, IROp}};

#[derive(PartialEq, Eq)]
enum SimpleOp {
    Add,
    Set,
}

impl IR {
    pub fn parse(code: &str) -> Result<(Vec<IR>, u8), SyntaxError> {
        let mut mul_offset = 0;
        let mut insts = vec![];
        let mut loop_stack = vec![];
        let mut pointer = 0;
        let mut split = false;

        for (i, c) in code.chars().enumerate() {
            let loc = i..(i + 1);
            match c {
                '>' => pointer += 1,
                '<' => pointer -= 1,
                '#' => insts.push(IR {
                    pointer,
                    opcode: IROp::Breakpoint,
                    loc,
                }),
                '.' => insts.push(IR {
                    pointer,
                    opcode: IROp::Out,
                    loc,
                }),
                ',' => insts.push(IR {
                    pointer,
                    opcode: IROp::In,
                    loc,
                }),
                '+' | '-' => {
                    let dir = if c == '+' { 1 } else { 255 };
                    if let Some(IR {
                        pointer: last_ptr,
                        opcode: IROp::Add(v) | IROp::Set(v),
                        loc,
                    }) = insts.last_mut()
                    {
                        if *last_ptr == pointer && !split {
                            *v = v.wrapping_add(dir);
                            loc.end = i + 1;
                            continue;
                        }
                    }
                    insts.push(IR {
                        pointer,
                        opcode: IROp::Add(dir),
                        loc,
                    });
                    split = false;
                }
                '[' => {
                    loop_stack.push(insts.len());
                    insts.push(IR {
                        pointer,
                        opcode: IROp::JumpZero(0),
                        loc,
                    });
                }
                ']' => {
                    let start_at = loop_stack
                        .pop()
                        .ok_or(SyntaxError::UnmatchedClosingBracket)?;
                    let start_ptr = insts[start_at].pointer;
                    let end_ptr = pointer;
                    let children = &insts[(start_at + 1)..];

                    if start_ptr != end_ptr {
                        pointer = start_ptr;
                        insts.push(IR {
                            pointer,
                            opcode: IROp::JumpNotZeroWithOffset(end_ptr - start_ptr, start_at + 1),
                            loc,
                        });
                        insts[start_at].opcode = IROp::JumpZero(insts.len());
                    } else {
                        if children.len() == 0 {
                            insts.pop();
                            continue;
                        }
                        if children.len() == 1
                            && children[0].pointer == pointer
                            && children[0].opcode == IROp::Add(255)
                        {
                            let start = insts[start_at].loc.start;
                            insts.truncate(start_at);
                            insts.push(IR {
                                pointer,
                                opcode: IROp::Set(0),
                                loc: start..(i + 1),
                            });
                            continue;
                        }

                        if let Some(muls) = children
                            .iter()
                            .map(|ir| match ir.opcode {
                                IROp::Add(val) => {
                                    Some((SimpleOp::Add, ir.pointer, val, ir.loc.clone()))
                                }
                                IROp::Set(val) => {
                                    Some((SimpleOp::Set, ir.pointer, val, ir.loc.clone()))
                                }
                                _ => None,
                            })
                            .collect::<Option<Vec<(SimpleOp, i16, u8, Range<usize>)>>>()
                            .as_mut()
                        {
                            if let Some(p) = muls.iter().position(|op| {
                                op.0 == SimpleOp::Add && op.1 == pointer && op.2 == 255
                            }) {
                                muls.remove(p);
                                if muls.iter().all(|op| op.1 != pointer) {
                                    insts.truncate(start_at + 1);

                                    for (op, ptr, val, loc) in muls.iter() {
                                        insts.push(IR {
                                            pointer: *ptr,
                                            opcode: match op {
                                                SimpleOp::Add => IROp::MulAdd(pointer, *val),
                                                SimpleOp::Set => IROp::Set(*val),
                                            },
                                            loc: loc.clone(),
                                        });
                                    }
                                    insts.push(IR {
                                        pointer,
                                        opcode: IROp::Set(0),
                                        loc: loc.clone(),
                                    });
                                    if muls.iter().any(|(op, ..)| *op == SimpleOp::Set) {
                                        insts[start_at].opcode = IROp::JumpZero(insts.len());
                                        split = true;
                                    } else {
                                        let offset = muls.iter().fold(0, |m, f| max(m, pointer - f.1));
                                        mul_offset = max(mul_offset, offset as u8);
                                        insts.remove(start_at);
                                    }
                                    continue;
                                }
                            }
                        }

                        insts.push(IR {
                            pointer,
                            opcode: IROp::JumpNotZero(start_at + 1),
                            loc,
                        });
                        insts[start_at].opcode = IROp::JumpZero(insts.len());
                    }
                }
                _ => {}
            }
        }

        if loop_stack.len() != 0 {
            return Err(SyntaxError::UnmatchedOpeningBracket);
        }

        Ok((insts, mul_offset))
    }
}
