fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();
    let f0: i64 = params[0].parse().unwrap();
    let f1: i64 = params[1].parse().unwrap();
    let n: usize = params[2].parse().unwrap();
    
    let mut v: Vec<i64> = Vec::new();
    v.push(f0);
    v.push(f1);


    let mut f: i64;
    loop {
        if v.len() == n {
            break;
        }

        f = v[v.len()-1] ^ v[v.len()-2];
        v.push(f);
    }

    let mut v_last = v[v.len()-1];
    println!("{}", v_last);
}