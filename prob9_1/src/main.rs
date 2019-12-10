mod ops;

use std::fs;

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let mut codes: Vec<i64> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();

    let mut machine = ops::IntMachine::new(vec![1], codes);
    println!("{:#?}", machine.run());
}
