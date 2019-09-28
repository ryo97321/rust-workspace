fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let d: f64 = params[0].parse().unwrap();
    let p: f64 = params[1].parse().unwrap();

    let result = ((d * (p / 100.0)) + d) as i32;

    println!("{}", result);
}