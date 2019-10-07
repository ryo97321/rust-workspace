fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let fossil_version = line.trim();

    let line = getline();
    let judge_version = line.trim();

    let fossil_version_numbers: Vec<_> = fossil_version.split('.').collect();
    let judge_version_numbers: Vec<_> = judge_version.split('.').collect();

    let mut is_fossil = false;

    let fossil_version_number1: i32 = fossil_version_numbers[0].parse().unwrap();
    let fossil_version_number2: i32 = fossil_version_numbers[1].parse().unwrap();
    let fossil_version_number3: i32 = fossil_version_numbers[2].parse().unwrap();

    let judge_version_number1: i32 = judge_version_numbers[0].parse().unwrap();
    let judge_version_number2: i32 = judge_version_numbers[1].parse().unwrap();
    let judge_version_number3: i32 = judge_version_numbers[2].parse().unwrap();

    let fossil_version_no_dot = fossil_version_number1 * 1000000 + fossil_version_number2 * 1000 + fossil_version_number3;
    let judge_version_no_dot = judge_version_number1 * 1000000 + judge_version_number2 * 1000 + judge_version_number3;

    if fossil_version_no_dot >= judge_version_no_dot {
        is_fossil = true;
    }
    
    if is_fossil {
        println!("YES");
    } else {
        println!("NO");
    }
}