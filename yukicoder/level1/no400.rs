fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let s = line.trim();

    for c in s.chars().into_iter().rev() {
        if c == '<' {
            print!(">");
        } else if c == '>' {
            print!("<");
        }
    }
    println!();
    
}