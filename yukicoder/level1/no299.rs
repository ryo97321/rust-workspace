fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let page_num_first_edition: usize = 316;
    let n: usize = getline().trim().parse().unwrap();
    let page_num = page_num_first_edition + (n-1) * 52;

    println!("{}", page_num);
}