use std::fs;
use std::f64;
use std::i64;
use std::usize;
use std::collections::BTreeMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Angle(f64);

impl Angle {
    fn canonicalize(&self) -> i64 {
        (self.0 * 10000.0).round() as i64
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl Eq for Angle {}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.canonicalize().cmp(&other.canonicalize())
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    for (i, a) in in_sight(14,17,&ast_map).iter().enumerate() {
        println!("{}: {},{}", i + 1, (a.1).0, (a.1).1);
    }
    //println!("{:#?}", in_sight(8,3,&ast_map));
}

fn in_sight(xa: usize, ya: usize, ast_map: &Vec<Vec<bool>>) -> BTreeMap<Angle, (usize, usize, f64)>{
    let mut slope_hash = BTreeMap::new();
    for (y, row) in ast_map.iter().enumerate() {
        for (x, ast_here) in row.iter().enumerate() {
            if !(xa == x && ya == y) && *ast_here {
                match slope_hash.get(&get_m((xa,ya),(x,y))) {
                    Some((_,_,val)) => {
                        if *val > get_dist((xa, ya), (x,y)) {
                            slope_hash.insert(get_m((xa,ya), (x,y)),
                                              (x,y,get_dist((xa, ya), (x,y))));
                        }
                    },
                    None => {
                        slope_hash.insert(get_m((xa,ya), (x,y)),
                                          (x,y,get_dist((xa, ya), (x,y))));
                    }
                }
                //println!("x: {}, y: {}, m: {:?}", x, y, get_m((xa,ya), (x,y)));
            }
        }
    }
    //println!("{:#?}", slope_hash);
    slope_hash
}

fn get_m(a0: (usize, usize), a1: (usize, usize)) -> Angle {
    let (x0, y0) = a0;
    let (x1, y1) = a1;
    let mut a = (y1 as f64-y0 as f64).atan2(x1 as f64- x0 as f64).to_degrees();
    if a < 0.0 {
        while a < 0.0 {
            a += 360.0;
        }
    }
    Angle((a + 90.0) % 360.0)
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
