fn main() {
    let input_start = 271973;
    let input_end = 785961;

    let mut count = 0;
    for i in input_start..input_end {
        if is_valid(i) {
            count += 1;
        }
    }

    println!("Valid Codes: {}", count);
}

fn is_valid(mut code: i32) -> bool {
    let mut has_double = false;
    let mut last_val = 10;
    while code > 0 {
        let dig = code % 10;
        if dig == last_val {
            has_double = true;
        } else if dig > last_val {
            return false;
        }

        last_val = dig;
        code /= 10;
    }

    has_double
}
