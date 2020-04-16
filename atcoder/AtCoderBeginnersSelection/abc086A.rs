fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();
    let a: i32 = params[0].parse().unwrap();
    let b: i32 = params[1].parse().unwrap();

    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}