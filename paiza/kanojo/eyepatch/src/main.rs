fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let own_num: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let mut owns: Vec<bool> = Vec::new();

    // setting owns
    for _ in 0..n {
        owns.push(false);
    }
    for i in 0..own_num {
        let vol_number: usize = params[i].parse().unwrap();
        owns[vol_number-1] = true;
    }

    let mut shop_owns: Vec<bool> = Vec::new();

    let shop_own_num: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    // setting shop_owns
    for _ in 0..n {
        shop_owns.push(false);
    }
    for i in 0..shop_own_num {
        let vol_number: usize = params[i].parse().unwrap();
        shop_owns[vol_number-1] = true;
    }

    let mut buy_vols: Vec<usize> = Vec::new();
    
    // check volumes that I need
    for i in 0..n {
        if owns[i] == false && shop_owns[i] == true {
            buy_vols.push(i+1);
        }
    }

    // show results
    if buy_vols.len() == 0 {
        println!("None");
    } else {
        for i in 0..buy_vols.len() {
            print!("{}", buy_vols[i]);
            if i != buy_vols.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
