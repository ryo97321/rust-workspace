#![crate_type = "lib"]
#![crate_name = "string_util"]

// 1行取得する
pub fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

// Stringを逆にして返す（ex. test -> tset）
pub fn reverse_str(input: &String) -> String {
    let mut reversed = String::new();
    let mut chars: Vec<char> = Vec::new();

    for c in input.chars().into_iter() {
        chars.push(c);
    }

    for i in (0..chars.len()).rev() {
        reversed += &chars[i].to_string();
    }

    return reversed;
}