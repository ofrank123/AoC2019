use std::collections::HashMap;
use std::fs;

fn get_path(bodies: &HashMap<&str, &str>, body: &str, c: i32) -> Vec<(String, i32)> {
    match body {
        "COM" => {
            vec![("COM".to_string(), c)]
        },
        _ => {
            let mut arr = get_path(bodies, bodies.get(body).unwrap(), c+1);
            arr.append(&mut vec![(body.to_string(), c)]);
            arr
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

    let you_path = get_path(&bodies, "YOU", 0);
    let san_path = get_path(&bodies, "SAN", 0);

    //println!("Path: {:#?}", you_path);

    let mut a = 0;
    for i in 0..you_path.len() {
        if(you_path[i].0 != san_path[i].0) {
            a = you_path[i-1].1 + san_path[i-1].1 - 2;
            break;
        }
    }

    println!("Distance: {}", a);
}
