//main.rs
//Copyright 2015 David Huddle

#[test]
fn it_works() {
}

use std::thread;

fn main(){
    println!("i kinda work some of the time");

    let nums = 1..20;
    let handles: Vec<_> = nums.into_iter().map(|num|{
            let bob = num;
            thread::spawn(move || {
                    println!("roger wilco {}", bob);
                    do_work(bob);
                    println!("wtf, over{}", bob);
                })
            }).collect();
    for h in handles {
        h.join().unwrap();
    }
}

fn do_work(thread_no: i32){
    let work = 1..30;
    for w in work {
        println!("thread: {} work: {}", thread_no, w);
    }
}
