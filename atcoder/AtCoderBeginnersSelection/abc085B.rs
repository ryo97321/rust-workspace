use std::collections::HashSet;

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let mut v = HashSet::new();
    for _ in 0..n {
        let d: i32 = getline().trim().parse().unwrap();
        v.insert(d);
    }

    println!("{}", v.len());
}