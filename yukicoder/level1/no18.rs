fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn codepoint_to_alphabet(codepoint: i32) -> &'static str {
    let alphabet = match codepoint {
        65 => "A",
        66 => "B",
        67 => "C",
        68 => "D",
        69 => "E",
        70 => "F",
        71 => "G",
        72 => "H",
        73 => "I",
        74 => "J",
        75 => "K",
        76 => "L",
        77 => "M",
        78 => "N",
        79 => "O",
        80 => "P",
        81 => "Q",
        82 => "R",
        83 => "S",
        84 => "T",
        85 => "U",
        86 => "V",
        87 => "W",
        88 => "X",
        89 => "Y",
        90 => "Z",
        97 => "a",
        98 => "b",
        99 => "c",
        100 => "d",
        101 => "e",
        102 => "f",
        103 => "g",
        104 => "h",
        105 => "i",
        106 => "j",
        107 => "k",
        108 => "l",
        109 => "m",
        110 => "n",
        111 => "o",
        112 => "p",
        113 => "q",
        114 => "r",
        115 => "s",
        116 => "t",
        117 => "u",
        118 => "v",
        119 => "w",
        120 => "x",
        121 => "y",
        122 => "z",
        _ => "",
    };
    alphabet
}

fn main() {
    let mut s = getline();
    s = s.trim().to_string();

    let s_vec: Vec<char> = s.chars().collect();

    let mut decrypted_s: String = "".to_string();
    for (ui, c) in s_vec.into_iter().enumerate() {
        let i = ui as i32;
        let codepoint = c as i32;
        let mut decrypted_codepoint = codepoint - ((i+1) % 26);

        if codepoint >= 97 && codepoint <= 122 {            // a ~ z
            if decrypted_codepoint < 97 {
                decrypted_codepoint += 26;
            }
        } else if codepoint >= 65 && codepoint <= 90 {      // A ~ Z
            if decrypted_codepoint < 65 {
                decrypted_codepoint += 26
            }
        }

        let alphabet = codepoint_to_alphabet(decrypted_codepoint);
        decrypted_s += alphabet;
    }

    println!("{}", decrypted_s);
}