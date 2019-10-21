fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let s = line.trim();

    let mut char_use_table: Vec<bool> = Vec::new();     // 使用済->true 未使用->false
    for _ in 0..s.len() {
        char_use_table.push(false);
    }

    let mut tree_chars: Vec<char> = Vec::new();
    tree_chars.push('t');
    tree_chars.push('r');
    tree_chars.push('e');
    tree_chars.push('e');
    let mut tree_chars_index: usize = 0;

    let mut valid_chars: Vec<char> = Vec::new();
    for c in s.chars().into_iter() {
        if c == 't' || c == 'r' || c == 'e' {
            valid_chars.push(c);
        }
    }

    let mut tree_count = 0;     // 何個treeができたか
    loop {
        let mut can_make_tree = false;

        for _ in 0..4 {
            let mut s_index = 0;
            for c in s.chars().into_iter() {
                if char_use_table[s_index] == false && c == tree_chars[tree_chars_index] {
                    tree_chars_index += 1;
                    if tree_chars_index >= tree_chars.len() {
                        can_make_tree = true;
                        tree_chars_index = 0;
                    }
                    char_use_table[s_index] = true;
                    break;
                }
                s_index += 1;
            }
        }

        if can_make_tree == true {
            tree_count += 1;
        } else {
            break;
        }
    }

    println!("{}", tree_count);
}