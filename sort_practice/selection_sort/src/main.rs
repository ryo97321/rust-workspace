fn selection_sort(v: &mut Vec<i32>) {
    for i in 0..v.len()-1 {
        let mut min_index = i;

        for j in i+1..v.len() {
            if v[min_index] > v[j] {
                min_index = j;
            }
        }

        let temp = v[i];
        v[i] = v[min_index];
        v[min_index] = temp;
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(1);
    v.push(5);
    v.push(99);
    v.push(11);

    println!("{:?}", v);

    selection_sort(&mut v);
    println!("{:?}", v);
}
