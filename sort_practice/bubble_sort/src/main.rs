fn bubble_sort_asc(v: &mut Vec<i32>) {

    loop {
        let mut is_element_changed = false;

        for i in 0..v.len()-1 {
            for j in i+1..v.len() {
                if v[j-1] > v[j] {
                    let temp = v[j-1];
                    v[j-1] = v[j];
                    v[j] = temp;
                    is_element_changed = true;
                }
            }
        }

        if is_element_changed == false {
            break;
        }
    }

}

fn bubble_sort_desc(v: &mut Vec<i32>) {

    loop {
        let mut is_element_changed = false;

        for i in (0..v.len()-1).rev() {
            for j in (1..i+2).rev() {
                if v[j-1] < v[j] {
                    let temp = v[j-1];
                    v[j-1] = v[j];
                    v[j] = temp;
                    is_element_changed = true;
                }
            }
        }

        if is_element_changed == false {
            break;
        }
    }

}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(1);
    v.push(6);
    v.push(25);
    v.push(2);
    println!("{:?}", v);

    bubble_sort_asc(&mut v);
    println!("{:?}", v);

    bubble_sort_desc(&mut v);
    println!("{:?}", v);
}
