fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let password_yesterday = line.trim();
    let mut password = String::new();

    for c in password_yesterday.chars().into_iter() {
        if c.is_ascii_lowercase() {
            let c_upper = &c.to_ascii_uppercase().to_string();
            password += c_upper;
        } else {
            let c_lower = &c.to_ascii_lowercase().to_string();
            password += c_lower;
        }
    }

    println!("{}", password);
}