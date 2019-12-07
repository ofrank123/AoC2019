use std::collections::HashMap;
use std::fs;

fn get_orbits(bodies: &HashMap<&str, &str>, body: &str) -> i32 {
    match body {
        "COM" => {
            0
        },
        _ => {
            get_orbits(bodies, bodies.get(body).unwrap()) + 1
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input")
        .expect("could not read file");
    let mut bodies = HashMap::new();
    for o in input.split('\n') {
        let o:Vec<&str> = o.split(')').collect();
        bodies.insert(o[1], o[0]);
    }

    // println!("{:#?}", bodies);
    let mut total_orbits = 0;
    for (b, _) in &bodies {
        total_orbits += get_orbits(&bodies, b);
    }
    println!("Total Orbits: {}", total_orbits);
}
