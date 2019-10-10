fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut number_of_correct_type = 0;
    let mut number_of_miss_type = 0;

    let number_of_game: i32 = getline().trim().parse().unwrap();

    for _ in 0..number_of_game {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();

        let time_limit: f64 = params[0].parse().unwrap();
        let word = params[1];
        let word_length = word.len();

        let can_type_max_word_length = (12.0 * time_limit / 1000.0) as usize;

        if can_type_max_word_length >= word_length {
            number_of_correct_type += word_length;
        } else {
            number_of_correct_type += can_type_max_word_length;
            number_of_miss_type += word_length - can_type_max_word_length;
        }
    }

    println!("{} {}", number_of_correct_type, number_of_miss_type);
}