pub struct IntMachine {
    eip: usize,
    rbp: usize,
    out: Vec<i64>,
    input: Vec<i64>,
    ins: Vec<i64>,
    op: Op,
}

impl IntMachine {
    pub fn new(input: Vec<i64>, ins: Vec<i64>) -> IntMachine {
        IntMachine {
            eip: 0,
            rbp: 0,
            out: vec![],
            input,
            ins,
            op: Op {
                op_type: OpType::ADD,
                parsed: vec![],
            }
        }
    }

    pub fn run(&mut self) -> Vec<i64> {
        while run_machine(self) {}

        self.out.clone()
    }
}

pub fn run_machine(m: &mut IntMachine) -> bool {
    while m.ins[m.eip as usize] != 99 {
        parse_op(m);
        handle_op(m);
        if m.out.len() > 0 {
            return true;
        }
    }

    return false;
}


#[derive(Debug)]
pub struct Op {
    op_type: OpType,
    parsed: Vec<usize>,
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
    RBP,
}

fn handle_op(m: &mut IntMachine) {
    match m.op.op_type {
        OpType::ADD => {
            op_add(m);
        },
        OpType::MUL => {
            op_mul(m);
        },
        OpType::IN => {
            op_in(m);
        },
        OpType::OUT => {
            op_out(m);
        },
        OpType::JNZ => {
            op_jnz(m);
        },
        OpType::JZ => {
            op_jz(m);
        },
        OpType::LT => {
            op_lt(m);
        },
        OpType::EQ => {
            op_eq(m);
        }
        OpType::RBP => {
            op_rbp(m);
        }
    }
}

fn parse_op(m: &mut IntMachine) {
    let mut op_code = m.ins[m.eip];
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
        9=>{
            reg_count = 1;
            OpType::RBP
        },
        _=>{
            panic!("Not a supported opcode: {}", op_code % 100);
        },
    };

    let mut parsed: Vec<usize> = vec![];

    op_code /= 100;
    for i in 0..reg_count {
        //raw.push(m.ins[m.eip + i + 1]);
        match op_code % 10 {
            0 => {
                let p = m.ins[m.eip + i + 1] as usize;
                // init mem with 0s if pointer is accessing outside of current mem
                if p >= m.ins.len() {
                    for _ in (m.ins.len() - 1)..p {
                        m.ins.push(0);
                    }
                }
                parsed.push(p);
            },
            1 => {parsed.push(m.eip + i + 1);},
            2 => {
                let p = (m.rbp as i64 + m.ins[m.eip + i + 1]) as usize;
                // init mem with 0s if pointer is accessing outside of current mem
                if p >= m.ins.len() {
                    for _ in (m.ins.len() - 1)..p {
                        m.ins.push(0);
                    }
                }
                parsed.push(p);
            }
            _ => {panic!("Not a supported parameter mode");}
        }
        op_code /= 10;
    }

    m.op = Op {
        op_type,
        parsed
    };
}

fn op_add(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    let val2 = m.ins[m.op.parsed[1]];
    m.ins[m.op.parsed[2] as usize] = val1 + val2;

    m.eip += 4;
}

fn op_mul(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    let val2 = m.ins[m.op.parsed[1]];
    m.ins[m.op.parsed[2] as usize] = val1 * val2;

    m.eip += 4
}

fn op_in(m: &mut IntMachine) {
    let num = m.input.pop();
    m.ins[m.op.parsed[0]] = num.unwrap();

    m.eip += 2;
}

fn op_out(m: &mut IntMachine) {
    m.out.push(m.ins[m.op.parsed[0] as usize]);
    m.eip += 2;
}

fn op_jnz(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    let val2 = m.ins[m.op.parsed[1]];
    if val1 != 0 {
        m.eip = val2 as usize;
    } else {
        m.eip += 3;
    }
}

fn op_jz(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    let val2 = m.ins[m.op.parsed[1]];
    if val1 == 0 {
        m.eip = val2 as usize;
    } else {
        m.eip += 3;
    }
}

fn op_lt(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    let val2 = m.ins[m.op.parsed[1]];
    m.ins[m.op.parsed[2] as usize] = if val1 < val2 {
        1
    } else {
        0
    };

    m.eip += 4;
}

fn op_eq(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    let val2 = m.ins[m.op.parsed[1]];
    m.ins[m.op.parsed[2] as usize] = if val1 == val2 {
        1
    } else {
        0
    };

    m.eip += 4;
}


fn op_rbp(m: &mut IntMachine) {
    let val1 = m.ins[m.op.parsed[0]];
    m.rbp = (m.rbp as i64 + val1) as usize;

    m.eip += 2;
}
