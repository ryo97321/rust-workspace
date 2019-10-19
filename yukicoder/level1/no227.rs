fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut cards: Vec<usize> = Vec::new();
    for i in 0..params.len() {
        let card: usize = params[i].parse().unwrap();
        cards.push(card);
    }

    let mut card_counts = vec![0; 14];

    let mut is_fullhouse = false;
    let mut is_threecard = false;
    let mut is_twopair = false;
    let mut is_onepair = false;

    // count card
    for i in 0..cards.len() {
        let card_number = cards[i];
        card_counts[card_number] += 1;
    }

    let mut is_three_set = false;   // 三枚組か否か
    let mut pair_count = 0;         // 二枚組の個数

    // check hand
    for i in 0..card_counts.len() {
        if card_counts[i] == 3 {
            is_three_set = true;
        }

        if card_counts[i] == 2 {
            pair_count += 1;
        }
    }

    // judge hand
    if is_three_set == true {
        if pair_count == 1 {
            is_fullhouse = true;
        } else {
            is_threecard = true;
        }
    } else {
        if pair_count == 1 {
            is_onepair = true;
        } else if pair_count == 2 {
            is_twopair = true;
        }
    }

    if is_fullhouse {
        println!("FULL HOUSE");
    } else if is_threecard {
        println!("THREE CARD");
    } else if is_twopair {
        println!("TWO PAIR");
    } else if is_onepair {
        println!("ONE PAIR");
    } else {
        println!("NO HAND");
    }
}