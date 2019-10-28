use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write};

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut n_node: usize = 0;                          // ノードの個数
    let mut costs: Vec<Vec<i32>> = Vec::new();          // ノード間のコストを格納

    let mut filepath = String::from("pathfiles/");
    print!("ファイル名（ex. sample.txt）: ");
    stdout().flush().unwrap();
    let line = getline();
    let filename = line.trim();
    filepath += filename;
    
    let file = File::open(filepath).unwrap();
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

    let n_point = n_node;                    // ノードの数

    print!("出発地の番号 : ");
    stdout().flush().unwrap();
    let line = getline();
    let sp: usize = line.trim().parse().unwrap();

    print!("目的地の番号 : ");
    stdout().flush().unwrap();
    let line = getline();
    let dp: usize = line.trim().parse().unwrap();

    let mut s_route: Vec<i32> = Vec::new();      // 最短経路で経由するノードを格納（逆順）
    let s_dist: i32;                          // 最短距離

    let mut p_dist: Vec<i32> = Vec::new();       // 出発地から各地点までの最短距離を設定する
    let mut p_route: Vec<i32> = Vec::new();
    let mut p_fixed: Vec<bool> = Vec::new();     // 出発地から各地点までの最短距離が確定しているかを格納する
    let mut i: usize;
    let mut j: usize;

    // Initialize sRoute, pDist, pFixed, pRoute
    for _ in 0..n_point {
        s_route.push(-1);
        p_dist.push(99999);
        p_fixed.push(false);
        p_route.push(-1);
    }

    p_dist[sp] = 0;  // start -> start : 0

    // Dijkstra
    loop {
        i = 0;
        // 未確定のノードを一つ探す
        loop {
            if i >= n_point {
                break;
            }
            if p_fixed[i] == false {
                break;
            }
            i += 1;
        }

        // 最後の地点まで最短距離が確定していればbreak
        if i == n_point {
            break;
        }

        // 最短距離がより短いノードを探す
        for j in i+1..n_point {
            if p_fixed[j] == false && (p_dist[j] < p_dist[i]) {
                i = j;
            }
        }

        let s_point = i;
        p_fixed[s_point] = true;      // 出発地からの最短距離を確定する

        // 最短経路を更新
        for j in 0..n_point {
            if costs[s_point][j] > 0 && p_fixed[j] == false {
                let new_dist = p_dist[s_point] + costs[s_point][j];
                if new_dist < p_dist[j] {
                    p_dist[j] = new_dist;
                    p_route[j] = s_point as i32;
                }
            }
        }
    }

    s_dist = p_dist[dp];
    j = 0;
    i = dp;
    loop {
        if i == sp {
            break;
        }
        s_route[j] = i as i32;
        i = p_route[i] as usize;
        j += 1;
    }
    s_route[j] = sp as i32;

    print!("最短経路 : ");
    for i in (0..j+1).rev() {
        print!("{} ", s_route[i]);
    }
    println!("\n最短距離 : {}", s_dist);
}
