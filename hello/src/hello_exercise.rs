fn main() {
    let hello = String::from("Hello");
    let hello_rusty = give_me_back(&hello);

    assert_eq!(hello, "Hello");
    assert_eq!(hello_rusty, "Hello my Rustacean friend")
}

fn give_me_back(message: &String) -> String {
    let mut another_message = message.clone();
    another_message.push_str(" my Rustacean friend");
    return another_message;
}
