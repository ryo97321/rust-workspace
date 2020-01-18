enum TYPE {
    TYPE_A(i32, u32, usize),
    TYPE_B{a: char, b: i32},
    TYPE_C,
}

fn main() {
    let t: TYPE = TYPE::TYPE_B{ a:'b', b:10 };

    match t {
        TYPE::TYPE_A(a, b, c) => { println!("{}, {}, {}", a, b, c); },
        TYPE::TYPE_B{a, b} => { println!("{}, {}", a, b); },
        TYPE::TYPE_C => { println!("TYPE_C"); },
    };
}