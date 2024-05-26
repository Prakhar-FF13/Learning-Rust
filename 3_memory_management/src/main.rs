fn greet(s: String) {
    println!("Hello, {}!", s);
}

fn greet2(s: String) -> String {
    println!("Hello, {}!", s);
    s
}

fn greet3(s: &String) {
    println!("Hello, {}!", s);

    // cannot mutate s as it does not own it
    // s.push_str(", World!"); // wrong
    // after function ends here local s is dropped but String isn't destroyed.
}

fn greet4(s: &mut String) {
    println!("Hello, {}!", s);

    s.push_str(" World"); // mutation is possible

    // although mutation is possible something like below is not.
    // let mut x = "Hello, World".to_string();
    // s = &mut x;
    /*
        If you uncomment above if you will see the error x does not live long enough
        This is because x is a reference to a string on heap.
        When function ends x is dropped and corresponding memory is also dropped on heap.
        This means - s would point to nothing - dangling pointer
        Hence rust already prevents this.
    */

    // we can directly change the content though
    *s = "Hello, From Greet 4".to_string();

    // after function ends here local s is dropped but String isn't destroyed.
}

fn main() {
    let name = "MJ".to_string();
    greet(name);
    /*
        calling again wont compile
        when you call a function with a non trivial type
        you move the ownership
        here name is in main so it has ownership of it.
        when you call greet and pass this non trivial type
        you gave the ownership to greet
        once ownership is given and function exits scope of function is over.
        anything that was owned in this function is dropped hence name no longer exists.
    */
    // greet(name);

    /*
       what if we dont want to give ownership?
       we use the greet 2 function
       we call it and pass it a non trivial type
       ownership is transferred and then returned.
    */
    let name = "MJ".to_string();
    let name = greet2(name); // ownership is transferred and returned to new variable
    println!("{}", name);

    /*
       Is there a better way to do above?
       Yes we can give a copy to greet function
    */
    greet(name.clone()); // clone can be slow so not used much

    /*
       Another way is to not give ownership to greet just let it borrow the value
       greet3 function only borrows the value
       A borrowed value cannot be mutated i.e greet3 function cannot change the value
       As it does not own it.
       Analogy - i give my friend keys to my car. he can drive it and then return it.
       He does own it so he cannot go make changes to its engine/paint etc. Only i have that right now.
    */
    greet3(&name); // send through borrow.

    println!("{}", name); // name is still accessible here.

    /*
        What if we want to use borrow feature but also allow the function to mutate it?
        see greet4 function is asks for a mutable borrow.
    */
    let mut name = "MJ".to_string();
    greet4(&mut name); // need a mutable variable to give a mutable reference.
    println!("{}", name);

    /*
       SOME POINTERS:
       we can have as many as immutable borrows as well.
       but only one mutable borrow can exists at a time.
       This is similar to Multiple Read locks at same time but only one write lock at a time.
    */
}
