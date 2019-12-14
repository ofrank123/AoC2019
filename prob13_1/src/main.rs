mod int_machine;
use int_machine::IntMachine;
use int_machine::Interrupt;
use std::collections::HashMap;
use std::{thread, time};
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let mut m = IntMachine::new(vec![]);

    let mut screen_map = HashMap::new();

    let mut draw_ins = vec![];

    //let device_state = DeviceState::new();
    loop {
        match m.run_interact() {
            Interrupt::Output(val) => {
                draw_ins.push(val);
                if draw_ins.len() == 3 {
                    screen_map.insert((draw_ins[0], draw_ins[1]), draw_ins[2]);
                    draw_ins = vec![]; // Clear the instruction buffer
                }
            }
            Interrupt::Input => {
                print_screen(&screen_map);

                thread::sleep(time::Duration::from_millis(5));
                //m.push_input(get_in(&device_state));
                m.push_input(choose_in(&screen_map));
            }
            Interrupt::Exit => {
                break;
            }
        }
    }

    let mut c = 0;
    for ((_, _), id) in screen_map {
        if id == 2 {
            c += 1;
        }
    }
    println!("{}", c);
}

fn choose_in(screen_map: &HashMap<(i64, i64), i64>) -> i64 {
    let mut ball_x = 0;
    let mut paddle_x = 0;
    for ((x, _), id) in screen_map {
        if *id == 4 {
            ball_x = *x;
        }
        if *id == 3 {
            paddle_x = *x;
        }
    }
    if ball_x > paddle_x {
        1
    } else if ball_x < paddle_x {
        -1
    } else {
        0
    }
}

fn get_in(device_state: &DeviceState) -> i64 {
    let keys: Vec<Keycode> = device_state.get_keys();
    let mut retval: i64 = 0;
    for key in keys {
        match key {
            Keycode::Z => {
                retval -= 1;
            }
            Keycode::X => {
                retval += 1;
            }
            _ => {}
        }
    }
    retval
}

fn print_screen(screen_map: &HashMap<(i64, i64), i64>) {
    let mut size_x = 0;
    for ((x, _), _) in screen_map {
        if *x > size_x {size_x = *x;}
    }
    let mut size_y = 0;
    for ((_, y), _) in screen_map {
        if *y > size_y {size_y = *y;}
    }
    let size_x = (size_x + 1) as usize;
    let size_y = (size_y + 1) as usize;

    let mut screen = vec![vec![' '; size_x]; size_y];
    let mut score = 0;
    for ((x, y), id) in screen_map {
        if *x != -1 {
            let x = *x as usize;
            let y = *y as usize;
            let tile = match *id {
                0 => ' ',
                1 => '█',
                2 => '▒',
                3 => '▔',
                4 => 'O',
                _ => panic!("Not a support tile id"),
            };
            screen[y][x] = tile;
        } else {
            score = *id;
        }
    }
    println!("SCORE: {}", score);
    for r in screen {
        for t in r {
            print!("{}", t);
        }
        println!();
    }
}
