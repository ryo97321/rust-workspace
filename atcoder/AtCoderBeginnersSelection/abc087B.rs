fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let a: i64 = getline().trim().parse().unwrap();
    let b: i64 = getline().trim().parse().unwrap();
    let c: i64 = getline().trim().parse().unwrap();
    let x: i64 = getline().trim().parse().unwrap();

    let mut count = 0;
    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if (500*i + 100*j + 50*k) == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}