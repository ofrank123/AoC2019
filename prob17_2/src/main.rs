mod int_machine;
use int_machine::IntMachine;
use int_machine::Interrupt;
use std::io;

#[derive(Debug)]
struct Bot {
    x: usize,
    y: usize,
    dir: u8,
}

fn get_bot(map: &Vec<Vec<char>>) -> Result<Bot, &str> {
    for (y, r) in map.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            match c {
                'v' => return Ok(Bot {
                    x, y,
                    dir: 0
                }),
                '>' => return Ok(Bot {
                    x, y,
                    dir: 1
                }),
                '^' => return Ok(Bot {
                    x, y,
                    dir: 2
                }),
                '<' => return Ok(Bot {
                    x, y,
                    dir: 3
                }),
                _ => {}
            }
        }
    }
    Err("No Bot sound")
}

fn find_path(map: &Vec<Vec<char>>) -> Result<Vec<char>, &str>{
    let mut len = 0;
    let mut retvec = vec![];
    let mut bot = get_bot(map)?;
    loop {
        if get_forward(&mut bot, map) == '#' {
            len += 1;
            //println!("{:?}", len);
            let pos = match bot.dir {
                0 => (bot.x as i32, bot.y as i32 + 1),
                1 => (bot.x as i32 + 1, bot.y as i32),
                2 => (bot.x as i32, bot.y as i32 - 1),
                3 => (bot.x as i32 - 1, bot.y as i32),
                _ => panic!("Direction not recognized")
            };
            bot.x = pos.0 as usize;
            bot.y = pos.1 as usize;
        } else {
            if len > 0 {
                let mut chars = vec![];
                for c in len.to_string().chars() {
                    chars.push(c);
                }
                retvec.append(&mut chars);
            }
            len = 0;
            match turn(&mut bot, map) {
                Turn::RIGHT => {retvec.push('R');},
                Turn::LEFT  => {retvec.push('L');},
                Turn::NONE  => {break;}
            }
        }
    }
    Ok(retvec)
}

fn get_pos((x, y): (i32, i32), map: &Vec<Vec<char>>) -> char {
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        '.'
    } else {
        map[y as usize][x as usize]
    }
}

enum Turn {
    RIGHT,
    LEFT,
    NONE,
}

fn turn(bot: &mut Bot, map: &Vec<Vec<char>>) -> Turn {
    let x = bot.x as i32;
    let y = bot.y as i32;
    match bot.dir {
        0 => {
            if get_pos((x - 1, y), map) == '#' {
                bot.dir = 3;
                Turn::RIGHT
            } else if get_pos((x + 1, y), map) == '#' {
                bot.dir = 1;
                Turn::LEFT
            } else {
                Turn::NONE
            }
        },
        1 => {
            if get_pos((x, y + 1), map) == '#' {
                bot.dir = 0;
                Turn::RIGHT
            } else if get_pos((x, y - 1), map) == '#' {
                bot.dir = 2;
                Turn::LEFT
            } else {
                Turn::NONE
            }
        },
        2 => {
            if get_pos((x + 1, y), map) == '#' {
                bot.dir = 1;
                Turn::RIGHT
            } else if get_pos((x - 1, y), map) == '#' {
                bot.dir = 3;
                Turn::LEFT
            } else {
                Turn::NONE
            }
        },
        3 => {
            if get_pos((x, y - 1), map) == '#' {
                bot.dir = 2;
                Turn::RIGHT
            } else if get_pos((x, y + 1), map) == '#' {
                bot.dir = 0;
                Turn::LEFT
            } else {
                Turn::NONE
            }
        },
        _ => {
            panic!("Direction not recognized")
        }
    }
}

fn get_forward(bot: &Bot, map: &Vec<Vec<char>>) -> char {
    let pos = match bot.dir {
        0 => (bot.x as i32, bot.y as i32 + 1),
        1 => (bot.x as i32 + 1, bot.y as i32),
        2 => (bot.x as i32, bot.y as i32 - 1),
        3 => (bot.x as i32 - 1, bot.y as i32),
        _ => panic!("Direction not recognized")
    };
    get_pos(pos, map)
}

fn main() -> Result<(), String> {
    let input_vec = vec![
        'A',',',
        'C',',',
        'C',',',
        'A',',',
        'B',',',
        'A',',',
        'B',',',
        'A',',',
        'B',',',
        'C','\n',
        'L',',',
        '6',',',
        'R',',',
        '1','2',',',
        'R',',',
        '8','\n',
        'R',',',
        '1','2',',',
        'L',',',
        '1','2',',',
        'L',',',
        '4',',',
        'L',',',
        '4','\n',
        'R',',',
        '8',',',
        'R',',',
        '1','2',',',
        'L',',',
        '1','2','\n',
        'n','\n',
    ];
    let input_vec: Vec<i64> = input_vec.iter().rev().map(|i| *i as i64).collect();
    println!("{:?}", input_vec);
    let mut m = IntMachine::new(input_vec);
    m.set_addr(0, 2);
    let mut out = m.run_interact();
    /*
    let mut map = vec![];
    let mut row = vec![];
     */
    let mut lastout = ' ';
    let mut char_buf = vec![];
    loop {
        match out {
            Interrupt::Exit => {
                break;
            },
            Interrupt::Output(val) => {
                if val < 255 {
                    let val = val as u8 as char;
                    lastout = val;
                    char_buf.push(val);
                    print!("{}", val);
                } else {
                    print!("{}", val);
                }

            },
            Interrupt::Input => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                m.push_input(input.as_bytes()[0] as i64);
                println!("{:?}", input.as_bytes());
            }
        }
        out = m.run_interact();
    }
    /*
    let map = Vec::from(&map[..map.len() - 1]);

    for c in find_path(&map)? {
        if c == 'L' || c == 'R' {
            print!("{}", ',');
        }
        print!("{}", c);
    }
    println!();
     */
    Ok(())
}
