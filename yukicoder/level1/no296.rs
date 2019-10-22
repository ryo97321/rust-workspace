fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let nidone_count: usize = params[0].parse().unwrap();
    let h: usize = params[1].parse().unwrap();
    let m: usize = params[2].parse().unwrap();
    let alerm_interval: usize = params[3].parse().unwrap();

    let add_minute: usize = (nidone_count-1) * alerm_interval;

    let mut h_wakeup;
    let mut m_wakeup = m + add_minute;
    if m_wakeup >= 60 {
        let add_hour = m_wakeup / 60;
        m_wakeup = m_wakeup % 60;
        h_wakeup = h + add_hour;
    } else {
        h_wakeup = h;
    }

    if h_wakeup >= 24 {
        h_wakeup = h_wakeup % 24;
    }

    println!("{}", h_wakeup);
    println!("{}", m_wakeup);
}