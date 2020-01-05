fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    
    let mut ducks: Vec<i32> = Vec::new();

    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    for param in params {
        let duck: i32 = param.parse().unwrap();
        ducks.push(duck);
    }
}
