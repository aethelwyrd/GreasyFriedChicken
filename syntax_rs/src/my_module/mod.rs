//my_module.rs
//Copyright 2015 David Huddle
//module example
//local module has the same name
//or
//module has same name as folder with a mod.rs

pub fn print_mod(){
    println!("I'm in a module!");
}

pub struct Pair<T>{
    pub first: T,
    pub second: T,
}

pub fn generic_swap<T>(pair: Pair<T>) -> Pair<T> {
    //destruct pair into components
    let Pair {first, second} = pair;
    //construct new struct 
    //implied return
    Pair{ first: second, second: first }
}

pub fn range_matching(x: f32){
    match x {
        n @ 0.0 ... 0.5 => println!("range match 0 to 0.5 : {}", n),
        n @ 0.5 ... 1.0 => println!("range match 0.5 to 1.0 : {}", n),
        n => println!("number out of scope : {}", n),
    }
}

//field default to private 
//usable only in the current module
pub struct FieldWithNoName(i32,);

//make methods for struct
impl FieldWithNoName {
    fn value(&self) -> i32 {
        let &FieldWithNoName( val ) = self;
        val
    }
}

pub fn tuple_struct(){
    let mystruct = FieldWithNoName ( 24 );
    println!("mystruct has a value of {}", mystruct.value());
}

pub fn iterator_example(){
    let mut range = 0..10;
    loop{
        match range.next(){
            Some(x) => {
                println!("iterating {} ...", x);
            },
            None => {
                println!("interation complete");
                break;
            }
        }
    }

    let nums = vec![423,4986,214];
    for num in &nums{
        println!("interating vec {}", num);
    }
    //find first element
    //also find returns an option which looks like it has iterator-ish powers
    let nums = (1..47).find(|x| *x > 42);
    for num in &nums{
        println!("interator magic bigger than 42 {}", num);
    }
    let nums2 = (1..47).find(|x| *x > 50);
    for num in &nums2{
        println!("interator magic bigger than 50 {}", num);
    }
    for i in (1..100).filter(|&x| x > 95 ).collect::<Vec<_>>(){
        println!("map an interator {}", i);
    }
    //map returns a new modified iterator
    let nums3 = 50..55;
    let nums4 = nums3.map(|x| x -50);
    for num in nums4{println!("maped some stuff {}", num);}
    //chain some iterators - first five numbers divisible by 2 and 3
    let nums5 = (1..).filter(|&x| x % 2 == 0)
                     .filter(|&x| x % 3 == 0)
                     .take(5)
                     .collect::<Vec<_>>();
    for num in nums5 { println!("chained {}", num);}
}

pub fn fizz_buzz2(nums:Vec<i32>){
    for num in nums {
        let ret = match num % 15{
            0 =>{"fizzbuzz"}
            _ =>{
                match num % 5{
                    0 => {"buzz"}
                   _ => {
                       match num % 3 {
                           0 => {"fizz"}
                           //TODO make a type that can hold and print
                           //fizz, buzz, fizzbuzz, and i32
                           //_ => {num}
                           _ => {""}
                       }
                   }
                }
            }
        };
        println!("ret {}", ret);
    }
}

pub fn fizz_buzz(nums:Vec<i32>){
    for num in nums {
        match (num %3 == 0, num % 5 == 0) {
            (true, true) =>{println!("fizzbuzz");}
            (true, false) =>{println!("fizz");}
            (false, true) =>{println!("buzz");}
            (false, false) =>{println!("{}", num);}
        }
    }
}

