struct Item {
    number: usize,
    value: i32,
    weight: i32,
    value_per_weight: f32,
}

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let n_item: usize = params[0].parse().unwrap();
    let n_terms: usize = params[1].parse().unwrap();
    let weight_limit: i32 = params[2].parse().unwrap();

    let mut item_count: usize = 0;
    let mut items: Vec<Item> = Vec::new();

    for _ in 0..n_item {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();

        item_count += 1;
        let item_weight: i32 = params[0].parse().unwrap();
        let item_value: i32 = params[1].parse().unwrap();
        let value_per_weight: f32 = item_value as f32 / item_weight as f32;

        let mut item = Item { number: item_count, weight: item_weight, value: item_value, value_per_weight: value_per_weight };
        items.push(item);
    }

    for _ in 0..n_terms {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let a: usize = params[0].parse().unwrap();
        let b: usize = params[1].parse().unwrap();

        item_count += 1;
        let sum_item_weight = items[a-1].weight + items[b-1].weight;
        let sum_item_value = items[a-1].value + items[b-1].value;
        let value_per_weight: f32 = sum_item_value as f32 / sum_item_weight as f32;

        let mut item = Item { number: item_count, weight: sum_item_weight, value: sum_item_value, value_per_weight: value_per_weight };
        items.push(item);
    }

    // for i in 0..item_count {
    //     println!("number:{}, weight:{}, value:{}, value_per_weight:{}", items[i].number, items[i].weight, items[i].value, items[i].value_per_weight);
    // }

    // bubble sort (asc)
    for i in 0..item_count-1 {
        for j in i+1..item_count {
            // println!("{}:{}", j-1, j);

            // TODO
            // number, weight, value, value_per_weightをそれぞれ退避して, 入れ替えする
            // ( structごと入れ替えることはできない )
            let temp: Item;
            if items[j-1].value_per_weight > items[j].value_per_weight {
                temp = items[j-1];
                items[j-1] = items[j];
                items[j] = temp;
            }
        }
    }
}