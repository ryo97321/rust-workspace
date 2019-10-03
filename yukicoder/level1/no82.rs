fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn change_color(color: &str) -> &str {
    if color == "W" {
        "B"
    } else {
        "W"
    }
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let width: i32 = params[0].trim().parse().unwrap();
    let height: i32 = params[1].trim().parse().unwrap();
    let first_color = params[2].trim();
    let mut paint_color = first_color;

    for _ in 0..height {
        for _ in 0..width {
            print!("{}", paint_color);

            paint_color = change_color(paint_color);
        }
        println!();

        if width % 2 == 0 {
            paint_color = change_color(paint_color);
        }
    }
}