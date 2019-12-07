fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(" ").collect();

    let roman_num_now = params[0];
    let dt: i32= params[1].parse().unwrap();

    let num_now = match roman_num_now {
        "I" => 1,
        "II" => 2,
        "III" => 3,
        "IIII" => 4,
        "V" => 5,
        "VI" => 6,
        "VII" => 7,
        "VIII" => 8,
        "IX" => 9,
        "X" => 10,
        "XI" => 11,
        "XII" => 12,
        _ => 0,
    };

    let mut num_after = num_now + dt;


    if num_after % 12 == 0 {
        num_after = 12;
    } else {
        num_after %= 12;
    }

    if num_after < 0 {
        num_after += 12;
    }


    let roman_num_after = match num_after {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IIII",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        11 => "XI",
        12 => "XII",
        _ => "error"
    };

    println!("{}", roman_num_after);
}