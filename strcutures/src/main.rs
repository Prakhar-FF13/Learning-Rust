// enums indicate a choice they arent used to combining data logically
// that is where strcutures come into play

#[derive(Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug)]
pub struct User {
    // field accessible by objects is denoted by pub
    pub username: String,
    pub email: String,
    password: String,
    pub role: Role, // structures can contain enums
}

// methods are defined using the impl block
impl User {
    // a constructor new (just a convention nothing related to rust)
    // pub makes sure the function is callable outside this module
    pub fn new(username: &str, email: &str, password: &str, role: Role) -> User {
        // Self refers to the type currently implementing for
        // can also use User
        Self {
            username: username.to_lowercase(),
            email: email.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }

    fn check_password(&self, password: &str) -> bool {
        self.password == password.to_string()
    }
}

// returns a static array of dummy data
pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "admin@localhost", "password", Role::Admin),
        User::new("user", "user@localhost", "password", Role::User),
    ]
}

fn main() {
    let users = get_users();

    // iterators are used to access list, vectors etc.
    for user in users.iter() {
        println!("{:?}", user); // needs the debug implemented
    }

    // iterators can be chained, like iter returns an iterator which is changed to find
    // |user| defines a closure -> inline function
    // every time iter runs find gets a pointer to user.
    // find returns an option
    // if user is found, it is returned
    if let Some(user) = users.iter().find(|user| user.username == "admin") {
        // user here is owned by the users array we only have a reference.
        println!("{:?}", user);
        if user.check_password("password") {
            println!("Logged in");
            match user.role {
                Role::Admin => println!("Admin Role"),
                Role::User => println!("User Role"),
            }
        }
    }
}
