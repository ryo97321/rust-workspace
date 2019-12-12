fn main() {
    let a: i64 = 32134;
    let b: i64 = 1584891;
    let c: i64 = 193127;
    let d: i64 = 3438478;

    let mut n: i64 = 0;
    loop {
        n += 1;
        let x = b * n + a;
        if x % d == c {
            println!("{}", n);
            break;
        }
    }
}
