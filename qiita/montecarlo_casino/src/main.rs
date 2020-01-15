use rand::Rng;
use std::io::{self, Write};

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let mut v = vec![1, 2, 3];

    let mut bet = v[0] + v[v.len()-1];

    let mut game_count = 1;
    let mut sum = 0;

    let mut max_win = 0;
    let mut max_lost = 0;

    let mut win = 0;
    let mut lost = 0;

    print!("Loop count: ");
    io::stdout().flush().unwrap();
    let loop_count: usize = getline().trim().parse().unwrap();

    for _ in 0..loop_count {
        let judge = rand::thread_rng().gen_range(0, 2);     // judge:1 -> win | judge:0 -> lose

        if judge == 1 {
            win = bet * 2;
            sum += win;
            
            if win > max_win {
                max_win = win;
            }

            v.remove(0);
            v.remove(v.len()-1);
            if v.len() == 0 {
                v.push(1);
                v.push(2);
                v.push(3);
            }
            bet = v[0] + v[v.len()-1];
        } else if judge == 0 {
            lost = bet;
            sum -= lost;
            
            if lost > max_lost {
                max_lost = lost;
            }

            v.push(bet);
            bet = v[0] + v[v.len()-1];
        }

        if v.len() < 2 {
            v.push(1);
            v.push(2);
            v.push(3);
        }

        // view result
        print!("{} | ", game_count);
        if judge == 0 {
            print!("x:-{}", lost);
        } else if judge == 1 {
            print!("o:{}", win);
        }
        println!(" | sum:{}", sum);

        game_count += 1;
    }

    // view max
    println!("max_win: {}", max_win);
    println!("max_lost: -{}", max_lost);
}
