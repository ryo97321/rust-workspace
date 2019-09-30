use std::collections::HashMap;

fn main() {
    let mut v = vec![100, 32, 44, 51, 97, 51, 66, 32, 91];
    println!("vector: {:?}", v);

    let mut sum = 0;
    for number in &v {
        sum += number;
    }

    let mean = sum as f64 / v.len() as f64;
    println!("mean: {}", mean);

    v.sort();
    let median = &v[(v.len() as usize / 2) as usize];
    println!("median: {}", median);

    let mut number_map = HashMap::new();
    for number in &v {
        let count = number_map.entry(number.to_string()).or_insert(0);
        *count += 1;
    }
    let mut number_map_vec: Vec<(&String, &i32)> = number_map.iter().collect();
    number_map_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mode: i32 = number_map_vec[0].0.parse().unwrap();
    println!("mode: {}", mode);
}