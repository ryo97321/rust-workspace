fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let n_candidate: usize = getline().trim().parse().unwrap();

    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let mut votes: Vec<i32> = Vec::new();
    for i in 0..n_candidate {
        let vote: i32 = params[i].parse().unwrap();
        votes.push(vote);
    }

    let mut total_valid_votes = 0;
    for i in 0..n_candidate {
        let vote = votes[i];
        total_valid_votes += vote;
    }

    let confiscation_criteria = total_valid_votes / 10;

    let deposit_money = 30;
    let mut total_deposit_money = 0;
    for i in 0..n_candidate {
        if votes[i] <= confiscation_criteria {
            total_deposit_money += deposit_money;
        }
    }

    println!("{}", total_deposit_money);
}