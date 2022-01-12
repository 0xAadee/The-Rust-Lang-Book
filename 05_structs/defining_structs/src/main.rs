// Structs allow you to group related data together
// Similar to object attributes in object oriented programming
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("john@test.com"),
        username: String::from("john123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("cooljohn123");

    let user2 = build_user(String::from("jane@test.com"), String::from("jane456"));

    let user3 = User {
        email: String::from("jace@test.com"),
        username: String::from("jace000"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    // Since the function arguments have the same name as the fields in our User Struct
    // We can simplfy it
    // This is called 'Field init shorthand syntax'
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
