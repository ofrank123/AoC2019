use std::fs;

const W: i32 = 25;
const H: i32 = 6;

type Layer = Vec<Vec<i32>>;

fn main() {
    let layer_size = W * H;

    let input = fs::read_to_string("./input")
        .expect("Could not read file");
    let input: Vec<char> = input.chars().collect();
    let input: Vec<i32> = input.iter().map(|d| *d as i32 - '0' as i32).collect();

    let mut layers: Vec<Layer> = vec![];
    let mut layer: Layer = vec![];
    let mut row: Vec<i32> = vec![];
    for (i, input) in input.iter().enumerate() {
        if i as i32 % W == 0 && i != 0 {
            layer.push(row.clone());
            row.drain(..);
        }
        if i as i32 % layer_size == 0 && i != 0{
            layers.push(layer.clone());
            layer.drain(..);
        }

        row.push(*input);
    }
    layer.push(row.clone());
    layers.push(layer.clone());

    /*
    for (i, l) in layers.iter().enumerate() {
        println!("Layer: {}", i);
        for a in l {
            println!("{:?}", a);
        }
    }
     */
    for a in decode(&layers){
        for d in a {
            if d == 0 {
                print!(" ")
            }
            if d == 1 {
                print!("█")
            }
        }
        println!()
    }
}

fn decode(layers: &Vec<Layer>) -> Layer {
    let mut image: Layer = layers[0].clone();
    for l in layers {
        for (y, r) in l.iter().enumerate() {
            for (x, d) in r.iter().enumerate() {
                if image[y][x] == 2 {
                    image[y][x] = *d;
                }
            }
        }
    }

    image
}
