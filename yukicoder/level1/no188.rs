fn main() {
    let mut month_max = 12;
    let mut date_left_max = 3;
    let mut date_right_max = 9;

    let mut happy_day_count = 0;

    for month in 1..month_max+1 {
        for date_left in 0..date_left_max+1 {
            for date_right in 0..date_right_max+1 {
                // 0日はあり得ない
                if date_left == 0 && date_right == 0 {
                    continue;
                }

                // 2月は28日まで
                if month == 2 && (date_left*10 + date_right) > 28 {
                    break;
                }

                // 4, 6, 9, 11は30日まで
                if (month == 4 || month == 6 || month == 9 || month == 11) && (date_left*10 + date_right) > 30 {
                    break;
                }

                // 31日より後の日付はあり得ない
                if date_left == 3 && date_right > 1 {
                    break;
                }

                if month == (date_left + date_right) {
                    happy_day_count += 1;
                }
            }
        }
    }

    println!("{}", happy_day_count);
}