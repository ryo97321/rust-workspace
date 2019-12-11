fn main() {
    // 暗号化されたフラグ
    let e_flag = vec!["r", "u", "o", "Y", "c", "e", "d", "_", "e", "h", "p", "i", "g", "n", "i", "r", "i", "k", "s", "_", "i", "_", "l", "l", "r", "g", "_", "s", "t", "a", "e"];

    for key in 1..e_flag.len() {
        let mut i: usize = 0;
        let mut flag = String::new();

        loop {
            for j in (i..(i+key)).rev() {
                if j >= e_flag.len() {
                    break;
                } else {
                    flag += &e_flag[j];
                }
            }
            i += key;

            // 残りを表示してbreak
            if i > e_flag.len() {
                i -= key;
                for j in (i..e_flag.len()).rev() {
                    flag += &e_flag[j];
                }
                break;
            }
        }

        // 結果を表示
        println!("{}:{}", key, flag);
    }
}
