fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let n: usize = params[0].parse().unwrap();
    let x: i32 = params[1].parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let mut themes: Vec<i32> = Vec::new();

    for i in 0..n {
        let theme: i32 = params[i].parse().unwrap();
        themes.push(theme);
    }

    themes.sort();

    for i in 0..n {
        println!("{}", themes[i]);
    }
}