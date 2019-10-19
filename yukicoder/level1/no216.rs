fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut results = [0; 101];
    let mut scores: Vec<i32> = Vec::new();

    let n: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    for i in 0..n {
        let score: i32 = params[i].parse().unwrap();
        scores.push(score);
    }

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut k_score = 0;

    for i in 0..n {
        let result = scores[i];
        let solve_player_number: usize = params[i].parse().unwrap();

        if solve_player_number == 0 {
            k_score += result;
        } else {
            results[solve_player_number] += result;
        }
    }

    let mut is_k_max = true;
    for i in 0..results.len() {
        if results[i] > k_score {
            is_k_max = false;
            break;
        }
    }

    if is_k_max {
        println!("YES");
    } else {
        println!("NO");
    }
}