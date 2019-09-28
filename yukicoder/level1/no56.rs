fn getline() -> String {
    let mut __ret == String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').correct();
    let d: f64 = params[0].parse().unwrap();
}