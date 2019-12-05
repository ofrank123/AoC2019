use std::io;

#[derive(Debug)]
pub struct Op {
    op_type: OpType,
    regs: Vec<i32>,
    modes: Vec<u8>,
}

#[derive(Debug)]
enum OpType {
    ADD,
    MUL,
    IN,
    OUT,
    JZ,
    JNZ,
    LT,
    EQ,
}

pub fn handle_op(codes: &mut Vec<i32>, op: Op, ip: &mut usize) {
    match op.op_type {
        OpType::ADD => {
            *ip += op_add(codes, op);
        },
        OpType::MUL => {
            *ip += op_mul(codes, op);
        },
        OpType::IN => {
            *ip += op_in(codes, op);
        },
        OpType::OUT => {
            *ip += op_out(codes, op);
        },
        OpType::JNZ => {
            let nip = op_jnz(codes, op);
            if nip != -1 {
                *ip = nip as usize;
            } else {
                *ip += 3;
            };
        },
        OpType::JZ => {
            let nip = op_jz(codes, op);
            if nip != -1 {
                *ip = nip as usize
            } else {
                *ip += 3
            };
        },
        OpType::LT => {
            *ip += op_lt(codes, op);
        },
        OpType::EQ => {
            *ip += op_eq(codes, op);
        }
    }
}

pub fn parse_op(codes: &Vec<i32>, ip: usize) -> Op {
    let mut op_code = codes[ip];
    let reg_count: usize;
    let op_type = match op_code % 100 {
        1=>{
            reg_count = 3;
            OpType::ADD
        },
        2=>{
            reg_count = 3;
            OpType::MUL
        },
        3=>{
            reg_count = 1;
            OpType::IN
        },
        4=>{
            reg_count = 1;
            OpType::OUT
        },
        5=>{
            reg_count = 2;
            OpType::JNZ
        },
        6=>{
            reg_count = 2;
            OpType::JZ
        },
        7=>{
            reg_count = 3;
            OpType::LT
        },
        8=>{
            reg_count = 3;
            OpType::EQ
        },
        _=>{
            panic!("Not a supported opcode");
        },
    };

    let mut regs: Vec<i32> = Vec::with_capacity(reg_count);
    let mut modes: Vec<u8> = Vec::with_capacity(reg_count);

    op_code /= 100;
    for i in 0..reg_count {
        regs.push(codes[ip + i + 1]);
        modes.push((op_code % 10) as u8);
        op_code /= 10;
    }

    Op {
        op_type,
        regs,
        modes
    }
}

fn op_add(codes: &mut Vec<i32>, op: Op) -> usize {
    let val1 = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    let val2 = if op.modes[1] == 0 {
        codes[op.regs[1] as usize] as i32
    } else {
        op.regs[1]
    };
    codes[op.regs[2] as usize] = val1 + val2;

    4
}

fn op_mul(codes: &mut Vec<i32>, op: Op) -> usize {
    let val1 = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    let val2 = if op.modes[1] == 0 {
        codes[op.regs[1] as usize] as i32
    } else {
        op.regs[1]
    };
    codes[op.regs[2] as usize] = val1 * val2;

    4
}

fn op_in(codes: &mut Vec<i32>, op: Op) -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Could not get input");
    let input: i32 = input.trim().parse()
        .expect("Not a number");
    codes[op.regs[0] as usize] = input;

    2
}

fn op_out(codes: &mut Vec<i32>, op: Op) -> usize {
    let val = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    println!("{}", val);

    2
}

fn op_jnz(codes: &mut Vec<i32>, op: Op) -> i32 {
    let val1 = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    let val2 = if op.modes[1] == 0 {
        codes[op.regs[1] as usize] as i32
    } else {
        op.regs[1]
    };

    if val1 != 0 {
        val2
    } else {
        -1
    }
}

fn op_jz(codes: &mut Vec<i32>, op: Op) -> i32 {
    let val1 = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    let val2 = if op.modes[1] == 0 {
        codes[op.regs[1] as usize] as i32
    } else {
        op.regs[1]
    };

    if val1 == 0 {
        val2
    } else {
        -1
    }
}

fn op_lt(codes: &mut Vec<i32>, op: Op) -> usize {
    let val1 = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    let val2 = if op.modes[1] == 0 {
        codes[op.regs[1] as usize] as i32
    } else {
        op.regs[1]
    };

    if val1 < val2 {
        codes[op.regs[2] as usize] = 1;
    } else {
        codes[op.regs[2] as usize] = 0;
    }

    4
}

fn op_eq(codes: &mut Vec<i32>, op: Op) -> usize {
    let val1 = if op.modes[0] == 0 {
        codes[op.regs[0] as usize] as i32
    } else {
        op.regs[0]
    };

    let val2 = if op.modes[1] == 0 {
        codes[op.regs[1] as usize] as i32
    } else {
        op.regs[1]
    };

    if val1 == val2 {
        codes[op.regs[2] as usize] = 1;
    } else {
        codes[op.regs[2] as usize] = 0;
    }

    4
}
