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

    let mut item_numbers: Vec<usize> = Vec::new();
    let mut item_values: Vec<i32> = Vec::new();
    let mut item_weights: Vec<i32> = Vec::new();
    let mut weight_per_values: Vec<f32> = Vec::new();

    let mut item_count: usize = 0;

    for _ in 0..n_item {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let item_weight: i32 = params[0].parse().unwrap();
        let item_value: i32 = params[1].parse().unwrap();
        let weight_per_value: f32 = item_value as f32 / item_weight as f32;

        item_weights.push(item_weight);
        item_values.push(item_value);
        weight_per_values.push(weight_per_value);

        item_count += 1;
        item_numbers.push(item_count);
    }

    for _ in 0..n_terms {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let a: usize = params[0].parse().unwrap();
        let b: usize = params[1].parse().unwrap();

        let sum_item_weight = item_weights[a-1] + item_weights[b-1];
        let sum_item_value = item_values[a-1] + item_values[b-1];
        let weight_per_value: f32 = sum_item_value as f32 / sum_item_weight as f32;

        item_weights.push(sum_item_weight);
        item_values.push(sum_item_value);
        weight_per_values.push(weight_per_value);

        item_count += 1;
        item_numbers.push(item_count);
    }

    for i in 0..item_count {
        println!("item_number:{} / item_weight:{} / item_value:{} / weight_per_value:{}", item_numbers[i], item_weights[i], item_values[i], weight_per_values[i]);
    }
}