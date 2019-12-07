fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let s: i32 = params[0].parse().unwrap();
    let f: i32 = params[1].parse().unwrap();

    let mut answer = 1;
    answer += (s / f) as i32;

    println!("{}", answer);
}