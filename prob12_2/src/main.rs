fn main() {
    // Just hardcoding it in because parsing is boring, and this input is small
    let step0 = vec![Planet::new( -4, -9, -3,0,0,0),
                     Planet::new(-13,-11,  0,0,0,0),
                     Planet::new(-17, -7, 15,0,0,0),
                     Planet::new(-16,  4,  2,0,0,0)];

    let mut c: i64 = 0;
    let mut step = step0.clone();
    let mut done = false;
    while !done {
        c += 1;
        step = next_step(&step);
        done = true;
        for (i, p) in step.iter().enumerate() {
            if (*p).pos.x != step0[i].pos.x || (*p).vel.x != step0[i].vel.x {
                done = false;
                break;
            }
        }
    }

    let x = c;
    println!("X: {}", c);

    c = 0;
    step = step0.clone();
    done = false;
    while !done {
        c += 1;
        step = next_step(&step);
        done = true;
        for (i, p) in step.iter().enumerate() {
            if (*p).pos.y != step0[i].pos.y || (*p).vel.y != step0[i].vel.y {
                done = false;
                break;
            }
        }
    }

    let y = c;
    println!("Y: {}", c);

    c = 0;
    step = step0.clone();
    done = false;
    while !done {
        c += 1;
        step = next_step(&step);
        done = true;
        for (i, p) in step.iter().enumerate() {
            if (*p).pos.z != step0[i].pos.z || (*p).vel.z != step0[i].vel.z {
                done = false;
                break;
            }
        }
    }

    let z = c;
    println!("Z: {}", c);

    println!("LCM: {}", lcm(lcm(x as i64, y as i64), z as i64));

}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0{
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

type Step = Vec<Planet>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Planet {
    pos: Vec3D,
    vel: Vec3D,
}

impl Planet {
    fn new(px:i32,py:i32,pz:i32,vx:i32,vy:i32,vz:i32) -> Planet{
        Planet {
            pos: Vec3D {
                x: px,
                y: py,
                z: pz,
            },
            vel: Vec3D {
                x: vx,
                y: vy,
                z: vz,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vec3D {
    x: i32,
    y: i32,
    z: i32,
}

fn next_step(step: &Step) -> Step {
    let mut next_step = vec![];
    for planet in step {
        let mut new_planet = Planet::new(planet.pos.x, planet.pos.y, planet.pos.z, 0,0,0);
        for other in step {
            if other.pos.x > planet.pos.x {
                new_planet.pos.x += 1;
            } else if other.pos.x != planet.pos.x {
                new_planet.pos.x -= 1;
            }

            if other.pos.y > planet.pos.y {
                new_planet.pos.y += 1;
            } else if other.pos.y != planet.pos.y {
                new_planet.pos.y -= 1;
            }

            if other.pos.z > planet.pos.z {
                new_planet.pos.z += 1;
            } else if other.pos.z != planet.pos.z {
                new_planet.pos.z -= 1;
            }
        }
        new_planet.pos.x += planet.vel.x;
        new_planet.pos.y += planet.vel.y;
        new_planet.pos.z += planet.vel.z;

        new_planet.vel.x = new_planet.pos.x - planet.pos.x;
        new_planet.vel.y = new_planet.pos.y - planet.pos.y;
        new_planet.vel.z = new_planet.pos.z - planet.pos.z;

        next_step.push(new_planet);
    }
    next_step
}
