fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let s = line.trim();

    let mut count = 0;
    for i in 0..3 {
        if &s[i..i+1] == "1" {
            count += 1;
        }
    }

    println!("{}", count);
}