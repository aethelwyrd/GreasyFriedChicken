//thread_share.rs
//Copyright 2015 David Huddle

use std::thread;
use std::sync::{Arc,Mutex};

/// Takes some data makes four copies and modifies each copy in a thread
///
///# Examples 
/// shared_data_example(100,5);
pub fn shared_data_example(max_data:i32, thread_count:i32){
    //this takes some data makes four copies and modifies each in a thread
    println!("shared_data_example");
    let data = Arc::new(Mutex::new(Vec::new()));
    for i in 1..max_data{
        data.lock().unwrap().push(i);
    }

    for i in 0..thread_count {
        let data = data.clone();
        println!("thread: {} ",i);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            //TODO make the max_data work 
            //let count:i32 = max_data - 1;
            for x in (0..10){
                data[x] += i*10;
                println!("thread: {} data: {}",i, data[x]);
                thread::sleep_ms(10);
            }
        });
    }

    thread::sleep_ms(5000);
//TODO be nice to use data after we modified it
//    let x = data.unwrap().unwrap();
//    for i in x {
//        println!("shared data: {}", i);
//    }
}

/// Takes some data makes four copies and modifies each copy in a thread
///
///# Examples 
pub fn shared_data_example2(){
    //never do it this way thread::sleep is there to ensure all thread complete
    //clearly not a good idea
    let numbers: Vec<_> = (5..15u32).collect();

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

