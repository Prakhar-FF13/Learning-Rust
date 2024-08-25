use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct User {
    username: String,
    email: String,
    password: String,
    role: String,
}

impl User {
    fn new(username: String, email: String, password: String, role: String) -> User {
        User {
            username,
            email,
            password,
            role,
        }
    }
}

fn get_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new(
            "admin".to_string(),
            "admin@localhost".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        ),
    );
    users
}

fn main() {
    let users = get_users();

    // unoptimal way of accessing a key.
    if let Some(u) = users.iter().find(|user| user.1.username == "admin") {
        println!("{:?}", u.1);
    }

    // search through hashmap
    if let Some(user) = users.get("admin") {
        println!("{:?}", user);
    }

    // insertion into hashmap is slower than vector insertions (at last position)
    // searching is way faster in hashmaps.
}
