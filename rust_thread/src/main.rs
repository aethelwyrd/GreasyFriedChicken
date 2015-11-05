//main.rs
//Copyright 2015 David Huddle

pub mod thread_simple;
pub mod thread_share;

extern crate time;

/// Program to play with rust features and do learning
pub fn main(){
    println!("i kinda work some of the time");
 
    thread_share::thread_handle();
    thread_simple::do_thread(10, 30);
    thread_share::shared_data_example();

    let start = time::precise_time_ns();
    println!("start: {}", start);
    let mut bob = Vec::new();
    for i in 0..2048 {
        bob.push(i *7);
    }
    let test1 = time::precise_time_ns();

    let numbers: Vec<_> = (0..8).collect();
    let numbers = thread_share::do_calc(numbers);
    for x in numbers {
        println!("did it work {}", x);
    }
    let test2 = time::precise_time_ns();

    let time1 = test1 - start;
    let time2 = test2 - start;
    println!("time1: {}  time2: {}", time1, time2);
    println!("time1 is {} times faster", time2 / time1);

    for x in thread_share::simple_mpsc(){
        println!("last test: {}", x);
    }

}

