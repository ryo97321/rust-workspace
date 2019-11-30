fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let s = line.trim();

    let mut c_flag = false;
    let mut w1_flag = false;
    let mut w2_flag = false;
    let mut count = 0;

    let mut cww_length_list: Vec<i32> = Vec::new();

    for c in s.chars().into_iter() {
        if c_flag == false {
            if c == 'c' {
                c_flag = true;
                count = 1;
            }
        } else if c_flag == true && w1_flag == false {
            count += 1;
            if c == 'w' {
                w1_flag = true;
            } else if c == 'c' {
                w1_flag = false;
                count = 1;
            }
        } else if c_flag == true && w1_flag == true && w2_flag == false {
            count += 1;
            if c == 'w' {
                w2_flag = true;
                cww_length_list.push(count);
            } else if c == 'c' {
                w1_flag = false;
                w2_flag = false;
                count = 1;
            }
        } else if c_flag == true && w1_flag == true && w2_flag == true {
            if c == 'c' {
                w1_flag = false;
                w2_flag = false;
                count = 1;
            }
        }
    }

    let mut min_length = 101;
    for length in cww_length_list {
        if length < min_length {
            min_length = length;
        }
    }

    if min_length != 101 {
        println!("{}", min_length);
    } else {
        println!("-1");
    }

}