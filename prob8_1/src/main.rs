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

    println!("Value: {}", calc_val(layers));
}

fn calc_val(layers: Vec<Layer>) -> i32 {
    let mut min_zeros_layer = (0, 10000);
    for (i, l) in layers.iter().enumerate() {
        let zeros = digits_in_layer(l, 0);
        println!("Zeros: {}", zeros);
        if zeros < min_zeros_layer.1 {
            min_zeros_layer.0 = i;
            min_zeros_layer.1 = zeros;
        }
    }

    println!("Layer: {}", min_zeros_layer.0);
    for a in &layers[min_zeros_layer.0] {
        println!("{:?}", a);
    }
    digits_in_layer(&layers[min_zeros_layer.0], 1) * digits_in_layer(&layers[min_zeros_layer.0], 2)
}

fn digits_in_layer(layer: &Layer, digit: i32) -> i32 {
    let mut digits = 0;
    for r in layer {
        for d in r {
            if *d == digit {
                digits += 1;
            }
        }
    }
    digits
}
