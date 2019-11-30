fn make_vigenere_square() -> Vec<Vec<char>> {
    let mut vigenere_square: Vec<Vec<char>> = Vec::new();

    let mut alphabets = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    for i in 0..26 {
        let mut v: Vec<char> = Vec::new();
        for j in 0..26 {
            let mut index: usize = i + j;
            if index >= 26 {
                index %= 26;
            }
            v.push(alphabets[index])
        }
        vigenere_square.push(v);
    }

    return vigenere_square;
}

fn main() {
    let key = "ARM";
    let plain = "CODE";

    // vigenere_square[key][plain] == encrypt
    let vigenere_square = make_vigenere_square();
    
}