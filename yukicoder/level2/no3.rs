fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

// Stringを逆にする（ex. test -> tset）
fn reverse_str(input: &String) -> String {
    let mut reversed = String::new();
    let mut chars: Vec<char> = Vec::new();

    for c in input.chars().into_iter() {
        chars.push(c);
    }

    for i in (0..chars.len()).rev() {
        reversed += &chars[i].to_string();
    }

    return reversed;
}

// 10進数を2進数にして返す（ex. 10 -> 1010）
fn dec_to_bin_str(number: i32) -> String {
    let mut number = number;
    let mut result = String::new();

    if number == 1 {
        result = "1".to_string();
    } else {
        loop {
            let rem = number % 2;
            result += &rem.to_string();

            number /= 2;

            if number == 1 {
                result += &number.to_string();
                break;
            }
        }
    }

    result = reverse_str(&result);
    return result;
}

// 移動量を計算する
fn calc_move(bin: &String) -> i32 {
    let mut sum_one = 0;
    for c in bin.chars().into_iter() {
        if c == '1' {
            sum_one += 1;
        }
    }
    return sum_one;
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();

    let mut position = 0;       // 今いる場所
    let goal = n;               // ゴール

    let mut total = 0;          // ゴールにつくまでに何手必要か

    position += 1;
    total += 1;

    loop {
        let d_position = calc_move(&dec_to_bin_str(position));      // 移動量
        if position + d_position > goal {
            position -= d_position;
        } else {
            position += d_position;
        }
        total += 1;
        if position == goal {
            break;
        }
    }

    println!("{}", total);
}