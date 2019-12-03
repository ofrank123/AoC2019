use std::fs;

fn main() {
    let input = fs::read_to_string("./input")
        .expect("Error Reading File");

    let mut total_fuel = 0.0;

    for m in input.split('\n') {
        let m: f64 = match m.parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        let mut fuel = calc_fuel(m);
        while fuel > 0.0 {
            total_fuel += fuel;
            fuel = calc_fuel(fuel);
        }
    }

    println!("Fuel: {}", total_fuel);
}

fn calc_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
