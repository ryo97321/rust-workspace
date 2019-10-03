fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let input = getline();
    let s = input.trim();

    let mut line = 1;

    if s != "" {
        for c in s.chars().into_iter() {
            if c == 'L' {
                line *= 2;
            } else if c == 'R' {
                line = line * 2 + 1;
            }
        }
    }

    println!("{}", line);
}