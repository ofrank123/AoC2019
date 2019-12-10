mod int_machine;

use std::fs;
use std::collections::BTreeMap;

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let mut codes: Vec<i64> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();
    let mut ins = BTreeMap::new();
    for (i, c) in codes.iter().enumerate() {
        ins.insert(i, *c);
    }
    //println!("{:#?}", ins);

    let mut machine = int_machine::IntMachine::new(vec![2], ins);
    println!("{:#?}", machine.run());
}
