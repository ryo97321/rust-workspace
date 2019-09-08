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

    // ２次元配列の例
    // let mut v1: Vec<Vec<i32>> = vec![vec![0; number_per_group as usize]; k as usize];
    // println!("{:?}", v1);

    // make group
    let mut group = Vec::new();     // グループ
    let mut count = 0;              // 値を格納するごとに++
    while count < n {
        group.push(numbers[count as usize]);
        count += 1;
    }
    // println!("{:?}", group);

    // TODO
    // 1. 二次元配列の作成
    // 2. グループごとに平均を計算
    // 3. 最大の平均 - 最小の平均を計算し, 「平均の差」を求める


}