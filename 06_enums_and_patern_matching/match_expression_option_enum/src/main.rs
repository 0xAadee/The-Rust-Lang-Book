fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match expressions are exhaustive, that means we have to match all possible values
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}