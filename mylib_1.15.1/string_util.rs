#![crate_type = "lib"]
#![crate_name = "string_util"]

// 1行取得する
pub fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}