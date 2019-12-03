use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Step {
    dir: Dir,
    dist: i32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

fn main() {
    let input = fs::read_to_string("./input")
        .expect("Error Reading File");

    let wires: Vec<&str> = input.split('\n').collect();

    let wire1 = get_steps(wires[0]);
    let wire2 = get_steps(wires[1]);

    let wire1 = get_path(wire1);
    let wire2 = get_path(wire2);

    let intersects: HashSet<_> = wire1.intersection(&wire2).collect();
    let closest = closest_point(intersects);

    println!("Closest Point: {:#?}", closest);
    println!("Dist: {}", closest.manhattan_dist());
}

fn closest_point(points: HashSet<&Point>) -> &Point {
    let mut closest: Option<&Point> = None;
    for point in points {
        match closest {
            Some(val) => {
                if point.manhattan_dist() < val.manhattan_dist() {
                    closest = Some(point);
                }
            },
            None => {
                closest = Some(point);
            }
        }
    }

    closest.unwrap()
}


fn get_steps(input: &str) -> Vec<Step> {
    let input: Vec<&str> = input.split(',').collect();
    let mut steps: Vec<Step> = Vec::with_capacity(input.len());

    for s in input {
        steps.push(Step {
            dir: match &s[0..1] {
                "R" => Dir::Right,
                "L" => Dir::Left,
                "U" => Dir::Up,
                "D" => Dir::Down,
                &_ => Dir::Up,
            },
            dist: (&s[1..s.len()]).parse()
                .expect("Not a number"),
        });
    }

    steps
}

fn get_path(steps: Vec<Step>) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::with_capacity(steps.len());
    let mut head = Point {
        x: 0,
        y: 0,
    };

    for s in steps {
        for _ in 0..s.dist {
            match s.dir {
                Dir::Right => head.x += 1,
                Dir::Left  => head.x -= 1,
                Dir::Up    => head.y += 1,
                Dir::Down  => head.y -= 1,
            }
            points.insert(Point {x: head.x, y: head.y});
        }
    }

    points
}
