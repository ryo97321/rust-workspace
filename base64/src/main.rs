use std::collections::HashMap;
use std::io::{stdout, Write};

// 1行取得
fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

// base64変換表を作成し, 返す
fn make_conversion_table() -> HashMap<String, String> {
    let mut conversion_table = HashMap::new();

    let values = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "+", "/"];

    for i in 0..values.len() {
        let bin = format!("{:06b}", i as i32);
        conversion_table.insert(bin, values[i].to_string());
    }

    return conversion_table;
}

fn main() {
    print!("Input : ");
    stdout().flush().unwrap();
    let plain = getline().trim().to_string();        // 変換する文字列を取得

    let mut all_bits = String::new();   // plainをバイナリ化したものを格納

    // plainをバイナリ化してall_bitsに格納
    for c in plain.chars().into_iter() {
        let bin = format!("{:08b}", c as i32);
        all_bits += &bin;
    }

    let mut six_bits: Vec<String> = Vec::new();     // バイナリを6bitごとに分割したもの
    let mut six_bit = String::new();                // six_bitsに格納する6ビット
    let mut count = 0;
    for c in all_bits.chars().into_iter() {
        count += 1;
        six_bit += &c.to_string();

        // bitsに6ビット入れたらsix_bitsに格納し, bitsをクリアする
        if count % 6 == 0 {
            six_bits.push(six_bit);
            six_bit = "".to_string();
        }
    }

    // bitsにビットが入っていたら, 6ビットにしてsix_bitsに格納する
    if six_bit.len() > 0 {
        let add_zero_count = 6 - six_bit.len();
        let mut add_zero_str = String::new();
        for _ in 0..add_zero_count {
            add_zero_str += "0";
        }
        six_bit += &add_zero_str;
        six_bits.push(six_bit);
    }

    let conversion_table = make_conversion_table();     // base64変換表

    let mut base64_str = six_bits.iter().flat_map(|six_bit| conversion_table[six_bit].chars()).collect::<String>();     // base64に変換した文字列

    // 足りない分"="を, base64_strに追加する
    if base64_str.len() % 4 != 0 {
        let mut equal_str = String::new();
        let add_equal_count = 4 - (base64_str.len() % 4);

        for _ in 0..add_equal_count {
            equal_str += "=";
        }
        base64_str += &equal_str;
    }

    // 変換結果を表示
    println!("Base64 : {}", base64_str);
}
