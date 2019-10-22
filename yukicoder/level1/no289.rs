fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let s = line.trim();

    let mut sum: u32 = 0;
    for c in s.chars().into_iter() {
        if c.is_ascii_digit() {
            let number = c.to_digit(10).unwrap();
            sum += number;
        }
    }

    println!("{}", sum);
}