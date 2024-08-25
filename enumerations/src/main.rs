// Enums in rust are powerful than C-Enums
// They can specify the most basic to complex structural types.

// pub allows it to be used outside the library.
pub enum E1 {
    LoggedIn,
    NotLoggedIn,
}

// adding this, the compiler will generate code to allow comparison between enums
// given it is a simple enum
#[derive(PartialEq, Eq, Debug)]
pub enum E2 {
    LoggedIn,
    NotLoggedIn,
}

#[derive(PartialEq, Debug)]
pub enum E3 {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum E4 {
    Granted(E3), // complex enum with enum inside
    Denied,
}

fn main() {
    // create an enum variable, mut to be changeable
    let mut _x = E1::LoggedIn;
    _x = E1::NotLoggedIn;
    // uncomment below - shows an error telling the enums cannot be compared.
    // let _x = E1::LoggedIn == E1::LoggedIn;

    // E2 has the comparison code using derive so it can be compared.
    let _x = E2::LoggedIn == E2::LoggedIn;

    println!("{:?}", _x);

    // assert eq need the Debug function to be implemented for enum
    // we already did it with derive
    assert_eq!(E2::LoggedIn, E2::LoggedIn);

    // complex enum
    let _x = E3::Admin;
    let _y = E4::Granted(_x);

    // pattern matching on enums
    // match gives error if every case is not handled
    // for E4 we have to handle Granted and Denied both
    match _y {
        // _z variable here captures whatever Granted(E3) is
        E4::Granted(_z) => match _z {
            // handle cases for E3
            E3::Admin => println!("Admin"),
            E3::User => println!("User"),
        },
        // can also match a more granular enum
        // E4::Granted(E3::Admin) => println!("Admin"),
        // E4::Granted(E3::User) => println!("User"),
        // if you uncomment above two, you get unreachable code as match already contains  a case to handle both.
        E4::Denied => {
            // do nothing
            println!("Denied")
        }
    }

    // A very common Enum is an Option Enum
    // Option<T> is a enum with two variants
    // Some(T) - value
    // None - no value
    let _x: Option<i32> = Some(5);
    let _y: Option<i32> = None;

    // what if we need to the value 5 out of the _x Option
    // you cannot use the value of complex enum without match
    // or using an if let.

    // this checks whether x contains Some value 'x'
    if let Some(x) = _x {
        println!("The value is: {}", x);
    }

    // above is similar to but in match we have to handle every case whereas if let allows
    // you to handle only one case
    match _x {
        // extract the value
        Some(x) => println!("The value is: {}", x),
        None => println!("No value"),
    }

    // None can stop null pointer exceptions
}
