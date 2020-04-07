fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    let mut n_hit = 0;
    let mut n_cookie = 1;

    loop {
        if n_cookie >= n {
            break;
        }

        n_cookie *= 2;
        n_hit += 1;
    }

    println!("{}", n_hit);
}