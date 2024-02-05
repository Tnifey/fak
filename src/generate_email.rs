pub fn generate_pre_email(email: &str) -> String {
    let [first, last] = match email.split_once('@') {
        Some((first, last)) => [first, last],
        None => panic!("Invalid email"),
    };

    let int = rand::random::<u32>();
    format!("{}+{}@{}", first, int, last)
}
