fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();
    let f0: i64 = params[0].parse().unwrap();
    let f1: i64 = params[1].parse().unwrap();
    let n: usize = params[2].parse().unwrap();
    
    let mut v: Vec<i64> = Vec::new();
    v.push(f0);
    v.push(f1);

    let f2: i64 = f0 ^ f1;
    v.push(f2);

    let answer: i64;
    if n < 2 {
        answer = v[n];
    } else {
        answer = v[n%3];
    }

    println!("{}", answer);
}