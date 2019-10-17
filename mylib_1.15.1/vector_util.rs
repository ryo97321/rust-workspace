#![crate_type  = "lib"]
#![crate_name = "vector_util"]

pub fn max<T: PartialOrd + Copy>(v: &Vec<T>) -> T {
    let mut max = v[0];

    for i in 1..v.len() {
        let num = v[i];
        if num > max {
            max = num;
        }
    }

    return max;
}