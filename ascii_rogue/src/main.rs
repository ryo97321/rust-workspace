use std::io::{self, Read};

fn main() {
    let map = [
        "##########",
        "#........#",
        "#..####..#",
        "#........#",
        "##########",
    ];

    let mut px: i32 = 1;
    let mut py: i32 = 1;

    loop {
        println!("\n\n");
        draw(&map, px, py);
        println!("Move with WASD (single step): ");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let c = buf.chars().next().unwrap_or('\n');

        if c == 'q' || c == 'Q' {
            break;
        }

        match c {
            'w' | 'W' => py -= 1,
            's' | 'S' => py += 1,
            'a' | 'A' => px -= 1,
            'd' | 'D' => px += 1,
            _ => {}
        }
    }

    println!("Bye!");
}

fn draw(map: &[&str], px: i32, py: i32) {
    for (y, row) in map.iter().enumerate() {
        let mut line = String::new();
        for (x, ch) in row.chars().enumerate() {
            if x as i32 == px && y as i32 == py {
                line.push('@');
            } else {
                line.push(ch);
            }
        }
        println!("{line}");
    }
}

