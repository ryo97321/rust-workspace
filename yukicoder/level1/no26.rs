fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let m: i32 = getline().trim().parse().unwrap();

    let mut cups = [-1; 3];    // 〇印があれば1 なければ-1
    
    // 最初の〇印の位置に1を入れる
    cups[n - 1] = 1;

    for _ in 0..m {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let cup_one: usize = params[0].parse().unwrap();
        let cup_two: usize = params[1].parse().unwrap();

        let cup_temp = cups[cup_one - 1];
        cups[cup_one - 1] = cups[cup_two - 1];
        cups[cup_two - 1] = cup_temp;
    }

    let mut circle_checked_cup_place: usize = 0;
    for i in 0..cups.len() {
        if cups[i] == 1 {
            circle_checked_cup_place = i + 1;
            break;
        }
    }

    println!("{}", circle_checked_cup_place);
}