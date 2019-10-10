fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let now_percent: f64 = getline().trim().parse().unwrap();
    let remain_minute: f64 = getline().trim().parse().unwrap();

    let copy_percent_per_minute: f64 = remain_minute / (100.0 - now_percent);
    let full_copy_time: f64 = copy_percent_per_minute * 100.0;

    println!("{}", full_copy_time as i32);
}