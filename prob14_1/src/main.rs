use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("./input")
        .expect("Could not read file");
    let reactions: Vec<&str> = input.split('\n').collect();
    let reactions = get_reactions(reactions)?;

    let mut waste = HashMap::new();
    println!("A ore requirement: {}", get_ore("FUEL", &mut waste, &reactions)?);
    Ok(())
}

fn get_ore(res: &str, waste: &mut HashMap<String, i32>, reactions: &HashSet<Reaction>) -> Result<i32, String> {
    let react = reactions.get(&Reaction{res: res.to_string(),amt:0,pres:vec![]})
        .ok_or("No such reaction")?;

    let mut total_ore = 0;
    let mut current_res = match waste.get(&react.res) {
        Some(val) => *val,
        None => 0,
    };
    for (name, qty) in &react.pres {
        if name == "ORE" {
            total_ore += *qty;
        } else {
            let mut current_pre = match waste.get(name){
                Some(val) => *val,
                None => 0,
            };
            current_pre -= qty; // subtract the amount we need
            waste.insert(name.to_string(), current_pre); // update the waste bucket
            // If we need more of this precursor, get more
            while current_pre < 0 {
                total_ore += get_ore(name, waste, reactions)?;
                current_pre = *waste.get(name).ok_or("Could not find precursor")?;
            }
        }
    }

    current_res += react.amt;
    waste.insert(react.res.clone(), current_res);
    Ok(total_ore)
}

#[derive(Debug)]
struct Reaction {
    res: String,
    amt: i32,
    pres: Vec<(String, i32)>
}

impl Hash for Reaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.res.hash(state);
    }
}
impl PartialEq for Reaction {
    fn eq(&self, other: &Self) -> bool {
        self.res == other.res
    }
}
impl Eq for Reaction {}

fn get_reactions(reacts: Vec<&str>) -> Result<HashSet<Reaction>, String> {
    let mut reactions = HashSet::new();
    for r in reacts {
        let mut sides = r.split(" => ");
        let pres_str = sides.next().ok_or("Parse error")?;
        let res = sides.next().ok_or("Parse error")?;
        let mut pres = vec![];
        for p in pres_str.split(", ") {
            let mut p_comp = p.split(" ");
            let amt:i32 = p_comp.next().ok_or("Parse error")?.parse().unwrap();
            let name = p_comp.next().ok_or("Parse error")?;
            let name = name.to_string();
            pres.push((name, amt));
        }
        let mut res = res.split(" ");
        let amt: i32 = res.next().ok_or("Parse error")?.parse().unwrap();
        let res = res.next().ok_or("Parse error")?;
        let res = res.to_string();
        reactions.insert(Reaction {res, amt, pres});
    }

    Ok(reactions)
}
