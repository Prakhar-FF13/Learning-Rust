// A library contains piece of code shared between multiple projects
// This is a library/crate
// created using cargo new --lib name_of_library
// a library does not get build into a binary - executable
// it is just a set of functions
// pub to export it out of library.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// &str is a pointer to a string in memory, it is non modifiable like a constant
pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

// mod creates a module lives inside a library/crate
// we can run the tests using cargo test
#[cfg(test)]
mod tests {
    // whatever module is above you are inside in
    use super::*;
    // above includes the function add as this is a module and this module resides in another module
    // we include that module in here

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greet_user("Carol");
        assert_eq!("Hello, Carol!", result);
    }
}
