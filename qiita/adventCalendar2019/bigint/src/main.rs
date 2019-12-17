use num_bigint::BigInt;

fn main() {
    let s1 = "100000000000000000000";
    let s2 = "-100000000000000000000";

    let n1: BigInt = s1.parse().unwrap();
    let n2: BigInt = s2.parse().unwrap();

    if n1 > n2 {
        println!("n1 > n2");
    } else if n1 < n2 {
        println!("n1 < n2");
    } else if n1 == n2 {
        println!("n1 == n2");
    }
}