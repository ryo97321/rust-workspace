fn main() {
    let character_table_lower = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    let character_table_upper = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    let encrypt_text = String::from("fsdz{Fdhvdu_flskhu_lv_fodvvlfdo_flskhu}");

    let mut encrypt_characters: Vec<char> = Vec::new();
    for c in encrypt_text.chars().into_iter() {
        encrypt_characters.push(c);
    }

    let mut key = 1;
    let character_pattern_length: usize = 26;

    // 全パターンを検索する
    for _ in 0..character_pattern_length {
        let mut plain_text = String::new();

        for ec in &encrypt_characters {

            if ec.is_ascii_uppercase() {

                for (i, c) in character_table_upper.iter().enumerate() {
                    if ec == c {
                        let mut plain_index = i + (key as usize);
                        if plain_index >= character_table_upper.len() {
                            plain_index -= character_table_upper.len(); 
                        }
                        plain_text += &character_table_upper[plain_index].to_string();
                        break;
                    }
                }

            } else if ec.is_ascii_lowercase() {

                for (i, c) in character_table_lower.iter().enumerate() {
                    if ec == c {
                        let mut index = i + (key as usize);
                        if index >= character_table_lower.len() {
                            index -= character_table_lower.len();
                        }
                        plain_text += &character_table_lower[index].to_string();
                        break;
                    }
                }

            } else {
                plain_text += &ec.to_string();
            }
        }

        println!("{} : {}", key, plain_text);
        key += 1;
    }
}
