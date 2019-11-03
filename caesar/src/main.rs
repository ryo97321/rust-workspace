fn main() {
    let character_table = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', '.', ',', '-'];

    let encrypt_text = String::from("qdq-gi.q-a ziatmxxitmdqibtqi-ustbi ri.qmoqrcxi.qbubu zir -ibtqi-qp-qaai ripmymsqkir -ibtqi-qy dmxi ri.cnxuoi rruoumxakir -ibtqiqzmobyqzbkii-q.qmxi -imyqzpyqzbi rixmeaki -puzmzoqai -i-qscxmbu zaimzpir -i btq-iymbbq-a;iz -iatmxximzgi.q-a zinqiuzimzgiemgipuao-uyuzmbqpimsmuzabir -ia. za -uzsiacotiimi.qbubu zj");

    let mut encrypt_characters: Vec<char> = Vec::new();
    for c in encrypt_text.chars().into_iter() {
        encrypt_characters.push(c);
    }

    let mut key = 1;

    // 全パターンを検索する
    for _ in 0..character_table.len() {
        let mut plain_text = String::new();

        for ec in &encrypt_characters {
            for (i, c) in character_table.iter().enumerate() {
                if ec == c {
                    let mut character_table_index = i as i32 - key;
                    if character_table_index < 0 {
                        character_table_index += character_table.len() as i32;
                    }
                    plain_text += &character_table[character_table_index as usize].to_string();
                    break;
                }

                // character_tableに該当する文字がなかったら, そのまま追加する
                if i == character_table.len() - 1 {
                    plain_text += &ec.to_string();
                }
            }
        }

        // 文字列中にpersonがあるか探索する
        let plain_text_words: Vec<_> = plain_text.split(' ').collect();
        let mut is_person_found = false;
        for word in plain_text_words {
            if word == "person".to_string() {
                is_person_found = true;
                break;
            }
        }
        if is_person_found {
            println!("\nkey:{} | {}\n", key, plain_text);
        }

        key += 1;
    }
}
