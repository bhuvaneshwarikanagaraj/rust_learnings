use std::thread;

fn main() {
    let thread1 = thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 1: Hello!");
        }
    });

    let thread2 = thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2: Hi!");
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}