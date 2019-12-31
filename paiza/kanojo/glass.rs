fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();

    let mut image: Vec<Vec<i32>> = Vec::new();

    // read image
    for _ in 0..n {
        let mut image_line: Vec<i32> = Vec::new();

        let line = getline();
        let params: Vec<_> = line.trim().split(" ").collect();
        for j in 0..n {
            let pixel: i32 = params[j].parse().unwrap();
            image_line.push(pixel);
        }
        image.push(image_line);
    }


    let m: usize = getline().trim().parse().unwrap();

    let mut pattern: Vec<Vec<i32>> = Vec::new();

    // read pattern
    for _ in 0..m {
        let mut pattern_line: Vec<i32> = Vec::new();

        let line = getline();
        let params: Vec<_> = line.trim().split(" ").collect();
        for j in 0..m {
            let pixel: i32 = params[j].parse().unwrap();
            pattern_line.push(pixel);
        }
        pattern.push(pattern_line);
    }

    // pattern matching

    let mut is_match = false;
    let mut match_count: usize = 0;

    let mut x_match: usize = 0;
    let mut y_match: usize = 0;

    for x_s in 0..(n-m+1) {
        for y_s in 0..(n-m+1) {
            for x in x_s..(x_s+m) {
                for y in y_s..(y_s+m) {
                    if image[x][y] == pattern[x-x_s][y-y_s] {
                        match_count += 1;
                    }
                }
            }

            if match_count == m * m {
                is_match = true;
                x_match = x_s;
                y_match = y_s;
                break;
            }
            match_count = 0;
        }

        if is_match {
            break;
        }
    }

    if is_match {
        println!("{} {}", x_match, y_match);
    }
}

