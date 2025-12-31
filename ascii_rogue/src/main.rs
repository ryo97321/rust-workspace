use std::io;

fn main() {
    let map: Vec<Vec<char>> = [
        "##########",
        "#........#",
        "#..####..#",
        "#........#",
        "##########",
    ]
    .iter()
    .map(|row| row.chars().collect())
    .collect();

    let mut px: i32 = 1;
    let mut py: i32 = 1;

    loop {
        clear_screen();
        draw(&map, px, py);
        println!("WASD to move, q to quit >  ");

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

fn is_wall(map: &[Vec<char>], x: i32, y: i32) -> bool {
    if y < 0 || y as usize >= map.len() {
        return true;
    }
    let row = &map[y as usize];
    if x < 0 || x as usize >= row.len() {
        return true;
    }
    row[x as usize] == '#'
}

fn draw(map: &[Vec<char>], px: i32, py: i32) {
    for (y, row) in map.iter().enumerate() {
        let mut line = String::with_capacity(row.len());
        for (x, &ch) in row.iter().enumerate() {
            if x as i32 == px && y as i32 == py {
                line.push('@');
            } else {
                line.push(ch);
            }
        }
        println!("{line}");
    }
}

