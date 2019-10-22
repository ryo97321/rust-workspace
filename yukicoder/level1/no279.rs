fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let s = line.trim();

    let mut tree_count = 0;     // 何個treeができたか

    let mut t_vector: Vec<char> = Vec::new();
    let mut r_vector: Vec<char> = Vec::new();
    let mut e_vector: Vec<char> = Vec::new();

    for c in s.chars().into_iter() {
        if c == 't' {
            t_vector.push(c);
        } else if c == 'r' {
            r_vector.push(c);
        } else if c == 'e' {
            e_vector.push(c);
        }
    }

    let mut t_vector_index: usize = 0;
    let mut r_vector_index: usize = 0;
    let mut e_vector_index: usize = 0;

    loop {
        if t_vector_index < t_vector.len() && r_vector_index < r_vector.len() && e_vector_index+1 < e_vector.len() {
            tree_count += 1;
        }

        t_vector_index += 1;
        r_vector_index += 1;
        e_vector_index += 2;

        if t_vector_index >= t_vector.len() || r_vector_index >= r_vector.len() || e_vector_index >= e_vector.len() {
            break;
        }
    }

    println!("{}", tree_count);
}