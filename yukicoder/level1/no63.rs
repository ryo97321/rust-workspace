fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let line = getline();
    let params: Vec<_> = line.trim().split(' ').collect();
    let mut pocky_length: i32 = params[0].parse().unwrap();
    let one_bite_length: i32 = params[1].parse().unwrap();

    let mut bite_count = 0;
    
    loop {
        if pocky_length <= one_bite_length * 2 {
            break;
        }
        pocky_length -= one_bite_length * 2;
        bite_count += 1;
    }

    let yu_can_eat_pocky = bite_count * one_bite_length;
    println!("{}", yu_can_eat_pocky);
}