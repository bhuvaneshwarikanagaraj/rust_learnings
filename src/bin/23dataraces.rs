// use std::thread;

fn main() {
    let mut counter = 0;

    let thread1 = thread::spawn(|| {
        for _ in 0..1_000_000 {
            counter += 1;
        }
    });

    let thread2 = thread::spawn(|| {
        for _ in 0..1_000_000 {
            counter += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Final counter value: {}", counter);
}


use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..2 {
        let counter_ref = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1_000_000 {
                let mut num = counter_ref.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}