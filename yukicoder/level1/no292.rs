fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let s = params[0];
    let t: usize = params[1].parse().unwrap();
    let u: usize = params[2].parse().unwrap();

    let mut s_chars: Vec<char> = Vec::new();
    for c in s.chars().into_iter() {
        s_chars.push(c);
    }

    let mut s_new = String::new();

    if t == u {
        let erase_index = t;
        for i in 0..s_chars.len() {
            if i == erase_index {
                continue;
            }
            s_new += &s_chars[i].to_string();
        }
    } else {
        for i in 0..s_chars.len() {
            if i == t || i == u {
                continue;
            }
            s_new += &s_chars[i].to_string();
        }
    }

    println!("{}", s_new);
}