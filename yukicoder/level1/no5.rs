fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let l: i32 = getline().trim().parse().unwrap();
    let n: i32 = getline().trim().parse().unwrap();
    let line = getline();
    let blockwidths_string: Vec<_> = line.trim().split(' ').collect();

    let mut blockwidths = Vec::new();
    for i in 0..n {
        let ui = i as usize;
        blockwidths.push(blockwidths_string[ui].parse::<i32>().unwrap());
    }

    blockwidths.sort();

    let mut block_count = 0;
    let mut blockwidth_total = 0;

    for blockwidth in &blockwidths {
        if blockwidth + blockwidth_total <= l {
            block_count += 1;
            blockwidth_total += blockwidth;
        }
    }

    println!("{}", block_count);
}