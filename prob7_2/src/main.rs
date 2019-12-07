mod ops;

use std::fs;

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let mut codes: Vec<i32> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();

    println!("Val: {}", max_perm(&mut codes));
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
    let mut amps: Vec<(i32, i32, Vec<i32>)> = vec![(0, 0, codes.clone()); 5];
    for (i, amp) in amps.iter_mut().enumerate() {
        amp.0 = input[i];
    }

    let mut last_out = 0;
    let mut last_e_out = 0;
    let mut exit = false;

    for (i, amp) in amps.iter_mut().enumerate() {
        let mut out: Vec<i32> = Vec::new();
        let ip = intcode_machine(amp.1, &mut out, vec![last_out, amp.0], &mut amp.2);
        amp.1 = ip; // save instruction pointer
        last_out = out[0];
        if i == 4 {
            last_e_out = out[0];
        }
        if ip == -1 {
            println!("EXIT NOW");
            exit = true;
            break;
        }
    }

    loop {
        for (i, amp) in amps.iter_mut().enumerate() {
            /*
            println!("Amp: {}, {}", amp.0, amp.1);
            println!("Last out: {}", last_out);
            println!("codes: {:?}", amp.2);
             */
            let mut out: Vec<i32> = Vec::new();
            let ip = intcode_machine(amp.1, &mut out, vec![last_out], &mut amp.2);
            if ip == -1 {
                exit = true;
                break;
            }
            amp.1 = ip; // save instruction pointer
            last_out = out[0];
            if i == 4 {
                last_e_out = out[0];
            }
        }

        if exit {
            break;
        }
    }

    last_e_out
}

fn intcode_machine(eip: i32, out: &mut Vec<i32>, input: Vec<i32>, codes: &mut Vec<i32>) -> i32 {
    let mut input = input;
    let mut eip = eip;
    while codes[eip as usize] != 99 {
        let op = ops::parse_op(codes, eip as usize);
        eip = ops::handle_op(out, &mut input, codes, op, eip as usize) as i32;
        // println!("{}", eip);
        // Pause on out
        if out.len() > 0 {
            return eip;
        }
    }

    -1
}
