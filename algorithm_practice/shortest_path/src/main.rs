use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut n_node: usize = 0;                          // ノードの個数
    let mut costs: Vec<Vec<i32>> = Vec::new();          // ノード間のコストを格納
    
    let filename = "pathfiles/node5.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read File
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if i == 0 {
            let params: Vec<_> = line.trim().split(' ').collect();
            let param: usize = params[1].parse().unwrap();
            n_node = param;

            // Initialize costs
            for i in 0..n_node {
                let mut costs_element: Vec<i32> = Vec::new();
                for j in 0..n_node {
                    if i == j {
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

            // Set costs
            costs[start][target] = cost;
        }
    }

    // TODO
    // 最短経路算出のロジックを書く（ダイクストラ法）
}
