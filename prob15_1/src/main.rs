mod int_machine;
use int_machine::IntMachine;
use int_machine::Interrupt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{thread, time};

#[derive(Clone, Debug)]
struct Node {
    val: u8,
    try_m: Vec<u8>,
}

struct Bot {
    x: i32,
    y: i32,
}

fn move_bot(next_move: u8, bot: &mut Bot, m: &mut IntMachine) -> Result<u8, String>{
    m.push_input(next_move as i64);
    let out = match m.run_interact() {
        Interrupt::Output(out) => out,
        _ => return Err("Unexpected interrupt".to_string())
    };
    match out {
        0 => Ok(out as u8),
        _ => {
            match next_move {
                1 => bot.y += 1,
                2 => bot.y -= 1,
                3 => bot.x -= 1,
                4 => bot.x += 1,
                _ => return Err("Not a valid move".to_string()),
            }
            Ok(out as u8)
        }
    }
}

fn map_region(m: &mut IntMachine) -> Result<HashMap<(i32, i32), Node>, String>{
    let mut nodes = HashMap::new();
    nodes.insert((0, 0), Node {val: 1, try_m: vec![1,2,3,4]});
    let mut moves: Vec<u8> = vec![]; // stack of moves
    let mut bot = Bot{x: 0, y: 0};
    let mut in_backtrack = false;

    loop {
        thread::sleep(time::Duration::from_millis(20));
        print_nodes(&bot, &nodes, &HashSet::new());

        if in_backtrack {
            let next_move = match moves.pop().ok_or("Out of moves")? {
                1 => 2,
                2 => 1,
                3 => 4,
                4 => 3,
                _ => return Err("Not a valid move".to_string()),
            };
            match move_bot(next_move, &mut bot, m)? {
                1 => {},
                _ => return Err("Failed move in backtracking".to_string()),
            }
            if nodes.get(&(bot.x, bot.y)).ok_or("Could not find node")?.try_m.len() > 0 {
                in_backtrack = false;
            }
        } else {
            let mut c_node = nodes.get(&(bot.x, bot.y)).ok_or("Could not find node")?.clone();
            let this_node = (bot.x, bot.y);
            in_backtrack = true;
            while c_node.try_m.len() > 0 {
                let next_move = c_node.try_m.pop().ok_or("Could not pop move")?;
                let out = move_bot(next_move, &mut bot, m)?;
                match out {
                    0 => {
                        let mut temp_node = (bot.x, bot.y);
                        match next_move {
                            1 => temp_node.1 += 1,
                            2 => temp_node.1 -= 1,
                            3 => temp_node.0 -= 1,
                            4 => temp_node.0 += 1,
                            _ => return Err("Not a valid move".to_string()),
                        }
                        nodes.insert(temp_node, Node{val: 0, try_m: vec![]});
                    }
                    _ => {
                        let try_m = match next_move {
                            1 => vec![1,3,4],
                            2 => vec![2,3,4],
                            3 => vec![1,2,3],
                            4 => vec![1,2,4],
                            _ => return Err("Not a valid move".to_string()),
                        };
                        nodes.insert((bot.x, bot.y), Node{val: out, try_m}); //insert new node
                        moves.push(next_move); //push move to stack
                        in_backtrack = false;
                        break;
                    }
                }
            }
            nodes.insert(this_node, c_node); // Put node back, with updated try_m
        }

        if moves.len() == 0 {
            break;
        }
    }

    Ok(nodes)
}

fn bfs_map(nodes: &HashMap<(i32, i32), Node>) -> Result<i32, String> {
    let mut iters = 0;
    let mut checked_nodes = HashSet::new();
    checked_nodes.insert((0,0));
    loop {
        iters += 1;
        print_nodes(&Bot{x:0,y:0}, nodes, &checked_nodes);
        thread::sleep(time::Duration::from_millis(20));
        let mut new_checked = HashSet::new();
        for (x, y) in checked_nodes.iter() {
            for (x_a, y_a) in vec![(0,1),(1,0),(0,-1),(-1,0)] {
                let x = x + x_a;
                let y = y + y_a;

                let val = nodes.get(&(x,y)).ok_or("Could not find node")?.val;
                if val == 2 {
                    return Ok(iters);
                }
                // If it's free and not already checked
                if val == 1 && checked_nodes.get(&(x,y)) == None {
                    new_checked.insert((x,y));
                }
            }
        }
        for n in new_checked {
            checked_nodes.insert(n);
        }
    }
}

fn print_nodes(bot: &Bot, nodes: &HashMap<(i32, i32), Node>, checked_nodes: &HashSet<(i32, i32)>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for ((x, y), _) in nodes {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    let size_x = (max_x - min_x + 1) as usize;
    let size_y = (max_y - min_y + 1) as usize;

    let mut node_map = vec![vec!['▒'; size_x]; size_y];
    for ((x,y), n) in nodes {
        let x = (*x + min_x * -1) as usize;
        let y = (*y + min_y * -1) as usize;
        node_map[y][x] = match n.val {
            1 => ' ',
            2 => 'O',
            _ => '█',
        }
    }

    node_map[(bot.y + min_y * -1) as usize][(bot.x + min_x * -1) as usize] = 'D';
    for (x, y) in checked_nodes {
        let x = (*x + min_x * -1) as usize;
        let y = (*y + min_y * -1) as usize;
        node_map[y][x] = 'X';
    }

    print!("{}[0;0H", 27 as char);
    for r in node_map {
        for n in r {
            print!("{}", n);
        }
        println!();
    }
}

fn main() -> Result<(), String> {
    let mut m = IntMachine::new(vec![]);
    print!("{}[2J", 27 as char);
    println!("Fewest Commands Neccesary: {}", bfs_map(&map_region(&mut m)?)?);

    print!("{}[?25h", 27 as char);
    Ok(())
}
