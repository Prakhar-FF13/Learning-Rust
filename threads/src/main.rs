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

// a function which prints some info related to thread
fn my_thread() {
    println!(
        "Hello from thread named {}",
        std::thread::current().name().unwrap()
    );
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

    // Dividing Workloads into chunks and handling each chunk in a thread
    let to_add: Vec<u32> = (0..5000).collect(); // 0-4999
    let mut thread_handles: Vec<std::thread::JoinHandle<u32>> = Vec::new();
    let chunks = to_add.chunks(8);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || my_chunk.iter().sum()));
    }

    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }
    println!("Sum is {}", sum);

    // For more control and getting more info about thread you are running.
    // Using thread builder to configure thread
    // std::mem::size_of::<usize> gives size of uint on the machine
    let thread_handle = std::thread::Builder::new()
        .name("named_thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4) // memory size in bytes, default is 2 MiB on linux
        .spawn(my_thread)
        .unwrap();
    thread_handle.join().unwrap();

    // Sharing data using scoped threads - inspired by rayon library.
    // automating joining and auto cleanup as within a scope.
    // all threads terminate when  scope ends
    let chunks = to_add.chunks(8);

    // notice how we dont need to move when using scoped threads
    // scoped threads handles ownership hence moving not needed.
    let sum = std::thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            // no move here either.
            let thread_handle = s.spawn(|| chunk.iter().sum::<u32>());
            thread_handles.push(thread_handle);
        }

        thread_handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<u32>() // no semicolon hence value is returned
    });

    println!("Scoped Thread Sum is {}", sum);
}
