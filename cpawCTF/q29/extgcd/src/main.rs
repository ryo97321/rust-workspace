use num_bigint::BigInt;
use num_traits::{One, Zero};

fn extgcd(e1: &BigInt, e2: &BigInt, s1: &mut BigInt, s2: &mut BigInt) -> BigInt {

    if e1 < e2 {
        return extgcd(e2, e1, s2, s1);
    }

    let zero_bigint: BigInt = Zero::zero();

    if e2 > &zero_bigint {
        let g = extgcd(e2, &(e1 % e2), s2, s1);
        let s2_new: BigInt = (e1 / e2) * s1.clone();
        *s2 -= s2_new;
        return g.clone();
    } else {
        let one_bigint: BigInt = One::one();
        let zero_bigint: BigInt = Zero::zero();
        *s1 = one_bigint;
        *s2 = zero_bigint;
        return e1.clone();
    }
}

fn main() {
    let e1_s = "34";
    let e2_s = "29";
    let s1_s = "1";
    let s2_s = "1";

    let e1: BigInt = e1_s.parse().unwrap();
    let e2: BigInt = e2_s.parse().unwrap();
    let mut s1: BigInt = s1_s.parse().unwrap();
    let mut s2: BigInt = s2_s.parse().unwrap();

    let extgcd_s1s2 = extgcd(&e1, &e2, &mut s1, &mut s2);

    println!("e1 * s1 = {} * {} = {}", e1, s1, &e1*&s1);
    println!("e2 * s2 = {} * {} = {}", e2, s2, &e2*&s2);
    println!("extgcd_s1s2: {}", extgcd_s1s2);
}