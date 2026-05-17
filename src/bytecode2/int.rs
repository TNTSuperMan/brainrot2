use std::io::{Read, Write, stdin, stdout};

use crate::bytecode2::{op, structs::{Memory, Program}};

fn range_contains(start: i16, end: u16, offset: isize) -> bool {
    (start as isize) <= offset && offset <= (end as isize)
}

pub fn interpret_bytecode(program: &mut Program, memory: &mut Memory) {
    let mut stdout = stdout().lock();
    let mut stdin = stdin().lock();
    let mut stdio_buf = [0];
    loop {
        match program.read_u8() {
            op::OP_SETCONST => {
                let ptr = program.read_i16();
                let val = program.read_u8();
                memory[ptr] = val;
            }
            op::OP_SETLOAD => {
                let dst = program.read_i16();
                let src  = program.read_i16();
                memory[dst] = memory[src];
            }
            op::OP_ADDASSIGN => {
                let ptr = program.read_i16();
                let val = program.read_u8();
                memory[ptr] = memory[ptr].wrapping_add(val);
            }
            op::OP_ADDCONST => {
                let dst = program.read_i16();
                let src = program.read_i16();
                let val = program.read_u8();
                memory[dst] = memory[src].wrapping_add(val);
            }
            op::OP_ADDLOAD => {
                let dst = program.read_i16();
                let src1 = program.read_i16();
                let src2 = program.read_i16();
                memory[dst] = memory[src1].wrapping_add(memory[src2]);
            }
            op::OP_SUBLC => {
                let dst = program.read_i16();
                let src = program.read_i16();
                let val = program.read_u8();
                memory[dst] = memory[src].wrapping_sub(val);
            }
            op::OP_SUBCL => {
                let dst = program.read_i16();
                let val = program.read_u8();
                let src = program.read_i16();
                memory[dst] = val.wrapping_sub(memory[src]);
            }
            op::OP_SUBLL => {
                let dst = program.read_i16();
                let src1 = program.read_i16();
                let src2 = program.read_i16();
                memory[dst] = memory[src1].wrapping_sub(memory[src2]);
            }
            op::OP_MULCONST => {
                let dst = program.read_i16();
                let src = program.read_i16();
                let val = program.read_u8();
                memory[dst] = memory[src].wrapping_mul(val);
            }
            op::OP_MULLOAD => {
                let dst = program.read_i16();
                let src1 = program.read_i16();
                let src2 = program.read_i16();
                memory[dst] = memory[src1].wrapping_mul(memory[src2]);
            }
            op::OP_MULADDCONST => {
                let dst = program.read_i16();
                let val1 = program.read_u8();
                let src = program.read_i16();
                let val2 = program.read_u8();
                memory[dst] = val1.wrapping_add(memory[src].wrapping_mul(val2));
            }
            op::OP_MULADDLOAD => {
                let dst = program.read_i16();
                let src1 = program.read_i16();
                let src2 = program.read_i16();
                let val = program.read_u8();
                memory[dst] = memory[src1].wrapping_add(memory[src2].wrapping_mul(val));
            }
            op::OP_IN => {
                let ptr = program.read_i16();
                memory[ptr] = match stdin.read_exact(&mut stdio_buf) {
                    Ok(_) => stdio_buf[0],
                    Err(_) => 0,
                };
            }
            op::OP_OUTLOAD => {
                let ptr = program.read_i16();
                stdio_buf[0] = memory[ptr];
                let _ = stdout.write(&stdio_buf);
            }
            op::OP_OUTCONST => {
                let val = program.read_u8();
                stdio_buf[0] = val;
                let _ = stdout.write(&stdio_buf);
            }
            op::OP_JUMP => {
                let addr = program.read_u32();
                program.jump(addr);
            }
            op::OP_JUMPIFZERO => {
                let ptr = program.read_i16();
                let addr = program.read_u32();
                if memory[ptr] == 0 {
                    program.jump(addr);
                }
            }
            op::OP_JUMPIFNOTZERO => {
                let ptr = program.read_i16();
                let addr = program.read_u32();
                if memory[ptr] != 0 {
                    program.jump(addr);
                }
            }
            op::OP_OFFSETRANGEJUMPZERO => {
                let offset = program.read_i16();
                let _rs = program.read_i16();
                let _re = program.read_u16();
                let ptr = program.read_i16();
                let addr = program.read_u32();

                memory.offset(offset);
                // if range_contains(rs, re, memory.get_offset()) {
                // 
                // }
                if memory[ptr] == 0 {
                    program.jump(addr);
                }
            }
            op::OP_OFFSETRANGEJUMPNOTZERO => {
                let offset = program.read_i16();
                let _rs = program.read_i16();
                let _re = program.read_u16();
                let ptr = program.read_i16();
                let addr = program.read_u32();

                memory.offset(offset);
                // if range_contains(rs, re, memory.get_offset()) {
                // 
                // }
                if memory[ptr] != 0 {
                    program.jump(addr);
                }
            }
            op::OP_OFFSET => {
                let offset = program.read_i16();

                memory.offset(offset);
            }
            op::OP_OFFSETWITHRANGECHECK => {
                let offset = program.read_i16();
                let _rs = program.read_i16();
                let _re = program.read_u16();
                
                memory.offset(offset);
                // if range_contains(rs, re, memory.get_offset()) {
                // 
                // }
            }
            op::OP_FINDZERO => {
                let ptr = program.read_i16();
                let offset = program.read_i16();

                while memory[ptr] != 0 {
                    memory.offset(offset);
                }
            }
            op::OP_FINDZEROWITHRANCECHECK => {
                let ptr = program.read_i16();
                let offset = program.read_i16();
                let _rs = program.read_i16();
                let _re = program.read_u16();
                
                while memory[ptr] != 0 {
                    memory.offset(offset);
                }
                // if range_contains(rs, re, memory.get_offset()) {
                // 
                // }
            }
            op::OP_END => {
                return;
            }
            n => unreachable!("Unreachable op: {n}"),
        }
    }
}
