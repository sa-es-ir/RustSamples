fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("saeed"),
        email: String::from("mock@email.com"),
        sign_in_count: 10,
    };

    println!("old username: {}", user1.username);

    user1.username = String::from("new_saeed");
    println!("new username: {}", user1.username);

    let new_user = build_user(String::new(), String::from("from_build@mail.com"));

    println!("username: {} email: {}", new_user.username, new_user.email);

    let user2 = User {
        email: String::from("new_email2@mail.com"),
        ..user1
    };

    // here can not use user1.username since it already borrowed by user2
    println!(
        "user2 --> username: {} email: {}",
        user1.sign_in_count, user2.email
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let alwaysEqual = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple struct

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;
