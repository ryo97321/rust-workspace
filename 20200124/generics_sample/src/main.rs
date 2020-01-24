struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

fn main() {
    let mut int_origin = Point { x: 10, y: 20 };
    let mut float_origin = Point { x: 10.0, y: 20.0 };

    println!("int_origin");
    println!("x: {}, y: {}", int_origin.x, int_origin.y);
    println!("float_origin");
    println!("x: {}, y: {}", float_origin.x, float_origin.y);

    int_origin.swap();
    float_origin.swap();

    println!();

    println!("int_origin");
    println!("x: {}, y: {}", int_origin.x, int_origin.y);
    println!("float_origin");
    println!("x: {}, y: {}", float_origin.x, float_origin.y);
}
