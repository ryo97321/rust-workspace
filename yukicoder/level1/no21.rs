fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    let k: i32 = getline().trim().parse().unwrap();

    // すべての数字を格納するnumbersを作成
    let mut numbers = Vec::new();
    for _ in 0.. n {
        let number: i32 = getline().trim().parse().unwrap();
        numbers.push(number);
    }

    // numbersを昇順にソート
    numbers.sort();

    let numbers_max = numbers[numbers.len()-1];
    let numbers_min = numbers[0];

    // 二次元Vectorを作成
    let mut vectorvector: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<i32> = Vec::new();     // グループ
    let mut count = 0;                        // 値を格納するごとに++

    // vectorvector[0]にnumbers_maxを入れる
    group.push(numbers_max);
    vectorvector.push(group.clone());
    count += 1;
    let numbers_len = numbers.len();
    numbers.remove((numbers_len - 1) as usize);
    group.clear();

    // vectorvector[1]にnumbers_minを入れる
    group.push(numbers_min);
    vectorvector.push(group.clone());
    count += 1;
    numbers.remove(0);
    group.clear();

    // vectorvectorにnumbersの残りを入れる
    let mut is_all_vectorvectorelement_have_one_element = false;
    for number in &numbers {
        if is_all_vectorvectorelement_have_one_element == false {
            group.push(*number);
            vectorvector.push(group.clone());
            group.clear();
            count += 1;
            if count >= k {
                is_all_vectorvectorelement_have_one_element = true;
                count = 2;
            }
        } else {
            vectorvector[count as usize].push(*number);
            count += 1;
            if count >= k {
                count = 2;
            }
        }
    }

    // 平均の差を求める
    let diff = vectorvector[0][0] - vectorvector[1][0];
    println!("{}", diff);
}