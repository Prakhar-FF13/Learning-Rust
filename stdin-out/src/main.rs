fn read_line() -> String {
    // mutable as its changed by standard input
    let mut line = String::new();
    // since stdin will modify it we need to pass mutable reference.
    // this might error if we run the prog with has no input stream
    // but since we know it wont error out in our case we will ignore error handling using unwrap
    std::io::stdin().read_line(&mut line).unwrap();
    // readline will also include \n at the end or carriage return
    // we remove it using trim()
    // trim() however return &str which is a constant string in memory
    line.trim().to_string()

    // there are other streams like stdout, stderr etc.
}

fn main() {
    let line = read_line();
    println!("{}", line);
}
