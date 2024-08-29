fn hello_thread() {
    println!("Hello from thread")
}

fn hello_thread2(n: u32) {
    println!("Hello from thread {}", n)
}

fn do_math(i: u32) -> u32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= i
    }
    n
}

fn main() {
    let thread_handle = std::thread::spawn(hello_thread);
    // when main programs ends all child threads will also end so we need to wait for children
    // to end
    thread_handle.join().unwrap();
    // we need to join threads always or use a scoped thread pool that always waits for
    // threads to end

    // create multiple threads -> returns multiple handlers so we store in vector.
    let mut thread_handles = Vec::new();
    for i in 0..5 {
        // spawn function takes a function with no parameter
        // but we want to provide a parameter, so we instead use closures
        // we 'decorate' that closure with the keyword 'move'
        // when threads are created ownership becomes fuzzy.
        // closures only last for the scope of for loop.
        // threads need data with ownership. so we need to provide a method so that data
        // outlives the closure as closure wont exists by the time the thread starts.
        // so we use 'move' keyword to move ownership of 'i' to thread.
        // (since i is a copyable type, move just copies it)
        let thread_handle = std::thread::spawn(move || {
            hello_thread2(i);
        });
        thread_handles.push(thread_handle);
    }
    // wait for all threads to finish
    // using into here as we will not use handles variable anymore after this.
    thread_handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    // vector of threads but this time joinhandle contains a return value of u32
    let mut thread_handles = Vec::new();
    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || {
            return do_math(i);
        });
        thread_handles.push(thread_handle);
    }
    thread_handles
        .into_iter()
        // unwrap will give the return value
        .for_each(|handle| println!("{}", handle.join().unwrap()));
}
