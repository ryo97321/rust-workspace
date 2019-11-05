use rand::Rng;

// 盤上のクイーンの位置が, n_queenを満たしているかどうかを返す
fn is_n_queen(board: &Vec<Vec<char>>) -> bool {
    let board_length = board[0].len();
    let mut queen_count: usize = 0;     // 設置されているクイーンの数

    // クイーンの数を取得
    for i in 0..board_length {
        for j in 0..board_length {
            if board[i][j] == 'x' {
                queen_count += 1;
            }
        }
    }

    // 盤上のクイーンの数が1ならば, 必ずn_queenを満たすのでtrueを返す
    if queen_count == 1 {
        return true;
    }

    // 設置された全てのクイーンのx座標, y座標を取得
    let mut queen_xs: Vec<usize> = Vec::new();
    let mut queen_ys: Vec<usize> = Vec::new();
    for i in 0..board_length {
        for j in 0..board_length {
            if board[i][j] == 'x' {
                queen_xs.push(i);
                queen_ys.push(j);
            }
        }
    }

    // クイーンがほかのクイーンとぶつかるかどうか確認する
    let mut is_collision = false;
    for i in 0..queen_count {
        let mut x1 = queen_xs[i] as i32;
        let mut y1 = queen_ys[i] as i32;

        for j in 0..queen_count {
            if i == j {
                continue;
            }

            let x2 = queen_xs[j] as i32;
            let y2 = queen_ys[j] as i32;

            // 上を探索
            loop {
                x1 -= 1;
                if x1 < 0 {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 下を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                x1 += 1;
                if x1 >= (board_length as i32) {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 右を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                y1 += 1;
                if y1 >= (board_length as i32) {
                    break;
                } else {
                    if x1 == y1 && x2 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 左を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                y1 -= 1;
                if y1 < 0 {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 右上を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                x1 -= 1;
                y1 += 1;
                if x1 < 0 || y1 >= (board_length as i32) {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 右下を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                x1 += 1;
                y1 += 1;
                if x1 >= (board_length as i32) || y1 >= (board_length as i32) {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 左上を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                x1 -= 1;
                y1 -= 1;
                if x1 < 0 || y1 < 0 {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }

            // 左下を探索
            x1 = queen_xs[i] as i32;
            y1 = queen_ys[i] as i32;

            loop {
                x1 += 1;
                y1 -= 1;
                if x1 >= (board_length as i32) || y1 < 0 {
                    break;
                } else {
                    if x1 == x2 && y1 == y2 {
                        is_collision = true;
                        break;
                    }
                }
            }

            if is_collision {
                break;
            }
        }

        if is_collision {
            break;
        }
    }

    // クイーンがほかのクイーンと衝突したら, n_queenを満たさないので
    if is_collision {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let mut board: Vec<Vec<char>> = Vec::new();     // 盤
    let board_length: usize = 7;                    // 盤の長さ

    // boardを初期化
    for _ in 0..board_length {
        let mut board_line = vec!['.', '.', '.', '.', '.', '.', '.'];
        board.push(board_line);
    }

    let place_queen_num: usize = 7;    // 設置するクイーンの数
    let mut placed_queen_num = 0;      // すでに設置したクイーンの数

    let mut placed_queen_xs: Vec<usize> = Vec::new();       // 設置したクイーンのx座標のvector
    let mut placed_queen_ys: Vec<usize> = Vec::new();       // 設置したクイーンのy座標のvector

    let max_try_num = 5;        // 最大試行回数
    let mut try_num = 0;        // クイーンの配置を試行した回数（max_try_numまで達したら, ひとつ前に配置したクイーンを取り除く）

    // 指定した数のクイーンを設置する
    loop {
        try_num += 1;

        let mut rng = rand::thread_rng();
        let mut rand_num: f64 = rng.gen();
        rand_num *= 7.0;
        let mut queen_x: usize = rand_num as usize;     // 0 <= queen_x <= 6
        rand_num = rng.gen();
        rand_num *= 7.0;
        let mut queen_y: usize = rand_num as usize;     // 0 <= queen_y <= 6

        placed_queen_xs.push(queen_x);
        placed_queen_ys.push(queen_y);

        // すでに置いた位置ならcontinue
        if placed_queen_xs.len() >= 2 && placed_queen_ys.len() >= 2 {
            let mut is_place_duplicate = false;
            for i in 0..placed_queen_xs.len()-1 {
                if queen_x == placed_queen_xs[i] && queen_y == placed_queen_ys[i] {
                    is_place_duplicate = true;
                    break;
                }
            }
            if is_place_duplicate {
                placed_queen_xs.pop();
                placed_queen_ys.pop();
                try_num = 0;
                continue;
            }
        }

        // クイーンを設置
        board[queen_x][queen_y] = 'x';
        placed_queen_num += 1;

        // n_queenを満たしていなかったら
        if is_n_queen(&board) == false {
            board[queen_x][queen_y] = '.';
            placed_queen_num -= 1;
            placed_queen_xs.pop();
            placed_queen_ys.pop();
        } else {
            try_num = 0;
        }


        // 試行回数がmax_try_numに達したらひとつ前に配置したクイーンを削除する
        if try_num == max_try_num {
            let remove_queen_x = placed_queen_xs.pop().unwrap();
            let remove_queen_y = placed_queen_ys.pop().unwrap();
            board[remove_queen_x][remove_queen_y] = '.';
            placed_queen_num -= 1;
            try_num = 0;
        }

        // 途中経過を表示
        println!();
        for i in 0..board_length {
            for j in 0..board_length {
                print!("{}", board[i][j]);
            }
            println!();
        }
        println!();

        // 指定した数配置したら
        if placed_queen_num == place_queen_num as i32 {
            break;
        }
    }

    // 結果を表示
    for i in 0..board_length {
        for j in 0..board_length {
            print!("{}", board[i][j]);
        }
        println!();
    }
}