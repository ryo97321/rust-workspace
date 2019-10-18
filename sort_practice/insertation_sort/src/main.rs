fn insertation_sort(v: &mut Vec<i32>) {
    for i in 1..v.len() {
        let mut j = i;

        loop {
            if j <= 0 {
                break;
            }

            if v[j-1] > v[j] {
                let temp = v[j-1];
                v[j-1] = v[j];
                v[j] = temp;
            }

            j -= 1;
        }
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(1);
    v.push(5);
    v.push(2);
    v.push(99);
    println!("{:?}", v);

    insertation_sort(&mut v);
    println!("{:?}", v);
}
