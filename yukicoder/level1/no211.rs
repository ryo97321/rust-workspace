fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let dice_prime_number = vec![2, 3, 5, 7, 11, 13];
    let dice_number_of_composites = vec![4, 6, 8, 9, 10, 12];

    let mut all_product_pattern: Vec<i32> = Vec::new();

    for i in 0..dice_prime_number.len() {
        for j in 0..dice_number_of_composites.len() {
            let product = dice_prime_number[i] * dice_number_of_composites[j];
            all_product_pattern.push(product);
        }
    }

    let k: i32 = getline().trim().parse().unwrap();

    let mut count = 0;

    for i in 0..all_product_pattern.len() {
        if all_product_pattern[i] == k {
            count += 1;
        }
    }

    let probability = count as f64 / 36.0;

    println!("{}", probability);

}