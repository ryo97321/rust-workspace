fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let n: i32 = params[0].parse().unwrap();
    let p: i32 = params[1].parse().unwrap();

    let np = n * p;

    if p == np {
        println!("=");
    } else if p != np {
        println!("!=");
    }
}