fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut w: f64 = getline().trim().parse().unwrap();
    let mut d: f64 = getline().trim().parse().unwrap();

    let mut workload: f64;

    loop {
        workload = w / (d * d);
        workload = workload.trunc();

        if workload >= w {
            break;
        }

        w -= workload;
        d -= 1.0;
    }

    println!("{}", workload);
}