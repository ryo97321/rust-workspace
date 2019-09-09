fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    let k: i32 = getline().trim().parse().unwrap();
    // println!("n:{}, k:{}", n, k);

    // すべての数字を格納するnumbersを作成
    let mut numbers = Vec::new();
    for _ in 0.. n {
        let number: i32 = getline().trim().parse().unwrap();
        numbers.push(number);
    }
    // for i in 0..n {
    //     let ui = i as usize;
    //     println!("{}", numbers[ui]);
    // }

    // numbersを昇順にソート
    numbers.sort();

    // グループごとの数字の個数を求める
    let mut number_per_group: i32 = 0;     // グループごとの数字の個数
    if n % k == 0 {
        number_per_group = n / k;
    } else if (n + 1) % k == 0 {
        number_per_group = (n + 1) / k;
    }
    // println!("number_per_group : {}", number_per_group);


    // 二次元Vectorを作成
    let mut vectorvector: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<i32> = Vec::new();     // グループ
    let mut count = 0;                        // 値を格納するごとに++
    while count < n {
        for _ in 0..number_per_group {
            group.push(numbers.remove(0));
            count += 1;
            if count == n {
                break;
            }
        }
        vectorvector.push(group.clone());
        group.clear();
    }
    println!("{:?}", vectorvector);

    // 平均を格納するVectorを作成
    let mut average_vector: Vec<f64> = Vec::new();
    for i in 0..k {
        let average: f64;;
        let mut sum = 0.0;
        let mut number_of_group_element = 0;
        for v in &vectorvector[i as usize] {
            sum += *v as f64;
            number_of_group_element += 1;
        }
        average = sum / number_of_group_element as f64;
        average_vector.push(average);
    }
    println!("{:?}", average_vector);

    // 最大の平均と最小の平均を求める
    let average_vector_lastindex = average_vector.len() - 1;
    let max_average = average_vector[average_vector_lastindex];
    let min_average = average_vector[0];
    println!("max: {} min: {}", max_average, min_average);

    // 平均の差を求める
    let diff_average = max_average - min_average;
    println!("diff: {}", diff_average);
    println!("diff(ceil): {}", diff_average.ceil());

    // TODO
    // 動くが予想した値が得られないため, 修正すること

}