mod ops;

use std::fs;

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let mut codes: Vec<i32> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();
    intcode_machine(&mut codes);
}
fn intcode_machine(codes: &mut Vec<i32>) {
    let mut ins_ptr = 0;
    while codes[ins_ptr] != 99 {
        let op = ops::parse_op(codes, ins_ptr);
        ops::handle_op(codes, op, &mut ins_ptr);
    }
}
