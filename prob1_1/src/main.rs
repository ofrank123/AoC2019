use std::fs;

fn main() {
    let input = fs::read_to_string("./input")
        .expect("Error Reading File");

    let mut total_fuel = 0;

    for m in input.split('\n') {
        let m: f64 = match m.parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        let fuel: i32 = ((m / 3.0).floor() as i32) - 2;
        total_fuel += fuel;
    }

    println!("Fuel: {}", total_fuel);
}
