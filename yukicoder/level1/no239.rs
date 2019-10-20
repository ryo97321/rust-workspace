fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let mut nyanpass_count_vector = [0; 100];

    for _ in 0..n {
        let line = getline();
        let greetings: Vec<_> = line.trim().split(' ').collect();

        for j in 0..n {
            let greeting = greetings[j];

            if greeting == "nyanpass" {
                nyanpass_count_vector[j] += 1;
            }
        }
    }

    let mut nyanpass_resident_number: i32 = -1;
    let mut nyanpass_resident_candidate_count = 0;

    for i in 0..nyanpass_count_vector.len() {
        if nyanpass_count_vector[i] == n as i32 - 1 {
            nyanpass_resident_number = i as i32;
            nyanpass_resident_candidate_count += 1;
        }
    }

    if nyanpass_resident_candidate_count == 1 && n > 2 {
        println!("{}", nyanpass_resident_number+1);
    } else {
        println!("-1");
    }
}