fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let input = getline();
    let s = input.trim();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for direction in s.chars().into_iter() {
        if direction == 'N' {
            y += 1;
        } else if direction == 'S' {
            y -= 1;
        } else if direction == 'E' {
            x += 1;
        } else if direction == 'W' {
            x -= 1;
        }
    }

    let sum_of_squares: f64 = x.pow(2) as f64 + y.pow(2) as f64;
    let distance_to_treasure: f64 = sum_of_squares.sqrt();

    println!("{}", distance_to_treasure);
}