fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n_day: usize = 14;
    let mut days: Vec<char> = Vec::new();

    let line = getline();
    let week1 = line.trim();
    for day in week1.chars().into_iter() {
        days.push(day);
    }

    let line = getline();
    let week2 = line.trim();
    for day in week2.chars().into_iter() {
        days.push(day);
    }

    let mut answer_vector: Vec<i32> = Vec::new();
    let mut holiday_count = 0;
    for i in 0..n_day {
        if days[i] == 'o' {
            holiday_count += 1;
        } else if days[i] == 'x' {
            answer_vector.push(holiday_count);
            holiday_count = 0;
        }
    }

    if holiday_count > 0 {
        answer_vector.push(holiday_count);
    }

    answer_vector.sort();

    let answer = answer_vector[answer_vector.len()-1];

    println!("{}", answer);
}