use std::fs;
use std::error::Error;
use std::collections::VecDeque;
fn run_fft(phases: i32, digits: Vec<i32>) -> Vec<i32> {
    let mut phases = phases;

    let mut offset: usize = 0;
    let mut c = 0;
    for d in (&digits[..7]).iter().rev() {
        offset += *d as usize * (10 as usize).pow(c);
        c += 1;
    }
    let mut digits = Vec::from(&digits[offset..]);
    while phases > 0 {
        phases -= 1;
        //println!("PHASE: {}", phases);
        let mut new_digits = VecDeque::new();
        for d in digits.iter().rev() {
            if new_digits.len() == 0 {
                new_digits.push_front(*d);
            } else {
                new_digits.push_front((d + new_digits[0]) % 10);
            }
        }
        digits = Vec::from(new_digits);
    }

    digits
}

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<char> = fs::read_to_string("./input")
        .expect("Could not read file")
        .chars()
        .collect();
    let mut digits: Vec<i32> = vec![];
    for _ in 0..10000 {
        for c in input.iter() {
            digits.push(c.to_string().parse()?)
        }
    }
    for d in (&run_fft(100, digits)[..8]).iter() {
        print!("{}",d);
    }
    println!();

    Ok(())
}
