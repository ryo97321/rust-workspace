fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let a: i32 = getline().trim().parse().unwrap();
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();
    let b: i32 = params[0].parse().unwrap();
    let c: i32 = params[1].parse().unwrap();

    let line = getline();
    let s = line.trim();

    println!("{} {}", (a+b+c), s);
}