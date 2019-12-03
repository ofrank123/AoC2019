use std::fs;

fn main() {
    for noun in 0..99 {
        for verb in 0..99 {
            if try_vals(noun, verb) == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn try_vals(noun: i32, verb: i32) -> i32 {
    let input = fs::read_to_string("./input")
        .expect("Error Reading File");

    let mut codes: Vec<usize> = input.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();

    // As per spec
    codes[1] = noun as usize;
    codes[2] = verb as usize;

    let mut counter = 0;
    while codes[counter] != 99 {
        match codes[counter] {
            1 => {
                let loc = codes[counter + 3];
                codes[loc] = codes[codes[counter+1]] + codes[codes[counter+2]];
            },
            2 => {
                let loc = codes[counter + 3];
                codes[loc] = codes[codes[counter+1]] * codes[codes[counter+2]];
            },
            _ => {
                println!("ERROR!");
            }
        }

        counter += 4;
    }

    codes[0] as i32
}
