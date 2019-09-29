fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let a = line.trim();
    let line = getline();
    let b = line.trim();

    let mut a_chars: Vec<_> = a.chars().collect();
    a_chars.sort();
    let a_sorted: String = a_chars.into_iter().collect();

    let mut b_chars: Vec<_> = b.chars().collect();
    b_chars.sort();
    let b_sorted: String = b_chars.into_iter().collect();

    if a_sorted == b_sorted {
        println!("YES");
    } else {
        println!("NO");
    }
}