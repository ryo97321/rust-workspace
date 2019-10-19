fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let line = getline();
    let s = line.trim();

    let line = getline();
    let t = line.trim();

    let mut s_vector: Vec<char> = Vec::new();
    let mut t_vector: Vec<char> = Vec::new();

    for c in s.chars().into_iter() {
        s_vector.push(c);
    }

    for c in t.chars().into_iter() {
        t_vector.push(c);
    }

    let mut count = 0;
    for i in 0..n {
        if s_vector[i] != t_vector[i] {
            count += 1;
        }
    }

    println!("{}", count);
}