use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap
    },
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut out = stdout();
    queue!(out, Hide, DisableLineWrap)?;
    out.flush()?;

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

    let mut ex: i32 = 5;
    let mut ey: i32 = 1;

    let mut hp: i32 = 10;
    let mut message = String::from("WASD to move, q to quit");

    let mut rng = thread_rng();

    loop {
        queue!(out, Clear(ClearType::All), MoveTo(0,0))?;
        draw(&mut out, &map, px, py, ex, ey)?;

        let status_y = map.len() as u16 + 1;
        queue!(
            out,
            MoveTo(0, status_y),
            Clear(ClearType::CurrentLine),
            Print(format!("HP: {hp}"))
        )?;

        let msg_y = map.len() as u16 + 2;
        queue!(
            out,
            MoveTo(0, msg_y),
            Clear(ClearType::CurrentLine),
            Print(&message)
        )?;

        out.flush()?;

        match read()? {
            Event::Key(event) => {
                message.clear();
                message.push_str("WASD to move, q to quit");

                if let KeyCode::Char('q') = event.code {
                    break;
                }

                let (mut nx, mut ny) = (px, py);
                match event.code {
                    KeyCode::Char('w') => ny -= 1,
                    KeyCode::Char('s') => ny += 1,
                    KeyCode::Char('a') => nx -= 1,
                    KeyCode::Char('d') => nx += 1,
                    _ => {}
                }

                if !is_wall(&map, nx, ny) {
                    px = nx;
                    py = ny;
                }

                // Move Enemy
                (ex, ey) = move_enemy_random(&map, ex, ey, &mut rng);

                // Judge
                if px == ex && py == ey {
                    hp -= 1;
                    message = format!("Ouch! Enemy hit you. HP is now {hp}");
                } else {
                    message = "You moved. Enemy wanderd.".to_string();
                }
            }
            _ => {}
        }
    }

    // End Graphic
    queue!(
        out,
        Clear(ClearType::All),
        MoveTo(0, 0),
        EnableLineWrap,
        Show
    )?;

    out.flush()?;
    disable_raw_mode()?;

    if hp <= 0 {
        println!("GAME OVER");
    } else {
        println!("Bye!");
    }
    Ok(())
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

fn move_enemy_random(
    map: &[Vec<char>],
    ex: i32,
    ey: i32,
    rng: &mut impl rand::Rng,
) -> (i32, i32) {
    let mut dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    dirs.shuffle(rng);

    for (dx, dy) in dirs {
        let nx = ex + dx;
        let ny = ey + dy;
        if !is_wall(map, nx, ny) {
            return (nx, ny);
        }
    }
    (ex, ey)
}

fn draw(
    out: &mut impl Write,
    map: &[Vec<char>],
    px: i32,
    py: i32,
    ex: i32,
    ey: i32) -> io::Result<()> {
    for (y, row) in map.iter().enumerate() {
        let mut line = String::with_capacity(row.len());
        for (x, &ch) in row.iter().enumerate() {
            let xi = x as i32;
            let yi = y as i32;

            let c = if xi == px && yi == py {
                '@'
            } else if xi == ex && yi == ey {
                'g'
            } else {
                ch
            };
            line.push(c);
        }
        queue!(out, MoveTo(0, y as u16), Print(&line))?;
    }
    Ok(())
}

