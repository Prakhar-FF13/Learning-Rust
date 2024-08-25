// Get serde package - cargo add serde -F derive
// -F is the feature flag to install required features.
// cargo add serde_json also to get json serial/deserial

use std::{collections::HashMap, fs, path::Path};

// import serde
use serde::{Deserialize, Serialize};

// add traits to auto generate code for serialize/deserialize
// compiler detects the format and auto generates code
// if a complex type is present inside struct then that struct also would need these traits
#[derive(Serialize, Deserialize, Debug)]
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

fn get_default_users() -> HashMap<String, User> {
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
    users.insert(
        "user".to_string(),
        User::new(
            "user".to_string(),
            "user@localhost".to_string(),
            "user".to_string(),
            "user".to_string(),
        ),
    );
    users
}

fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // read the file
        let contents = fs::read_to_string(users_path).unwrap();
        // deserialize
        let users: HashMap<String, User> = serde_json::from_str(&contents).unwrap();
        users
    } else {
        let users = get_default_users();
        let contents = serde_json::to_string(&users).unwrap();
        fs::write(users_path, contents).unwrap();
        users
    }
}

fn main() {
    get_users();
}
