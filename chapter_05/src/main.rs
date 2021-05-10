fn main() {
    println!("Hello, world!");

    // instantinating a struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    // instantianating and shadowing with a mutable struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    // changing a struct field because its mutable
    user1.email = String::from("changed_email@typograph.ai");

    // testing the update syntax
    let user1 = build_user(String::from("someone@somewhere.sm"), String::from("idk.."));
    let user2 = User {
        email: String::from("kek@kekness.kek"),
        username: String::from("kek"),
        ..user1
    };
    println!("user2.active: {}", user2.active);

    // testing out tuple structs
    let black = Color(0, 0, 0); // even though their fields have the same value
    let origin = Point(0, 0, 0); // these structs arent equal
}

// expirementing with structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structrs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// creating structs with a shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,    // parameter and field have the same name
        username, // username: username is skipped
        active: true,
        sign_in_count: 1,
    }
}
