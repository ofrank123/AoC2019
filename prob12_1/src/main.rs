fn main() {
    // Just hardcoding it in because parsing is boring, and this input is small
    let step0 = vec![Planet::new( -4, -9, -3,0,0,0),
                     Planet::new(-13,-11,  0,0,0,0),
                     Planet::new(-17, -7, 15,0,0,0),
                     Planet::new(-16,  4,  2,0,0,0)];

    //println!("{:#?}", step0);
    println!("{:#?}", get_energy(&get_n_step(step0, 1000)));
}

type Step = Vec<Planet>;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct Vec3D {
    x: i32,
    y: i32,
    z: i32
}

fn get_energy(step: &Step) -> i32 {
    let mut total = 0;
    for p in step {
        let pot = p.pos.x.abs() + p.pos.y.abs() + p.pos.z.abs();
        let kin = p.vel.x.abs() + p.vel.y.abs() + p.vel.z.abs();
        total += pot * kin;
    }

    total
}

fn get_n_step(step0: Step, n: i32) -> Step {
    let mut step = step0;
    for _ in 0..n {
        step = next_step(&step);
    }
    step
}

fn next_step(step: &Step) -> Step {
    let mut next_step = vec![];
    for planet in step {
        let mut new_planet = planet.clone();
        for other in step {
            if planet.pos.x > other.pos.x {
                new_planet.pos.x -= 1;
            } else if planet.pos.x < other.pos.x {
                new_planet.pos.x += 1;
            }

            if planet.pos.y > other.pos.y {
                new_planet.pos.y -= 1;
            } else if planet.pos.y < other.pos.y {
                new_planet.pos.y += 1;
            }

            if planet.pos.z > other.pos.z {
                new_planet.pos.z -= 1;
            } else if planet.pos.z < other.pos.z {
                new_planet.pos.z += 1;
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
