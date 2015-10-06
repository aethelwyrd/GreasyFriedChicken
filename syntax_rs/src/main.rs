//rusty.rs

mod my_module;

fn main() {
    console_out();
    //switch, if, struct
    let x = 4i32;
    let y = 2i32;
    match cmp(x,y){
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal"),
    }
    for_ex();
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
}

fn console_out(){
    println!("lisard king");
}

fn cmp(x: i32, y: i32) -> Ordering {
    if x < y { Ordering::Less }
    else if x > y { Ordering::Greater }
    else { Ordering::Equal }
}

enum Ordering { Less, Greater, Equal }

fn print_option(x: Option<i32>) {
    match x {
        Some(val) => println!("value is {}", val),
        None => {
            println!("cannot print null")
        }
    }
}

fn vector_mod_by_ref( nums: &mut Vec<i32>) {
    for x in nums {
        *x = *x + 1; 
    }
}
 
fn vector_interation_by_ref( nums: &Vec<i32>){
    for x in nums{
        println!("{} is in vec", x);
    }
 }

fn loop_ex(){
    let mut a: i32 = 3i32;
    loop{
        a = a + 1i32;
        println!("loop{}", a);
        if a > 5i32 { break; }
    }
}

fn while_ex(){
    let mut z = 0i32;
    let mut done = false;
    while !done {
        println!("while{}", z);
        z = z + 1;
        if z == 5 { done = true; }
    }
}

fn for_ex(){
    for x in 0i32..6i32{
        println!("for:{}", x);
    }
}

