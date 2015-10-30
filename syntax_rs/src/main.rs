//rusty.rs
//Copyright 2015 David Huddle

mod my_module;
mod main_mod;
mod test;

use my_other_mod::*;

fn main() {
    console_out();
    match_is_like_switch(2,5);
    for_in_is_for_each();
    while_ex();
    loop_ex();
    //vector
    // declare as ref so we can allow borrowing
    let ref nums = vec![1i32, 2i32, 3i32];
    let othernums = vec!{2i32, 1i32, 3i32};
    if *nums == othernums { println!("vector: equal"); }
    else { println!("vector: not equal"); }
    //nums is borrowed here
    vector_interation_by_ref(nums);
    //nums is still valid because ref
    //vector random access
    let element = nums[1];
    println!("{} is the 2nd element in vec", element);
    //Box (pointer) othernums and make it ref mut
    //also othernumbs is now out of scope
    let ref mut mut_vec = Box::new(othernums);
    //mut_vec is borrowed and mut
    vector_mod_by_ref(mut_vec);
    //mut_vec is borrowed not mut
    vector_interation_by_ref(mut_vec);
    //nullable type
    let notnull = Some(28i32);
    let amnull = None;
    print_option(notnull);
    print_option(amnull);
    my_module::print_mod();
    let firstpair = my_module::Pair{ first:23i32, second:67i32 };
    println!("first: {}, second: {}", firstpair.first, firstpair.second);
    let secondpair = my_module::generic_swap(firstpair);
    println!("first: {}, second: {}", secondpair.first, secondpair.second);
    my_module::range_matching(0.25);
    my_module::range_matching(0.5);
    my_module::range_matching(1.5);
    my_module::tuple_struct();

    //play_ground
    let ref mut myvar = 45i32;
    *myvar = 22i32;
    let myothervar = &myvar;
    play_ground(**myothervar);
//    play_ground(myothervar);
}

fn play_ground(bob:i32){
    println!("this is bob: {}", bob);
}

#[test]
fn locale_test(){
    #[should_panic]
    assert(false);
}
