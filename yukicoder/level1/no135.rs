fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn abs_diatance(distance: i32) -> i32 {
    let mut result = distance;
    if distance < 0 {
        result = distance * -1;
    }

    return result;
}

fn check_all_elements_same(points: &Vec<i32>) -> bool {
    let mut flag = true;

    for i in 0..points.len()-2 {
        if points[i] != points[i+1] {
            flag = false;
            break;
        }
    }

    return flag;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    
    let line = getline();
    let v: Vec<_> = line.trim().split(' ').collect();

    let mut points: Vec<i32> = Vec::new();

    for i in 0..n {
        let point: i32 = v[i].trim().parse().unwrap();
        points.push(point);
    }

    points.sort();

    let mut min_distance = 1000000;
    let is_all_element_same = check_all_elements_same(&points);

    if is_all_element_same {
        min_distance = 0;
    } else {
        for i in 1..n-1 {
            if points[i+1] == points[i] {
                continue;
            }

            let mut distance = points[i+1] - points[i];
            distance = abs_diatance(distance);

            if distance < min_distance {
                min_distance = distance;
            }
        }
    }

    println!("{}", min_distance);
}