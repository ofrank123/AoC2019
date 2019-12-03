use std::fs;

fn main() {
    let input = fs::read_to_string("./input")
        .expect("Error Reading File");

    let mut codes: Vec<usize> = input.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();

    // As per spec
    codes[1] = 12;
    codes[2] = 2;

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

    println!("{}", codes[0]);
}
