//main.rs
//Copyright 2015 David Huddle

pub mod thread_simple;
pub mod thread_share;

/// Program to play with rust features and do learning
pub fn main(){
    println!("i kinda work some of the time");
 
    thread_simple::do_thread(10, 30);
    thread_share::shared_data_example(10,30);
    thread_share::shared_data_example2();
}

