fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut items = [0; 11];
    let mut max_powerup_count = 0;

    let n: i32 = getline().trim().parse().unwrap();

    for _ in 0..n {
        let line = getline();
        let drop_items: Vec<_> = line.trim().split(' ').collect();
        let drop_item_one: usize = drop_items[0].parse().unwrap();
        let drop_item_two: usize = drop_items[1].parse().unwrap();
        let drop_item_three: usize = drop_items[2].parse().unwrap();

        items[drop_item_one] += 1;
        items[drop_item_two] += 1;
        items[drop_item_three] += 1;
    }

    // 同じアイテム２つでパワーアップできるものを探す
    for i in 1..items.len() {
        loop {
            if items[i] < 2 {
                break;
            }
            items[i] -= 2;
            max_powerup_count += 1;
        }
    }

    // 任意のアイテム４つでパワーアップできるものを探す
    let mut item_select_count = 0;
    loop {
        for i in 1..items.len() {
            if items[i] > 0 {
                item_select_count += 1;
                items[i] -= 1;
            }
            if item_select_count >= 4 {
                break;
            }
        }

        if item_select_count < 4 {
            break;
        } else {
            max_powerup_count += 1;
            item_select_count = 0;
        }
    }

    // 結果を表示
    println!("{}", max_powerup_count);
}