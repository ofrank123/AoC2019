use std::fs;
use std::f64;
use std::i64;
use std::usize;
use std::collections::HashMap;
use std::hash::{Hasher, Hash};

#[derive(Debug)]
struct Slope(f64, bool, bool);

impl Slope {
    fn canonicalize(&self) -> i64 {
        if self.0.is_infinite() && self.0.is_sign_positive() {
            i64::max_value()
        } else if self.0.is_infinite() && self.0.is_sign_negative() {
            i64::min_value()
        } else {
            //println!("canon: {}", (self.0 * 1000.0).round() as i64);
            (self.0 * 1000.0).round() as i64
        }
    }
}

impl PartialEq for Slope {
    fn eq(&self, other: &Slope) -> bool {
        self.canonicalize() == other.canonicalize() && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Slope {}

impl Hash for Slope {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        (self.canonicalize(), self.1, self.2).hash(state);
    }
}

fn main() {
    let input = fs::read_to_string("./input")
        .expect("Could not read file");
    let input: Vec<&str> = input.split('\n').collect();
    let mut ast_map: Vec<Vec<bool>> = vec![];
    for l in input {
        ast_map.push(l.chars().map(|c| !(c=='.')).collect());
    }
    //println!("{:#?}", ast_map);
    //println!("{}", in_sight(2,2,&ast_map));
    println!("{:?}", get_max_asteroid(&ast_map));
}

fn get_max_asteroid(ast_map: &Vec<Vec<bool>>) -> (usize, usize, usize){
    let mut asts:Vec<(usize, usize, usize)> = vec![];
    for (y, row) in ast_map.iter().enumerate() {
        for (x, ast_here) in row.iter().enumerate() {
            if *ast_here {
                asts.push((x, y, in_sight(x, y, ast_map)));
            }
        }
    }

    println!("{:#?}",asts);
    let mut max = (0, 0, 0);
    for a in asts {
        if a.2 > max.2 {
            max = a;
        }
    }
    max
}

fn in_sight(xa: usize, ya: usize, ast_map: &Vec<Vec<bool>>) -> usize {
    let mut slope_hash: HashMap<Slope, f64> = HashMap::new();
    for (y, row) in ast_map.iter().enumerate() {
        for (x, ast_here) in row.iter().enumerate() {
            if !(xa == x && ya == y) && *ast_here {
                //println!("x: {}, y: {}, m: {:?}", x, y, get_m((xa,ya), (x,y)));
                slope_hash.insert(get_m((xa,ya), (x,y)),
                                  get_dist((xa, ya), (x,y)));
            }
        }
    }
    //println!("{:#?}", slope_hash);
    slope_hash.len()
}

fn get_m(a0: (usize, usize), a1: (usize, usize)) -> Slope {
    let (x0, y0) = a0;
    let (x1, y1) = a1;

    Slope((x1 as f64 - x0 as f64) /
          (y1 as f64 - y0 as f64),
          (x1 as i64 - x0 as i64) > 0,
          (y1 as i64 - y0 as i64) > 0)
}

fn get_dist(a0: (usize, usize), a1: (usize, usize)) -> f64 {
    let (x0, y0) = a0;
    let (x1, y1) = a1;
    let x0: f64 = x0 as f64;
    let y0: f64 = y0 as f64;
    let x1: f64 = x1 as f64;
    let y1: f64 = y1 as f64;

    ((x1-x0)*(x1-x0)+(y1-y0)*(y1-y0)).sqrt()
}
