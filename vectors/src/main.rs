// Vectors are like an array, they can have variable size.
// They live on heap hence slightly slower to access as pointer is involved.
// They are fast and very similar to C++ vector just safe by default (as long as unsafe methods not used).
// Queues/Stack etc all use vectors underhood.
#[derive(Debug, PartialEq)]
enum Role {
    Admin,
    User,
}

#[derive(Debug, PartialEq)]
struct User {
    username: String,
    email: String,
    password: String,
    role: Role,
}

impl User {
    fn new(username: &str, email: &str, password: &str, role: Role) -> User {
        Self {
            username: username.to_lowercase(),
            email: email.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

fn get_users() -> Vec<User> {
    vec![User::new("user", "user", "password", Role::User)]
}

fn main() {
    let mut users = get_users();
    users.push(User::new("admin", "admin", "password", Role::Admin));

    // into_iter moves unlike iter which gives a reference.
    let admin_users = users
        .into_iter()
        .filter(|user| user.role == Role::Admin)
        .map(|user| user.username)
        .collect::<Vec<String>>();
}
