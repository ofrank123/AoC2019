use std::fs;

#[derive(Debug)]
struct Step {
    dir: Dir,
    dist: i32,
}

#[derive(Debug, Hash, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    signal_dist: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
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

    let intersects: Vec<Point> = wire_intersects(wire1, wire2);
    let closest = closest_point(intersects);

    println!("Closest Point: {:#?}", closest);
    println!("Dist: {}", closest.signal_dist);
}

fn closest_point(points: Vec<Point>) -> Point {
    let mut closest: Option<Point> = None;
    for point in points {
        match closest {
            Some(val) => {
                if point.signal_dist < val.signal_dist {
                    println!("Test");
                    closest = Some(Point {
                        x: point.x,
                        y: point.y,
                        signal_dist: point.signal_dist,
                    });
                }
            },
            None => {
                closest = Some(Point {
                    x: point.x,
                    y: point.y,
                    signal_dist: point.signal_dist,
                });
            }
        }
    }

    closest.unwrap()
}

fn wire_intersects(wire1: Vec<Point>, wire2: Vec<Point>) -> Vec<Point> {
    let mut intersects: Vec<Point> = Vec::with_capacity(wire1.len());
    for p1 in &wire1 {
        for p2 in &wire2 {
            if p1 == p2 {
                println!("Overlap");
                intersects.push(Point {
                    x: p1.x,
                    y: p1.y,
                    signal_dist: p1.signal_dist + p2.signal_dist,
                });
            }
        }
    }

    intersects
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

fn get_path(steps: Vec<Step>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::with_capacity(steps.len());
    let mut head = Point {
        x: 0,
        y: 0,
        signal_dist: 0,
    };

    for s in steps {
        for _ in 0..s.dist {
            match s.dir {
                Dir::Right => head.x += 1,
                Dir::Left  => head.x -= 1,
                Dir::Up    => head.y += 1,
                Dir::Down  => head.y -= 1,
            }
            head.signal_dist += 1;
            points.push(Point {x: head.x,
                                 y: head.y,
                                 signal_dist: head.signal_dist});
        }
    }

    points
}
