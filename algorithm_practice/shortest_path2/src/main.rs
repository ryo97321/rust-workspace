use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write};

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut n_node: usize = 0;                      // ノードの個数
    let mut costs: Vec<Vec<i32>> = Vec::new();      // ノード間のコスト

    let mut filepath = String::from("pathfiles/");
    print!("ファイル名（ex. sample.txt）: ");
    stdout().flush().unwrap();
    let line = getline();
    let filename = line.trim();
    filepath += filename;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    // n_nodeとcostsを設定
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if i == 0 {
            let params: Vec<_> = line.trim().split(' ').collect();
            let param: usize = params[1].parse().unwrap();
            n_node = param;

            // costsの初期化
            for j in 0..n_node {
                let mut costs_element: Vec<i32> = Vec::new();
                for k in 0..n_node {
                    if j == k {
                        costs_element.push(0);
                    } else {
                        costs_element.push(-1);
                    }
                }
                costs.push(costs_element);
            }
        } else {
            let params: Vec<_> = line.trim().split(' ').collect();
            let start: usize = params[0].parse().unwrap();
            let target: usize = params[1].parse().unwrap();
            let cost: i32 = params[2].parse().unwrap();

            costs[start][target] = cost;
        }
    }

    for i in 0..n_node {
        for j in 0..n_node {
            print!("{} ", costs[i][j])
        }
        println!();
    }

    // 経路を使用したか否か
    // 例. ノードiからノードjを通ったら, route_used_check[i][j] = true
    let mut route_used_check: Vec<Vec<bool>> = Vec::new();
    for _ in 0..n_node {
        let mut route_used_check_element: Vec<bool> = Vec::new();
        for _ in 0..n_node {
            route_used_check_element.push(false);
        }
        route_used_check.push(route_used_check_element);
    }



}
