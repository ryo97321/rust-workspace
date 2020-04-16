fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut a: Vec<i64> = Vec::new();

    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    for i in 0..n {
        let elem: i64 = params[i].parse().unwrap();
        a.push(elem);
    }

    let mut count = 0;
    let mut flag = true;

    loop {
        // judge
        for i in 0..n {
            if a[i] % 2 != 0 {
                flag = false;
                break;
            }
        }

        if flag == false {
            break;
        }

        // devide 2
        for i in 0..n {
            a[i] = a[i] / 2;
        }
        count += 1;
    }

    println!("{}", count);
}