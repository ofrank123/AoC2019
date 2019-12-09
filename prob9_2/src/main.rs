mod int_machine;

use std::fs;

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let mut codes: Vec<i64> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();

    let mut machine = int_machine::IntMachine::new(vec![2], codes);
    println!("{:#?}", machine.run());
}
