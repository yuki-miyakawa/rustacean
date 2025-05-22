struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user_string(email: String, username:String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 0,
        active: true,
    }
}

fn build_user_str(email: &str, username:&str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        sign_in_count: 0,
        active: true,
    }
}

fn main() {
    let user1 = build_user_string("test@test.com".to_string(), "test_user".to_string());
    let user2 = build_user_str("test2@test.com", "test_user2");
    println!("{},{},{},{}", user1.email, user1.username, user1.sign_in_count, user1.active);
    println!("{},{},{},{}", user2.email, user2.username, user2.sign_in_count, user2.active);
}
