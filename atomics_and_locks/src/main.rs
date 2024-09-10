use std::sync::{atomic::AtomicI32, Mutex, RwLock};

use once_cell::sync::Lazy;

fn unsafe_code() {
    // using static so that we dont need to move in closure and worry about ownership
    static mut COUNTER: i32 = 0;
    // a code that is valid in another language but not in rust
    let mut handlers = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    // compiler gives a warning if not unsafe used
                    COUNTER += 1;
                }
            }
        });
        handlers.push(handle);
    }

    handlers
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    unsafe {
        // every time a different result is returned -> data race
        println!("{}", COUNTER);
    }
}

fn atomic_safe_code() {
    // A Safe way to increment counter -> syncrhonization primitives prevent this data race
    // Atomics are easiest way to make numeric operations thread safe -> as no context switch
    // not making below mutable -> coz of interior mutability.
    // interior mutability is when a type changes itself but the outside you are seeing doesnt change
    // mut is exterior mutability
    static COUNTER2: AtomicI32 = AtomicI32::new(0);
    let mut handlers = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1000 {
                // adding 1 so val is 1,
                // ordering is relaxed as thread running cannot be controlled
                // atomics makes sure context switch does not happend when adding
                // look at docs for more ordering options some are not supported on some processor.
                // intel/amd/arm processor supports all options
                // book on atomics - https://marabos.nl/atomics/
                COUNTER2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handlers.push(handle);
    }

    handlers
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    // every time a different result is returned -> data race
    println!("{}", COUNTER2.load(std::sync::atomic::Ordering::Relaxed));

    // ATOMICS only work for simple type as atomic only exist for them
    // For complex types use mutexes instead.
}

fn mutexes_safe_code() {
    // wrap the type which is to syncrhonized inside a mutex like String/Vec
    static NUMBERS: Mutex<Vec<i32>> = Mutex::new(Vec::new());
    let mut handlers = Vec::new();
    for _ in 0..10 {
        // spawn 10 threads
        let handle = std::thread::spawn(|| {
            // lock and get numbers array.
            let mut numbers = NUMBERS.lock().unwrap();
            // push element
            numbers.push(1);
            // scope ends lock is dropped.
        });
        handlers.push(handle);
    }

    // wait for threads to finish
    handlers
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    // get lock for printing
    let numbers = NUMBERS.lock().unwrap();
    println!("{:#?}", numbers);

    // mutexes are slower than atomics as taking the locks takes time and releasing it also.
    // cost of using mutexes has to be justified.
    // it might be possible that a single loop without mutex is faster rather than synchronized code
    // mutexes coz other threads to wait for mutex to be released.
}

fn rw_mutex_safe_code() {
    // read mutexes are fast. writes can only be done one at a time.
    // all readers must finish before write lock is acquired.
    // lazy initialization -> when does a global variable becomes initialized.
    // lazy takes in a function which initializes the variable.
    // lazy initialize has to be done if something is not determined at compile time.
    // constructor is not constant -> lazy initialize is needed
    static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

    fn build_users() -> Vec<String> {
        vec!["MS".to_string(), "MJ".to_string()]
    }

    fn read_line() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    // spawn a thread which continously reads
    std::thread::spawn(|| loop {
        println!("current users (in a thread)");
        let users = USERS.read().unwrap();
        println!("{:#?}", users);
        std::thread::sleep(std::time::Duration::from_secs(3));
    });

    // loop to enter names
    loop {
        println!("Enter a name to add to the user list (q to quit)");
        let input = read_line();
        if input == "q" {
            break;
        } else {
            let mut users = USERS.write().unwrap();
            users.push(input);
        }
    }
}

fn main() {
    unsafe_code();
    atomic_safe_code();
    mutexes_safe_code();
    rw_mutex_safe_code();
}
