fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();

    let bean_per_pack: i32 = params[0].parse().unwrap();
    let number_of_packs: i32 = params[1].parse().unwrap();
    let number_of_family: usize = params[2].parse().unwrap();

    let line = getline();
    let family_ages: Vec<_> = line.trim().split(' ').collect();

    let total_bean = bean_per_pack * number_of_packs;
    
    let mut total_family_age = 0;
    for i in 0..number_of_family {
        let family_age: i32 = family_ages[i].parse().unwrap();
        total_family_age += family_age;
    }

    let remain_bean: i32;

    if total_bean < total_family_age {
        remain_bean = -1;
    } else {
        remain_bean = total_bean - total_family_age;
    }

    println!("{}", remain_bean);
}