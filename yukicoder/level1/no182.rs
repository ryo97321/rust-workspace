fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut unique_number_count = 0;
    let number_of_integer: usize = getline().trim().parse().unwrap();
    
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut v: Vec<i32> = Vec::new();

    for i in 0..number_of_integer {
        let number = params[i].parse().unwrap();
        v.push(number);
    }

    if v.len() == 1 {
        unique_number_count = 1;
    } else {
        v.sort();

        let mut number_length = 1;
        let mut before_number = v[0];

        for i in 1..number_of_integer {
            let mut is_before_number_changed = false;

            if v[i] == before_number {
                number_length += 1;
            } else if v[i] != before_number {
                if number_length == 1 {
                    unique_number_count += 1;
                }
                is_before_number_changed = true;
                before_number = v[i];
                number_length = 1;
            }

            if i == number_of_integer - 1 {
                if is_before_number_changed == true {
                    unique_number_count += 1;
                }
            }
        }
    }

    println!("{}", unique_number_count);
}