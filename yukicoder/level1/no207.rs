fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let a: usize = params[0].parse().unwrap();
    let b: usize = params[1].parse().unwrap();

    let mut multiple_of_3_vector: Vec<usize> = Vec::new();

    for number in a..b+1 {
        let mut is_multiple_of_3 = false;
        let number_string = number.to_string();

        let mut sum = 0;
        for number_c in number_string.chars().into_iter() {

            // 各位が3か否か
            if number_c == '3' {
                is_multiple_of_3 = true;
                break;
            }

            let number_c_i32: i32 = number_c as i32 - 48;
            sum += number_c_i32;
        }

        // 各位の数字の合計値が3で割り切れるか否か
        if sum % 3 == 0 {
            is_multiple_of_3 = true;
        }

        if is_multiple_of_3 {
            multiple_of_3_vector.push(number);
        }
    }

    for i in 0..multiple_of_3_vector.len() {
        println!("{}", multiple_of_3_vector[i]);
    }
}