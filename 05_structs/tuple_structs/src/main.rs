struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Tuple structs are useful when you want your entire tuple to have a name and be of different type than other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
