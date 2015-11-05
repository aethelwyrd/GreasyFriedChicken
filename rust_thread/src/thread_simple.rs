//thread_simple.rs
//Copyright 2015 David Huddle

use std::thread;

/// Spawns num_threads threads. Each thread counts to work_items on the console
/// 
/// # Examples 
/// //Spawns five threads that each count to 10
///
/// do_thread(5,10);
pub fn do_thread(num_threads:i32, work_items: i32){
    let nums = 1..num_threads;
    let handles: Vec<_> = nums.into_iter().map(|num|{
        thread::spawn(move || {
            println!("roger wilco {}", num);
            do_work(num, work_items);
            println!("wtf, over{}", num);
        })
    }).collect();
    for h in handles {
        h.join().unwrap();
    }
}

/// Prints work_items number of console statements
/// 
/// # Examples 
/// do_work(5,10);
fn do_work(thread_no: i32, work_items: i32){
    let work = 1..work_items;
    for w in work {
        println!("thread: {} work: {}", thread_no, w);
        thread::sleep_ms(10);
    }
}

