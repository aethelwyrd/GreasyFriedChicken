//thread_share.rs
//Copyright 2015 David Huddle

use std::thread;
use std::sync::{Arc,mpsc};

extern crate time;

/// takes a vector and modifies in on a different thread
pub fn do_amazing_things(data:Vec<i32>)->Vec<i32>{
    let (tx, rx) = mpsc::channel();
    let tx = tx.clone();
    thread::spawn(move || {
        let mut ret = Vec::new();
        for x in data {
            ret.push(x * 7);
        }
        tx.send(ret);
    });

    rx.recv().ok().expect("Could not receive answer")
}

/// Takes a vec and breaks it up to do calculations
pub fn do_calc(data: Vec<i32>)->Vec<i32>{
    let mut package = vec![data];
    let start = time::precise_time_ns();
    for _ in 0..2 {
        package = break_vec(package);
    }
    let stop =  time::precise_time_ns();
    println!("split time: {}", stop - start);

    let count = package.len();
    let (tx, rx) = mpsc::channel();
    for vec in package {
        let tx = tx.clone();
        let data_t = vec;
        thread::spawn(move || {
            let mut ret = Vec::new();
            for x in data_t {
                ret.push(x * 7);
            }
            tx.send(ret);
        });
    }

    let mut ret:Vec<i32> = Vec::new();
    for _ in 0..count {
        let mut source = rx.recv().ok().expect("Could not receive answer");
        ret.append(&mut source);
    }
    ret
}

/// Takes the vectors inside a vector and splits them in half
/// No checking is done to miss rounding errors
fn break_vec(data: Vec<Vec<i32>>)->Vec<Vec<i32>>{
    let mut ret: Vec<Vec<i32>> = Vec::new();
    for mut vec in data {
        let size = vec.len()/2;
        let vec1 = vec.split_off(size);
        ret.push(vec);
        ret.push(vec1);
    }
    ret
}

/// Takes some data makes four copies and modifies each copy in a thread
///
///# Examples 
pub fn shared_data_example(){
    //never do it this way thread::sleep is there to ensure all thread complete
    //clearly not a good idea
    let numbers: Vec<_> = (5..15i32).collect();

    let shared_numbers = Arc::new(numbers);

    for thread_no in 0..3 {
        println!("threadno: {}", thread_no);
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &child_numbers[..];
            // Work with the local numbers
            for x in local_numbers {
                let x = x+1;
                println!("threadno: {} mod data: {}", thread_no, x);
            }
        });
    }
    thread::sleep_ms(1000);
}

/// Simple example of multi provider single consumer
pub fn simple_mpsc()->Vec<i32>{
    let (tx, rx) = mpsc::channel();
    for _ in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let answer = 42;
            tx.send(answer);
        });
    }
    let mut ret:Vec<i32> = Vec::new();
    for _ in 0..10 {
        ret.push(rx.recv().ok().expect("Could not receive answer"));
    }
    ret
}

/// Simple example of spawning a thread and rejoining
/// Also handles a panic during thread execution for graceful recovery
pub fn thread_handle(){
    use std::thread;
    let handle = thread::spawn(move || {
        panic!("oops!");
    });
    let result = handle.join();
    assert!(result.is_err());
}
