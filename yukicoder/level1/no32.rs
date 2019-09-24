fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut number_of_thousand = 0;

    let mut number_of_hundred: i32 = getline().trim().parse().unwrap();
    let mut number_of_twentyfive: i32 = getline().trim().parse().unwrap();
    let mut number_of_one: i32 = getline().trim().parse().unwrap();

    // 1円硬貨を25円硬貨に両替
    loop {
        if number_of_one < 25 {
            break;
        } else {
            number_of_one -= 25;
            number_of_twentyfive += 1;
        }
    }

    // 25円硬貨を100円硬貨に両替
    loop {
        if number_of_twentyfive < 4 {
            break;
        } else {
            number_of_twentyfive -= 4;
            number_of_hundred += 1;
        }
    }

    // 100円硬貨を1000円札に両替
    loop {
        if number_of_hundred < 10 {
            break;
        } else {
            number_of_hundred -= 10;
            number_of_thousand += 1;
        }
    }

    let number_of_coin = number_of_one + number_of_twentyfive + number_of_hundred;
    println!("{}", number_of_coin);
}