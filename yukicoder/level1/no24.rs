fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut numbers = [0; 10];
    let n: i32 = getline().trim().parse().unwrap();

    for _ in 0..n {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let is_number_exist = params[params.len() - 1];
        if is_number_exist == "YES" {
            for j in 0..params.len() - 1 {
                let number: usize = params[j].parse().unwrap();
                numbers[number] += 1;
            }
        } else if is_number_exist == "NO" {
            for j in 0..params.len() - 1 {
                let number: usize = params[j].parse().unwrap();
                numbers[number] -= 100;
            }
        }
    }

    let mut max_numbers = numbers[0];
    let mut max_numbers_index: usize  = 0;

    for i in 1..numbers.len() {
        if numbers[i] > max_numbers {
            max_numbers = numbers[i];
            max_numbers_index = i;
        }
    }

    println!("{}", max_numbers_index);
}