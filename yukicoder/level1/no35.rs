fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let ab: Vec<_> = line.trim().split(' ').collect();
    let a: i32 = ab[0].parse().unwrap();
    let b: i32 = ab[1].parse().unwrap();

    let answer: i32;

    if b % a == 0 {
        answer = b / a;
    } else {
        answer = (b / a) + 1;
    }

    println!("{}", answer);
}