use num_bigint::BigInt;
use num_traits::Zero;

fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if a < b {
        return gcd(b, a);
    }

    let zero_bigint: BigInt = Zero::zero();

    if b > &zero_bigint {
        return gcd(b, &(a % b));
    } else {
        return a.clone();
    }
}

fn main() {
    let s1 = "1200000000000000000000000000000000";
    let s2 = "240000000000000000000000000000000";

    let a: BigInt = s1.parse().unwrap();
    let b: BigInt = s2.parse().unwrap();

    let gcd_ab = gcd(&a, &b);

    println!("gcd: {}", gcd_ab);
}