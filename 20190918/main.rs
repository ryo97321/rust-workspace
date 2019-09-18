trait DuckLike {
    fn quack(&self);

    fn walk(&self) {
        println!("walking");
    }
}

struct Duck;
impl DuckLike for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

struct Tsuchinoko;
impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        println!("mew");
    }

    fn walk(&self) {
        println!("wriggling");
    }
}

impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

fn duck_go<D: DuckLike>(duck: D) {
    duck.quack();
    duck.walk();
}

fn main() {
    let duck = Duck;
    let f = 0.0;

    duck_go(duck);
    duck_go(f);
}