use std::sync::{Arc, Mutex};
use std::{thread};
use std::collections::HashMap;
use std::thread::JoinHandle;
use std::time::Duration;
use rand::{Rng};
extern crate termion;

use termion::{color};

const COUNT: usize = 20;
const MAX_DELAY: usize = 5;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads_map: HashMap<usize, JoinHandle<()>> = HashMap::new();
    let mut thread_sleeps: HashMap<usize, Duration> = HashMap::new();

    for e in 0..COUNT {
        // Setup Sleep the thread to randomize concurrent thread execution
        let mut rng = rand::thread_rng();
        let random_sleep = rng.gen_range(0..MAX_DELAY);
        let message = format!("[Setting up] thread id:{}, sleep: {}", e, random_sleep);
        println!("{}{}", color::Fg(color::Blue), message);
        let sleep_duration = Duration::from_secs(random_sleep.try_into().unwrap());
        thread_sleeps.insert(e, sleep_duration);
    }

    // make a clone of the configured sleeps
    let configured_thread_sleeps = Arc::new(Mutex::new(thread_sleeps));

    for t in 0..COUNT {
        // Use the ARC (Automatic Reference Counting boxing system). The variables will be aliased later.
        let counter = Arc::clone(&counter);
        let configured_thread_sleeps = Arc::clone(&configured_thread_sleeps);

        let thread = thread::spawn(move || {
            do_counter_stuff(counter, t, configured_thread_sleeps);
        });

        threads_map.insert(t, thread);
    }

    let mut thread_keys: Vec<usize> = Vec::new();
    for (k, h) in threads_map {
        thread_keys.push(k);
        let loop_message = format!("[Main] Asking the thread {} to join.", k);
        println!("{}{}", color::Fg(color::Cyan), loop_message);
        h.join().unwrap();
    }
    let end_message = format!("[End] Asked the threads to join in this order :{:?}", thread_keys);
    println!("{}{}", color::Fg(color::Blue), end_message)
}

fn do_counter_stuff(counter: Arc<Mutex<i32>>, thread_index: usize, configured_thread_sleeps: Arc<Mutex<HashMap<usize, Duration>>>) {

    let message = format!("[Thread {}] starting!", thread_index);
    println!("{}{}", color::Fg(color::Green), message);
    let configured_sleeps = configured_thread_sleeps.lock().unwrap();
    // make the thread sleep
    // this could simulate some computation before storing values

    // lock the counter
    let mut i = counter.lock().unwrap();

    let this_thread_duration = configured_sleeps.get(&thread_index).expect("Invalid Key");
    thread::sleep(*this_thread_duration);

    let intermediate_message = format!("[Thread {}] counter 'i' is now :{}, processed/slept: {}\n", thread_index, i, this_thread_duration.as_secs());
    println!("{}{}", color::Fg(color::Yellow), intermediate_message);

    drop(configured_sleeps);

    // mutate the counter
    *i += 1;
    let finished_message = format!("[Thread {}] finished and incremented the counter to {}!", thread_index, i);
    println!("{}{}",color::Fg(color::Red), finished_message);
}
