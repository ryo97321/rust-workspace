fn main() {
    let mut word = String::from("first");
    let mut is_first_char_vowel = false;
    
    if &word[0..1] == "a" || &word[0..1] == "i" || &word[0..1] == "u" || &word[0..1] == "e" || &word[0..1] == "o" {
        is_first_char_vowel = true;
    }

    if is_first_char_vowel {
        word = format!("{}-hay", &word);
    } else {
        word = format!("{}-{}ay", &word[1..], &word[0..1]);
    }

    println!("{}", word);
}