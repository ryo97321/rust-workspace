use std::io;

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
        clear_screen();
        draw(&map, px, py);
        println!("Move with WASD (single step): ");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let c = buf.chars().next().unwrap_or('\n');

        if c == 'q' || c == 'Q' {
            break;
        }

        let (mut nx, mut ny) = (px, py);
        match c {
            'w' | 'W' => ny -= 1,
            's' | 'S' => ny += 1,
            'a' | 'A' => nx -= 1,
            'd' | 'D' => nx += 1,
            _ => {}
        }

        if !is_wall(&map, nx, ny) {
            px = nx;
            py = ny;
        }
    }

    clear_screen();
    println!("Bye!");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn is_wall(map: &[&str], x: i32, y: i32) -> bool {
    if y < 0 || y as usize >= map.len() {
        return true;
    }
    let row = map[y as usize];
    if x < 0 || x as usize >= row.chars().count() {
        return true;
    }
    row.chars().nth(x as usize).unwrap() == '#'
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

