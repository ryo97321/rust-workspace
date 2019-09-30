use std::collections::HashMap;

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut level_map: HashMap<String, i32> = HashMap::new();

    for i in 0..n {
        let count = level_map.entry(params[i].to_string()).or_insert(0);
        *count += 1;
    }

    let mut level_map_vec: Vec<(&String, &i32)> = level_map.iter().collect();
    level_map_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut tuple_vec: Vec<(String, i32)> = Vec::new();
    tuple_vec.push((level_map_vec[0].0.to_string(), *level_map_vec[0].1));

    for i in 1..level_map_vec.len() {
        if tuple_vec[0].1 == *level_map_vec[i].1 {
            tuple_vec.push((level_map_vec[i].0.to_string(), *level_map_vec[i].1));
        }
    }

    let mut most_number_vec: Vec<i32> = Vec::new();
    for tuple in &tuple_vec {
        let number: i32 = tuple.0.parse().unwrap();
        most_number_vec.push(number);
    }

    most_number_vec.sort();

    let the_most_common_answer = most_number_vec[most_number_vec.len() - 1];
    println!("{}", the_most_common_answer);
}