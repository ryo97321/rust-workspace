fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let budget: i64 = getline().trim().parse().unwrap();
    let number_of_child: i64 = getline().trim().parse().unwrap();

    let otoshidama_unit: i64 = 1000;
    let otoshidama_per_person: i64;

    if budget / number_of_child < otoshidama_unit {
        otoshidama_per_person = 0;
    } else {
        if budget % number_of_child == 0 {
            otoshidama_per_person = budget / number_of_child;
        } else {
            let number_of_thousand_yen = (budget / number_of_child) / 1000;
            otoshidama_per_person = number_of_thousand_yen * otoshidama_unit;
        }
    }

    println!("{}", otoshidama_per_person);
}