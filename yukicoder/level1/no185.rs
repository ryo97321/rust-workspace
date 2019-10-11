fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut answer_vector: Vec<i32> = Vec::new();

    let n: usize = getline().trim().parse().unwrap();

    for _ in 0..n {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        
        let x: i32 = params[0].parse().unwrap();
        let y: i32 = params[1].parse().unwrap();

        let answer = y - x;
        answer_vector.push(answer);
    }

    let mut is_answer_exist = true;
    let mut before_number = 0;

    for i in 0..n {
        if i == 0 {
            before_number = answer_vector[0];
        }

        if answer_vector[i] != before_number || answer_vector[i] <= 0 {
            is_answer_exist = false;
            break;
        }

        before_number = answer_vector[i];
    }

    let answer: i32;

    if is_answer_exist {
        answer = answer_vector[0];
    } else {
        answer = -1;
    }

    println!("{}", answer);
}