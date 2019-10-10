fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut number_of_empty_candy_box = 0;

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let number_of_candy_box: usize = params[0].parse().unwrap();
    let number_of_pick_candy: usize = params[1].parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let mut candy_boxes: Vec<i32> = Vec::new();

    for i in 0..number_of_candy_box {
        let number_of_candy: i32 = params[i].parse().unwrap();
        candy_boxes.push(number_of_candy);
    }


    for _ in 0..number_of_pick_candy {
        let mut min = 1000000;
        let mut min_candy_box_index: usize = 0;

        for i in 0..number_of_candy_box {
            if candy_boxes[i] == 0 {
                continue;
            }
            if candy_boxes[i] <= min {
                min = candy_boxes[i];
                min_candy_box_index = i;
            }
        }

        if candy_boxes[min_candy_box_index] - 1 >= 0 {
            candy_boxes[min_candy_box_index] -= 1;
            if candy_boxes[min_candy_box_index] == 0 {
                number_of_empty_candy_box += 1;
            }
        }
    }

    println!("{}", number_of_empty_candy_box);
}