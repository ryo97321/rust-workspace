use std::collections::HashMap;

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn morsecode_to_text(morsecode: &str) -> String {
    let mut morsecode_table = HashMap::new();
    morsecode_table.insert(".-", 'A');
    morsecode_table.insert("-...", 'B');
    morsecode_table.insert("-.-.", 'C');
    morsecode_table.insert("-..", 'D');
    morsecode_table.insert(".", 'E');
    morsecode_table.insert("..-.", 'F');
    morsecode_table.insert("--.", 'G');
    morsecode_table.insert("....", 'H');
    morsecode_table.insert("..", 'I');
    morsecode_table.insert(".---", 'J');
    morsecode_table.insert("-.-", 'K');
    morsecode_table.insert(".-..", 'L');
    morsecode_table.insert("--", 'M');
    morsecode_table.insert("-.", 'N');
    morsecode_table.insert("---", 'O');
    morsecode_table.insert(".--.", 'P');
    morsecode_table.insert("--.-", 'Q');
    morsecode_table.insert(".-.", 'R');
    morsecode_table.insert("...", 'S');
    morsecode_table.insert("-", 'T');
    morsecode_table.insert("..-", 'U');
    morsecode_table.insert("...-", 'V');
    morsecode_table.insert(".--", 'W');
    morsecode_table.insert("-..-", 'X');
    morsecode_table.insert("-.--", 'Y');
    morsecode_table.insert("--..", 'Z');
    morsecode_table.insert(".-.-.-", '.');
    morsecode_table.insert("--..--", ',');
    morsecode_table.insert("---...", ':');
    morsecode_table.insert("..--..", '?');
    morsecode_table.insert(".-.-..", '「');
    morsecode_table.insert("-.--.", '(');
    morsecode_table.insert("-.--.-", ')');
    morsecode_table.insert("..--.-", '_');
    morsecode_table.insert("-----", '0');
    morsecode_table.insert(".----", '1');
    morsecode_table.insert("..---", '2');
    morsecode_table.insert("...--", '3');
    morsecode_table.insert("....-", '4');
    morsecode_table.insert(".....", '5');
    morsecode_table.insert("-....", '6');
    morsecode_table.insert("--...", '7');
    morsecode_table.insert("---..", '8');
    morsecode_table.insert("----.", '9');

    let mut text = String::new();

    let morsecode_parts: Vec<_> = morsecode.split(" ").collect();

    for morsecode_part in morsecode_parts {
        let c = morsecode_table.get(morsecode_part);
        if c == None {
            text += "*";
        } else {
            text += &c.unwrap().to_string();
        }
    }
    text
}

fn main() {
    let line = getline();
    let morsecode = line.trim();
    let text = morsecode_to_text(morsecode);

    println!("{}", morsecode);
    println!("{}", text);
}
