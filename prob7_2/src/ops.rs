#[derive(Debug)]
pub struct Op {
    op_type: OpType,
    raw: Vec<i32>,
    parsed: Vec<i32>,
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

pub fn handle_op(out: &mut Vec<i32>, input: &mut Vec<i32>, codes: &mut Vec<i32>, op: Op, ip: usize) -> usize {
    match op.op_type {
        OpType::ADD => {
            ip + op_add(codes, op)
        },
        OpType::MUL => {
            ip + op_mul(codes, op)
        },
        OpType::IN => {
            ip + op_in(input, codes, op)
        },
        OpType::OUT => {
            ip + op_out(out, op)
        },
        OpType::JNZ => {
            let nip = op_jnz(op);
            if nip != -1 {
                nip as usize
            } else {
                ip + 3
            }
        },
        OpType::JZ => {
            let nip = op_jz(op);
            if nip != -1 {
                nip as usize
            } else {
                ip + 3
            }
        },
        OpType::LT => {
            ip + op_lt(codes, op)
        },
        OpType::EQ => {
            ip + op_eq(codes, op)
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
            panic!("Not a supported opcode: {}", op_code % 100);
        },
    };

    let mut raw: Vec<i32> = Vec::new();
    let mut parsed: Vec<i32> = Vec::new();

    op_code /= 100;
    for i in 0..reg_count {
        raw.push(codes[ip + i + 1]);
        if op_code % 10 == 0 {
            parsed.push(codes[codes[ip + i + 1] as usize])
        } else {
            parsed.push(codes[ip + i + 1]);
        }
        op_code /= 10;
    }

    Op {
        op_type,
        raw,
        parsed
    }
}

fn op_add(codes: &mut Vec<i32>, op: Op) -> usize {
    codes[op.raw[2] as usize] = op.parsed[0] + op.parsed[1];

    4
}

fn op_mul(codes: &mut Vec<i32>, op: Op) -> usize {
    codes[op.raw[2] as usize] = op.parsed[0] * op.parsed[1];

    4
}

fn op_in(input: &mut Vec<i32>, codes: &mut Vec<i32>, op: Op) -> usize {
    let num = input.pop();
    /*
    println!("In: {}", match num {
        Some(val) => val,
        None => -1,
    });
     */
    codes[op.raw[0] as usize] = num.unwrap();

    2
}

fn op_out(out: &mut Vec<i32>, op: Op) -> usize {
    out.push(op.parsed[0]);
    2
}

fn op_jnz(op: Op) -> i32 {
    if op.parsed[0] != 0 {
        op.parsed[1]
    } else {
        -1
    }
}

fn op_jz(op: Op) -> i32 {
    if op.parsed[0] == 0 {
        op.parsed[1]
    } else {
        -1
    }
}

fn op_lt(codes: &mut Vec<i32>, op: Op) -> usize {
    codes[op.raw[2] as usize] = if op.parsed[0] < op.parsed[1] {
        1
    } else {
        0
    };

    4
}

fn op_eq(codes: &mut Vec<i32>, op: Op) -> usize {
    codes[op.raw[2] as usize] = if op.parsed[0] == op.parsed[1] {
        1
    } else {
        0
    };

    4
}
