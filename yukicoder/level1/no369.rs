fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let mut sum_collect = 0;
    for i in 0..n {
        let number: i32 = params[i].parse().unwrap();
        sum_collect += number;
    }

    let sum_yuki: i32 = getline().trim().parse().unwrap();

    let diff = sum_collect - sum_yuki;

    println!("{}", diff);
}