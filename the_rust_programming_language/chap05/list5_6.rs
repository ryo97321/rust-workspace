struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anothername567"),
        ..user1
    };
}