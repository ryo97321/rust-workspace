fn main() {
    let word = "Hello";
    let mut is_first_character_vowel = false;   // 文字列の最初の文字が母音かどうか

    let first_character = word.chars().nth(0).unwrap();

    if first_character == 'a' || first_character == 'i' || first_character == 'u' || first_character == 'e' || first_character == 'o' {
        is_first_character_vowel = true;
    }

    let mut result = String::new();

    if is_first_character_vowel {
        result = String::from(word) + "-hay";
    } else {
        let word_without_firstcharacter: &str = &word[1..word.len()];
        result = format!("{}-{}ay", word_without_firstcharacter.to_string(), first_character.to_string());
    }

    println!("{}", result);
}