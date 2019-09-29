fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

// 分で返される
fn get_sleeptime(wakeuptime_hour: i32, wakeuptime_minute: i32, bedtime_hour: i32, bedtime_minute: i32) -> i32 {
    let sleeptime;
    if bedtime_hour < 24 && bedtime_hour > 12 {
        // bedtime_hourから0:00までの時間 + 0:00からwakeuptimeまでの時間
        sleeptime = ((24 * 60) - (bedtime_hour * 60 + bedtime_minute)) + (wakeuptime_hour * 60 + wakeuptime_minute);
    } else {
        // 0:00からwakeuptimeまでの時間 - 0:00からbedtimeの時間
        if (wakeuptime_hour == 0 && wakeuptime_minute == 0) && (bedtime_hour == 0 && bedtime_minute == 1) {
            sleeptime = (24 * 60) - (bedtime_hour * 60 + bedtime_minute);
        } else {
            sleeptime = (wakeuptime_hour * 60 + wakeuptime_minute) - (bedtime_hour * 60 + bedtime_minute);
        }
    }
    return sleeptime;
}

fn main() {
    let mut sleeptime_total = 0;
    let n: i32 = getline().trim().parse().unwrap();
    
    for _ in 0..n {
        let line = getline();
        let params: Vec<_> = line.trim().split(' ').collect();
        let bedtime = params[0];
        let wakeuptime = params[1];

        let wakeuptime_params: Vec<_> = wakeuptime.trim().split(':').collect();
        let wakeuptime_hour: i32 = wakeuptime_params[0].trim().parse().unwrap();
        let wakeuptime_minute: i32 = wakeuptime_params[1].trim().parse().unwrap();

        let bedtime_params: Vec<_> = bedtime.trim().split(':').collect();
        let bedtime_hour: i32 = bedtime_params[0].trim().parse().unwrap();
        let bedtime_minute: i32 = bedtime_params[1].trim().parse().unwrap();

        let sleeptime = get_sleeptime(wakeuptime_hour, wakeuptime_minute, bedtime_hour, bedtime_minute);
        sleeptime_total += sleeptime;
    }

    println!("{}", sleeptime_total);
}
