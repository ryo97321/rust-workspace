fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..n {
        let number = params[i].parse().unwrap();
        numbers.push(number);
    }

    numbers.sort();

    let mid: f64;

    if n % 2 == 0 {
        let center_left = numbers[n/2-1] as f64;
        let center_right = numbers[n/2] as f64;
        mid = (center_left + center_right) / 2.0;
    } else {
        mid = numbers[((n+1)/2)-1] as f64;
    }

    println!("{}", mid);
}