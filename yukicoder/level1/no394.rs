fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let mut scores: Vec<i32> = Vec::new();

    for param in params {
        let score: i32 = param.parse().unwrap();
        scores.push(score);
    }

    // sort asc
    scores.sort();

    let mut sum_score = 0;
    for i in 1..scores.len()-1 {
        let score = scores[i];
        sum_score += score;
    }

    let average_score: f64 = (sum_score as f64) / 4.0;

    println!("{:.2}", average_score);
}