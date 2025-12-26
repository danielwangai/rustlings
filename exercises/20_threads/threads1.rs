// This program spawns multiple threads that each runs for at least 250ms, and
// each thread returns how much time it took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

use std::{
    sync::mpsc, thread, time::{Duration, Instant}
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        let result = handle.join().unwrap();
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}

fn threads() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Hi from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // force main to wait for the thread to finish executing
    handle.join().unwrap();

    for i in 1..5 {
        println!("Main thread: {i}");
    }
}

fn threads2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        // val is moved out of scope
        tx.send(val).unwrap();
        // attempt to use after free
        // println!("val is: {val}");
    });

    let received = rx.recv().unwrap();
    println!("Received: {received}");
}

fn threads3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // list of strings
        let vals = vec![
            String::from("One"),
            String::from("Two"),
            String::from("Three"),
            String::from("Four"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }

    });

    for received in rx {
        println!("Got: {received}");
    }
}
