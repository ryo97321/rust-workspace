fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    let k: i32 = getline().trim().parse().unwrap();
    // println!("n:{}, k:{}", n, k);

    let mut numbers = Vec::new();
    for _ in 0.. n {
        let number: i32 = getline().trim().parse().unwrap();
        numbers.push(number);
    }
    // for i in 0..n {
    //     let ui = i as usize;
    //     println!("{}", numbers[ui]);
    // }

    let mut number_per_group: i32 = 0;     // グループごとの数字の個数
    if n % k == 0 {
        number_per_group = n / k;
    } else if (n + 1) % k == 0 {
        number_per_group = (n + 1) / k;
    }
    // println!("number_per_group : {}", number_per_group);


    // make vectorvector
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
    // for i in 0..k {
    //     for v in &vectorvector[i as usize] {
    //         println!("{}", v);
    //     }
    // }

    // make average_vector
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

    // TODO
    // 3. 最大の平均 - 最小の平均を計算し, 「平均の差」を求める

}