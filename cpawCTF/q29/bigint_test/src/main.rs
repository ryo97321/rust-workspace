use num_bigint::BigInt;

fn main() {
    let s = "10000000000000000";
    let s2 = "9999999999999998";

    let n: BigInt = s.parse().unwrap();
    let n2: BigInt = s2.parse().unwrap();
    let mut n3: BigInt;
    n3 = n2.clone();

    println!("n = {}", n);
    println!("n2 = {}", n2);
    println!("n3 = {}", n3);

    // Copyトレイトが実装されていないため, &nのように参照を使用
    let add_result = &n + &n2;
    let diff_result = &n - &n2;
    let mul_result = &n * &n2;
    let div_result = &n / &n2;
    let remain_result = &n % &n2;

    if n > n2 {
        println!("n > n2");
    } else if n < n2 {
        println!("n < n2");
    } else if n == n2 {
        println!("n == n2");
    }
    
    println!("{} + {} = {}", n, n2, add_result);
    println!("{} - {} = {}", n, n2, diff_result);
    println!("{} * {} = {}", n, n2, mul_result);
    println!("{} / {} = {}", n, n2, div_result);
    println!("{} % {} = {}", n, n2, remain_result);
}