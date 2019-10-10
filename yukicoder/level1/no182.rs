fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut unique_number_count = 0;
    let number_of_integer: usize = getline().trim().parse().unwrap();
    
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut v: Vec<i32> = Vec::new();

    for i in 0..number_of_integer {
        let number = params[i].parse().unwrap();
        v.push(number);
    }

    // vの中でmaxを探す
    let mut max = 0;

    for i in 0..number_of_integer {
        if v[i] > max {
            max = v[i];
        }
    }

    // 要素数maxの新しいvectorを作成する
    let mut number_appear_count_vector: Vec<i32> = Vec::new();
    for _ in 0..max {
        number_appear_count_vector.push(0);
    }

    let number_appear_count_vector_length = number_appear_count_vector.len();

    // vを走査し, vector[v[i]-1] += 1
    for i in 0..number_of_integer {
        let increment_index = (v[i] - 1) as usize; 
        number_appear_count_vector[increment_index] += 1;
    }

    // vectorのなかで vector[i] == 1 を見つけるたび, unique_number_count += 1
    for i in 0..number_appear_count_vector_length {
        if number_appear_count_vector[i] == 1 {
            unique_number_count += 1;
        }
    }

    println!("{}", unique_number_count);
}