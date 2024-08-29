fn main() {
    let n = 5;
    // variables are immutable by default
    // cannot do the following
    // n += 1
    println!("The value of n is: {}", n);

    // to make a variable mutable, use the `mut` keyword
    // notice previous n gets shadowed
    let mut n = 5;
    n += 1;
    println!("The value of mutable n is: {}", n);

    // below we create a scope using { }
    // in the scope previous n is shadowed by the n declared in inner scope
    {
        let n = 100;
        println!("The value of n in the inner scope is: {}", n);
    }

    // here the n in the outer scope is not shadowed
    println!("The value of n in the outer scope is: {}", n);

    // we can also return values from a scope/block
    // dont need a return keyword
    // make sure not to add a ';' on last line of block
    let n = { 7 };
    println!("The value of n after returning from block is: {}", n);

    // what if we add a ';' on the last line?
    // return type is the unit type '()'
    // everything in rust returns a type whether you explicitly say it or not.
    // unit type is basically saying nothing
    let _n = {
        let _x = 5;
    };
    // cannot print a unit type
    // println!("The value of n is: {}", n);

    // functions
    let n = 2;
    println!("The value of 2 after doubling is: {}", double(n));

    // ternary operator
    let n = if n == 2 { 5 } else { 6 };
    println!("The value of n after comparison is: {}", n);
}

// can declare functions using fn name(args) -> return type {}
fn double(n: i32) -> i32 {
    n * 2 // no semicolon means implicit return
}
