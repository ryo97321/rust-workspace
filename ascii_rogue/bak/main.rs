use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap
    },
};
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

    loop {
        queue!(out, Clear(ClearType::All), MoveTo(0,0))?;

        draw(&mut out, &map, px, py, ex, ey)?;

        let msg_y = map.len() as u16 + 1;

        queue!(
            out,
            MoveTo(0, msg_y),
            Clear(ClearType::CurrentLine),
            Print("WASD to move, q to quit (enemy: g)")
        )?;

        out.flush()?;

        match read()? {
            Event::Key(event) => {
                let (mut nx, mut ny) = (px, py);

                match event.code {
                    KeyCode::Char('q') => break,
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
            }
            _ => {}
        }
    }

    queue!(
        out,
        Clear(ClearType::All),
        MoveTo(0, 0),
        EnableLineWrap,
        Show
    )?;

    out.flush()?;
    disable_raw_mode()?;
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

