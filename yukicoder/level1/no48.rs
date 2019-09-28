struct Robot {
    x: i32,
    y: i32,
    direction: String,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            direction: String::from("N"),
        }
    }

    fn move_x(&mut self, dx: i32) {
        self.x += dx;
    }

    fn move_y(&mut self, dy: i32) {
        self.y += dy;
    }

    fn rotate_90_degree(&mut self) {
        let now = &self.direction;
        if *now == String::from("N") {
            self.direction = String::from("E");
        } else if *now == String::from("E") {
            self.direction = String::from("S");
        } else if *now == String::from("S") {
            self.direction = String::from("W");
        } else if *now == String::from("W") {
            self.direction = String::from("N")
        }
    }

    fn rotate_minus_90_degree(&mut self) {
        let now = &self.direction;
        if *now == String::from("N") {
            self.direction = String::from("W");
        } else if *now == String::from("W") {
            self.direction = String::from("S");
        } else if *now == String::from("S") {
            self.direction = String::from("E");
        } else if *now == String::from("E") {
            self.direction = String::from("N")
        }
    }
}

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let x_goal: i32 = getline().trim().parse().unwrap();
    let y_goal: i32 = getline().trim().parse().unwrap();
    let robot_can_move_distance: i32 = getline().trim().parse().unwrap();

    let mut remain_x: i32 = x_goal.abs();
    let mut remain_y: i32 = y_goal.abs();

    let mut order_count: i32 = 0;
    let mut robot = Robot::new();

    // xが0, yがマイナスの時
    if x_goal == 0 && y_goal < 0 {
        robot.rotate_90_degree();
        order_count += 1;
        robot.rotate_90_degree();
        order_count += 1;

        loop {
            if remain_y <= 0 {
                break;
            }

            if remain_y - robot_can_move_distance <= 0 {
                robot.move_y(remain_y * -1);
            } else {
                robot.move_y(robot_can_move_distance);
            }
            remain_y -= robot_can_move_distance;
            order_count += 1;
        }
        println!("{}", order_count);
        return;
    }

    // 1. 最初の方向を決める
    if y_goal < 0 {
        if x_goal > 0 {
            robot.rotate_90_degree();
        } else if x_goal < 0 {
            robot.rotate_minus_90_degree();
        }
        order_count += 1;
    }

    // 2. 進行する
    if robot.direction == String::from("N") {
        loop {
            if remain_y <= 0 {
                break;
            }

            if remain_y - robot_can_move_distance <= 0 {
                robot.move_y(remain_y);
            } else {
                robot.move_y(robot_can_move_distance);
            }
            remain_y -= robot_can_move_distance;
            order_count += 1;
        }
    } else if robot.direction == String::from("E") {
        loop {
            if remain_x <= 0 {
                break;
            }

            if remain_x - robot_can_move_distance <= 0 {
                robot.move_x(remain_x);
            } else {
                robot.move_x(robot_can_move_distance);
            }
            remain_x -= robot_can_move_distance;
            order_count += 1;

        }
    } else if robot.direction == String::from("W") {
        loop {
            if remain_x <= 0 {
                break;
            }

            if remain_x - robot_can_move_distance <= 0 {
                robot.move_x(remain_x * -1);
            } else {
                robot.move_x(robot_can_move_distance * -1);
            }
            remain_x -= robot_can_move_distance;
            order_count += 1;
        }
    }

    // 3. 回転する
    if robot.direction == String::from("N") {
        if x_goal > 0 {
            robot.rotate_90_degree();
            order_count += 1;
        } else if x_goal < 0 {
            robot.rotate_minus_90_degree();
            order_count += 1;
        }
    } else if robot.direction == String::from("E") {
        robot.rotate_90_degree();
        order_count += 1;
    } else if robot.direction == String::from("W") {
        robot.rotate_minus_90_degree();
        order_count += 1;
    }

    // 4. 残りの分進行する
    if robot.direction == String::from("E") {
        loop {
            if remain_x <= 0 {
                break;
            }

            if remain_x - robot_can_move_distance <= 0 {
                robot.move_x(remain_x);
            } else {
                robot.move_x(robot_can_move_distance);
            }
            remain_x -= robot_can_move_distance;
            order_count += 1;

        }
    } else if robot.direction == String::from("W") {
        loop {
            if remain_x <= 0 {
                break;
            }

            if remain_x - robot_can_move_distance <= 0 {
                robot.move_x(remain_x * -1);
            } else {
                robot.move_x(robot_can_move_distance * -1);
            }
            remain_x -= robot_can_move_distance;
            order_count += 1;

        }
    } else if robot.direction == String::from("S") {
        loop {
            if remain_y <= 0 {
                break;
            }

            if remain_y - robot_can_move_distance <= 0 {
                robot.move_y(remain_y * -1);
            } else {
                robot.move_y(robot_can_move_distance * -1);
            }
            remain_y -= robot_can_move_distance;
            order_count += 1;

        }
    }

    println!("{}", order_count);
}