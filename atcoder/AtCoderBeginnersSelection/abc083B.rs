fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let N: i32 = params[0].trim().parse().unwrap();
    let A: i32 = params[1].trim().parse().unwrap();
    let B: i32 = params[2].trim().parse().unwrap();

    let mut answer = 0;

    for i in 1..N+1 {
        let mut sum = 0;
        let s = i.to_string();
        let s_vec: Vec<_> = s.split("").collect();
        for c in s_vec {
            if c == "" {
                continue;
            }
            let c_i: i32 = c.parse().unwrap();
            sum += c_i;
        }
        
        if sum >= A && sum <= B {
            answer += i;
        }
    }

    println!("{}", answer);
}