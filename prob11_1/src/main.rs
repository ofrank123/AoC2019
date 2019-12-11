use std::fs;
use std::collections::HashMap;
use std::collections::BTreeMap;
mod int_machine;
use int_machine::IntMachine;


enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Bot {
    loc: (i32, i32),
    dir: Dir,
}

impl Bot {
    fn move_forward(&mut self) {
        match self.dir {
            Dir::UP    => {
                self.loc.0 += 0;
                self.loc.1 += 1;
            }
            Dir::RIGHT => {
                self.loc.0 += 1;
                self.loc.1 += 0;
            },
            Dir::DOWN  => {
                self.loc.0 += 0;
                self.loc.1 += -1;
            },
            Dir::LEFT  => {
                self.loc.0 += -1;
                self.loc.1 += 0;
            },
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::UP    => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN  => Dir::LEFT,
            Dir::LEFT  => Dir::UP,
        };
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Dir::LEFT  => Dir::DOWN,
            Dir::DOWN  => Dir::RIGHT,
            Dir::RIGHT => Dir::UP,
            Dir::UP    => Dir::LEFT,
        };
    }
}

fn main() {
    let codes = fs::read_to_string("./input")
        .expect("Error Reading File");
    let codes: Vec<i64> = codes.split(',')
        .map(|code| code.parse().expect("Not a number"))
        .collect();
    let mut ins = BTreeMap::new();
    for (i, c) in codes.iter().enumerate() {
        ins.insert(i, *c);
    }

    let mut machine = IntMachine::new(vec![0], ins);
    let tiles = run_bot(&mut machine);
    println!("tiles: {}", tiles.len());
}

fn run_bot(machine: &mut IntMachine) -> HashMap<(i32,i32), u8> {
    let mut bot = Bot {
        loc: (0,0),
        dir: Dir::UP,
    };

    let mut out = machine.run_pause();
    let mut tiles = HashMap::new();

    while out.len() > 0 {
        //println!("({},{})",bot.loc.0,bot.loc.1);
        // Handle first output
        tiles.insert(bot.loc.clone(), out[0] as u8);

        out = machine.run_pause();
        // Handle second output
        match out[0] {
            0 => bot.turn_left(),
            1 => bot.turn_right(),
            _ => panic!("Error: invalid turn instruction")
        };

        // get next output
        bot.move_forward();
        machine.push_input(match tiles.get(&bot.loc) {
            Some(val) => *val as i64,
            None => 0,
        });
        out = machine.run_pause();
    }

    tiles
}
