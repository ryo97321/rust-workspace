struct A;
struct B;

trait C {
    type TMP;
    fn Add(self: Self, x: Self::TMP, y: Self::TMP) -> Self::TMP;
}

impl C for A {
    type TMP = i32;
    fn Add(self: A, x: i32, y: i32) -> i32 {
        x + y
    }
}

impl C for B {
    type TMP = String;
    fn Add(self: B, x: String, y: String) -> String {
        x + &y
    }
}

fn main() {
    let a: A = A{};
    let b: B = B{};

    println!("add = {}", a.Add(10, 15));
    println!("add = {}", b.Add("hello ".to_string(), "world".to_string()));
}