use std::collections::HashSet;

struct Item {
    number: usize,
    value: i32,
    weight: i32,
    weight_per_value: f32,
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
    let n_term: usize = params[1].parse().unwrap();
    let weight_limit: i32 = params[2].parse().unwrap();

    let mut item_count: usize = 0;
    let mut items: Vec<Item> = Vec::new();

    let mut remove_item_numbers: Vec<usize> = Vec::new();

    for _ in 0..n_item {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();

        item_count += 1;
        let item_weight: i32 = params[0].parse().unwrap();
        let item_value: i32 = params[1].parse().unwrap();
        let weight_per_value: f32 = item_value as f32 / item_weight as f32;

        let mut item = Item { number: item_count, weight: item_weight, value: item_value, weight_per_value: weight_per_value };
        items.push(item);
    }

    // for _ in 0..n_term {
    //     let line = getline();
    //     let params: Vec<_> = line.trim().split(' ').collect();
    //     let a: usize = params[0].parse().unwrap();
    //     let b: usize = params[1].parse().unwrap();

    //     item_count += 1;
    //     let sum_item_weight = items[a-1].weight + items[b-1].weight;
    //     let sum_item_value = items[a-1].value + items[b-1].value;
    //     let weight_per_value: f32 = sum_item_value as f32 / sum_item_weight as f32;

    //     let mut item = Item { number: item_count, weight: sum_item_weight, value: sum_item_value, weight_per_value: weight_per_value };
    //     items.push(item);

    //     remove_item_numbers.push(a);
    //     remove_item_numbers.push(b);
    // }

    // 1 2
    // 2 3
    // 4 5
    // -> [[1,2,3], [4,5]]
    let mut term_chain_vector: Vec<Vec<usize>> = Vec::new();

    // term_chain_vectorに [[1, 2], [2, 3], [4, 5]]のように入れていく
    for i in 0..n_term {
        let mut term_item_numbers: Vec<usize> = Vec::new();
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let a: usize = params[0].parse().unwrap();
        let b: usize = params[1].parse().unwrap();

        let mut larger = a;
        let mut smaller = b;
        if a < b {
            larger = b;
            smaller = a;
        }

        term_item_numbers.push(smaller);
        term_item_numbers.push(larger);

        term_chain_vector.push(term_item_numbers);
    }

    // TODO
    // [[1,3], [3,6], [3,4]] -> [[1,3,4,6]]になるようにする
    // 以下のコードでは, [[1,3,6], [3,4]]となってしまう

    // term_chain_vectorの中身をくっつける
    let mut is_term_chain = false;
    loop {
        for i in 0..term_chain_vector.len()-1 {
            // let vector1 = term_chain_vector[i];
            let mut vector1: Vec<usize> = Vec::new();
            for j in 0..term_chain_vector[i].len() {
                vector1.push(term_chain_vector[i][j]);
            }

            // let vector2 = term_chain_vector[i+1];
            let mut vector2: Vec<usize> = Vec::new();
            for j in 0..term_chain_vector[i+1].len() {
                vector2.push(term_chain_vector[i+1][j]);
            }

            if vector1[1] == vector2[0] {
                let mut temp_vector: Vec<usize> = Vec::new();
                for j in 0..vector1.len()-1 {
                    temp_vector.push(vector1[j]);
                }
                for j in 0..vector2.len() {
                    temp_vector.push(vector2[j]);
                }

                term_chain_vector.swap_remove(i+1);
                term_chain_vector.swap_remove(i);
                term_chain_vector.push(temp_vector);
                is_term_chain = true;
            }

        }

        if is_term_chain == false {
            break;
        }
    }

    for i in 0..term_chain_vector.len() {
        item_count += 1;
        let mut sum_item_weight = 0;
        let mut sum_item_value = 0;
        for j in 0..term_chain_vector[i].len() {
            let item_number = term_chain_vector[i][j];
            sum_item_weight += items[item_number-1].weight;
            sum_item_value += items[item_number-1].value;
            remove_item_numbers.push(item_number);
        }
        let weight_per_value: f32 = sum_item_value as f32 / sum_item_weight as f32;

        let mut item = Item { number: item_count, weight: sum_item_weight, value: sum_item_value, weight_per_value: weight_per_value };
        items.push(item);
    }


    // remove duplicate element
    let remove_item_numbers_unique: HashSet<usize> = remove_item_numbers.into_iter().collect();

    for remove_item_number in &remove_item_numbers_unique {
        items.swap_remove(remove_item_number-1);
        item_count -= 1;
    }

    for i in 0..item_count {
        println!("number:{}, weight:{}, value:{}, weight_per_value:{}", items[i].number, items[i].weight, items[i].value, items[i].weight_per_value);
    }


    // items bubble sort (asc)
    loop {
        let mut is_item_change = false;

        for i in 0..item_count-1 {
            for j in i+1..item_count {
                if items[j-1].weight_per_value >  items[j].weight_per_value {
                    let temp_number = items[j-1].number;
                    let temp_weight = items[j-1].weight;
                    let temp_value = items[j-1].value;
                    let temp_weight_per_value = items[j-1].weight_per_value;

                    items[j-1].number = items[j].number;
                    items[j-1].weight = items[j].weight;
                    items[j-1].value = items[j].value;
                    items[j-1].weight_per_value = items[j].weight_per_value;

                    items[j].number = temp_number;
                    items[j].weight = temp_weight;
                    items[j].value = temp_value;
                    items[j].weight_per_value = temp_weight_per_value;
                    is_item_change = true;
                }
            }
        }

        if is_item_change == false {
            break;
        }
    }

    // knapsack
    let mut value_count = 0;
    let mut weight_remain = weight_limit;
    for i in 0..item_count {
        if weight_remain - items[i].weight >= 0 {
            value_count += items[i].value;
            weight_remain -= items[i].weight;
        }
    }

    println!("{}", value_count);
}