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

    let themes_max = themes[themes.len()-1];
    let mut answer_candidate_count = 0;

    for i in 0..n {
        let theme_plus_x = themes[i] + x;
        if theme_plus_x >= themes_max {
            answer_candidate_count += 1;
        }
    }

    println!("{}", answer_candidate_count);
}