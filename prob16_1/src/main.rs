use std::fs;
use std::error::Error;
use std::collections::VecDeque;

fn gen_pattern(p: i32, l: i32) -> Vec<i32> {
    let pattern = vec![0,1,0,-1];
    let mut p_current = 0;
    let mut ret_pattern = vec![];
    let p = p + 1;
    let mut l = l + 1;
    let mut done = false;
    loop {
        for _ in 0..p {
            ret_pattern.push(pattern[p_current]);
            l -= 1;
            if l == 0 {
                done = true;
                break;
            }
        }
        if done {
            break;
        }
        p_current = (p_current + 1) % 4;
    }
    ret_pattern.remove(0);
    ret_pattern
}

fn run_fft(phases: i32, digits: Vec<i32>) -> Vec<i32> {
    let mut phases = phases;
    let mut digits = digits;
    let len = digits.len();
    while phases > 0 {
        phases -= 1;
        println!("{:?}", digits);
        let mut new_digits = VecDeque::new();
        for (i, _) in (&digits).iter().rev().enumerate() {
            let mut total: i64 = 0;
            let pattern = gen_pattern(i as i32, len as i32);
            for (i, d) in digits.iter().rev().enumerate() {
                total += (d * pattern[i]) as i64;
            }
            new_digits.push_front((total % 10).abs() as i32);
        }
        digits = Vec::from(new_digits);
    }

    digits
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut input: Vec<char> = fs::read_to_string("./input")
        .expect("Could not read file")
        .chars()
        .collect();
    let mut digits = vec![];
    for c in input.iter().rev() {
        digits.push(c.to_string().parse()?)
    }
    let digits = Vec::from(digits);
    println!("{:?}",digits);
    for d in run_fft(100, digits).iter().rev() {
        print!("{}",d);
    }
    println!();

    Ok(())
}
