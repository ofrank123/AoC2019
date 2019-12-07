mod ops;

use std::fs;

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let mut codes: Vec<i32> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();

    println!("Max val: {}", max_perm(&mut codes));
}

fn max_perm(codes: &mut Vec<i32>) -> i32 {
    let mut max = 0;

    for p in get_perms() {
        let val = try_seq(p, codes);
        if val > max {
            max = val;
        }
    }

    max
}

fn get_perms()-> Vec<Vec<i32>> {
    let perms = fs::read_to_string("./perms")
        .expect("Error Reading File");
    let perms: Vec<&str> = perms.split('\n').collect();
    let perms: Vec<Vec<&str>> = perms.iter().map(|p| p.split(',').collect()).collect();
    perms.iter()
        .map(|p| p.iter()
             .map(|n|n.parse().expect("not a number")).collect())
        .collect()
}

fn try_seq(input: Vec<i32>, codes: &Vec<i32>) -> i32 {
    let mut last_out = 0;
    for &i in input.iter() {
        let mut out: Vec<i32> = Vec::new();
        intcode_machine(&mut out, vec![last_out, i], &mut codes.clone());
        last_out = out[0];
    }
    last_out
}

fn intcode_machine(out: &mut Vec<i32>, input: Vec<i32>, codes: &mut Vec<i32>) {
    let mut ins_ptr = 0;
    let mut input = input;
    while codes[ins_ptr] != 99 {
        let op = ops::parse_op(codes, ins_ptr);
        ops::handle_op(out, &mut input, codes, op, &mut ins_ptr);
    }
}
