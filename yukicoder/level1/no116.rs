fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let number_of_bamboo: usize = getline().trim().parse().unwrap();

    let input = getline();
    let kadomatsu_lengths: Vec<_> = input.trim().split(' ').collect();

    let mut number_of_kadomatsu_line = 0;

    for i in 0..number_of_bamboo {
        if i + 2 > number_of_bamboo - 1 {
            break;
        }

        let left: i32 = kadomatsu_lengths[i].trim().parse().unwrap();
        let center: i32 = kadomatsu_lengths[i+1].trim().parse().unwrap();
        let right: i32 = kadomatsu_lengths[i+2].trim().parse().unwrap();

        if (left > center && left < right) || (left > right && left < center) || (right > center && right < left) || (right > left && right < center) {
            number_of_kadomatsu_line += 1;
        }
    }

    println!("{}", number_of_kadomatsu_line);
}