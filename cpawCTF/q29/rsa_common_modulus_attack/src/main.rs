use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero, Pow};

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
    let e1_s = "13";
    let e2_s = "11";
    let s1_s = "1";
    let s2_s = "1";
    
    let e1: BigInt = e1_s.parse().unwrap();
    let e2: BigInt = e2_s.parse().unwrap();
    let mut s1: BigInt = s1_s.parse().unwrap();
    let mut s2: BigInt = s2_s.parse().unwrap();
    
    // calc s1 & s2
    extgcd(&e1, &e2, &mut s1, &mut s2);

    println!("s1:{}", s1);
    println!("s2:{}", s2);

    let c1_s = "80265690974140286785447882525076768851800986505783169077080797677035805215248640465159446426193422263912423067392651719120282968933314718780685629466284745121303594495759721471318134122366715904";
    let c2_s = "14451037575679461333658489727928902053807202350950440400755535465672646289383249206721118279217195146247129636809289035793102103248547070620691905918862697416910065303500798664102685376006097589955370023822867897020714767877873664";

    let c1: BigInt = c1_s.parse().unwrap();
    let c2: BigInt = c2_s.parse().unwrap();

    let s1_biguint: BigUint = s1.to_biguint().unwrap();
    let s2_biguint: BigUint = s2.to_biguint().unwrap();

    // it doesn't works
    let m = c1.pow(s1_biguint) + c2.pow(s2_biguint);
    println!("m : {}", m)

    // println!("e1 * s1 = {} * {} = {}", e1, s1, &e1*&s1);
    // println!("e2 * s2 = {} * {} = {}", e2, s2, &e2*&s2);
    // println!("extgcd_s1s2: {}", extgcd_s1s2);
}