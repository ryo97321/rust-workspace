fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();

    if n == 0 {
        println!("1");
    } else if n == 1 {
        println!("0");
    }
}