fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let my_hand: i32 = params[0].parse().unwrap();
    let your_hand: i32 = params[1].parse().unwrap();

    let judge = (my_hand - your_hand + 3) % 3;

    if judge == 0 {
        println!("Drew");
    } else if judge == 1 {
        println!("Lost");
    } else if judge == 2 {
        println!("Won");
    }
}