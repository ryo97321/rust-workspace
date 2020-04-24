fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let mut v: Vec<i32> = Vec::new();

    for i in 0..n {
        let a: i32 = params[i].parse().unwrap();
        v.push(a);
    }

    v.sort();
    v.reverse();

    let mut alice_score = 0;
    let mut bob_score = 0;

    for (i, a) in v.iter().enumerate() {
        if i % 2 == 0 {
            alice_score += *a;
        } else if i % 2 == 1 {
            bob_score += *a;
        }
    }

    let score_diff = alice_score - bob_score;
    println!("{}", score_diff);
}