mod int_machine;
use int_machine::IntMachine;
use int_machine::Interrupt;

fn main() {
    let mut m = IntMachine::new(vec![]);
    let mut out = m.run_interact();
    let mut map = vec![];
    let mut row = vec![];
    loop {
        match out {
            Interrupt::Exit => {
                break;
            },
            Interrupt::Output(val) => {
                let val = val as u8 as char;
                match val {
                    '\n' => {
                        map.push(row);
                        row = vec![];
                    },
                    _ => {
                        row.push(val);
                    }
                }
                print!("{}", val as u8 as char);
            },
            Interrupt::Input => {
                println!("Unhandled Input");
                break;
            }
        }
        out = m.run_interact();
    }
    let map = Vec::from(&map[..map.len() - 1]);

    println!("{}", calc_align_params(&get_intersections(&map)));
}

fn get_intersections(map: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let mut retvec = vec![];
    for (y, r) in map.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if *c == '#' {
                if x > 0 && y > 0 &&
                    x < r.len()-1 && y < map.len()-1 &&
                    map[y + 1][x] == '#' &&
                    map[y - 1][x] == '#' &&
                    map[y][x + 1] == '#' &&
                    map[y][x - 1] == '#' {
                        retvec.push((x,y));
                }
            }
        }
    }
    retvec
}

fn calc_align_params(intersects: &Vec<(usize, usize)>) -> i32 {
    let mut total = 0;
    for (x, y) in intersects {
        total += *x as i32 * *y as i32;
    }
    total
}
