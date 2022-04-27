use std::fs::File;
use std::sync::{Arc, Mutex};
use std::{thread};
use std::collections::HashMap;
use std::os::unix::prelude::FileExt;
use std::thread::JoinHandle;
use std::time::Duration;
use rand::{Rng};

const COUNT: usize = 10;

fn main() {
    let file = Arc::new(Mutex::new(File::create("foo.txt").unwrap()));

    let counter = Arc::new(Mutex::new(0));

    // for ordered results
    // let mut threads = Vec::new();

    let mut threads_map: HashMap<usize, JoinHandle<()>> = HashMap::new();

    let mut thread_sleeps: HashMap<usize, Duration> = HashMap::new();

    for e in 0..COUNT {
        // Setup Sleep the thread to randomize execution
        let mut rng = rand::thread_rng();
        let random_sleep = rng.gen_range(0..5);
        println!("setting up thread id:{}, sleep: {}", e, random_sleep);
        let sleep_duration = Duration::from_secs(random_sleep);
        thread_sleeps.insert(e, sleep_duration);
        println!("{:?}", thread_sleeps)
    }

    // make a clone of the configured sleeps
    let configured_thread_sleeps= Arc::new(Mutex::new(thread_sleeps));

    for t in 0..COUNT {
        // Use the ARC (Automatic Reference Counting boxing system). The variables will be aliased later.
        let counter = Arc::clone(&counter);
        let file = Arc::clone(&file);
        let configured_thread_sleeps = Arc::clone(&configured_thread_sleeps);

        let thread = thread::spawn(move || {

            // id
            let thread_index = t;

            // lock the counter and the file
            let mut i = counter.lock().unwrap();
            let configured_sleeps = configured_thread_sleeps.lock().unwrap();
            // make the thread sleep before acquiring the file lock
            // this could simulate some computation before storing values
            let this_thread_duration = configured_sleeps.get(&t).expect("Invalid Key");
            thread::sleep(*this_thread_duration);

            // lock on the aliased file variable
            let file = file.lock().unwrap();

            // Debug info
            if let _print_thread_and_sleep_on_debug = cfg!(debug_assertions) {
                println!("Scheduled thread {}, counter: {} which slept: {}", thread_index, i, this_thread_duration.as_secs())
            }

            // using metadata to get a 'Result' and unwrapping the length when OK.
            let file_metadata_result = file.metadata();
            let offset = match file_metadata_result {
                Ok(metadata) => metadata.len(),
                _ => 0 as u64
            };
            // format the message and add to the offset (manual append)
            let line = format!("i: {}, processed/slept: {}\n", i, this_thread_duration.as_secs());
            file.write_all_at(line.as_ref(), offset).unwrap();

            // mutate the counter
            *i += 1;
        });

        // threads.push(thread);
        threads_map.insert(t, thread);
    }

    // wait for all threads to finish
    // ordered
    /*
    for thread in threads {
        thread.join().unwrap();
    }
    */
    let mut thread_keys: Vec<usize> = Vec::new();
    for (k, v) in threads_map {
        thread_keys.push(k);
        v.join().unwrap();
    }
    println!("Traversed threads in this order :{:?}", thread_keys);

}
