//my_other_mod.rs
//Copyright 2015 David Huddle

    pub fn used_only_in_test() -> i32{
        235
    }

    pub fn play_ground(bob:i32){
        println!("this is bob: {}", bob);
    }

    pub fn match_is_like_switch(x:i32, y:i32){
        match cmp(x,y){
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => println!("Equal"),
        }
    }

    pub fn console_out(){
        println!("lisard king");
    }

    pub fn cmp(x: i32, y: i32) -> Ordering {
        if x < y { Ordering::Less }
        else if x > y { Ordering::Greater }
        else { Ordering::Equal }
    }

    pub enum Ordering { Less, Greater, Equal }

    pub fn print_option(x: Option<i32>) {
        match x {
            Some(val) => println!("value is {}", val),
            None => {
                println!("cannot print null")
            }
        }
    }

    pub fn vector_mod_by_ref( nums: &mut Vec<i32>) {
        for x in nums {
            *x = *x + 1; 
        }
    }
     
    pub fn vector_interation_by_ref( nums: &Vec<i32>){
        for x in nums{
            println!("{} is in vec", x);
        }
     }

    pub fn loop_ex(){
        let mut a: i32 = 3i32;
        loop{
            a = a + 1i32;
            println!("loop{}", a);
            if a > 5i32 { break; }
        }
    }

    pub fn while_ex(){
        let mut z = 0i32;
        let mut done = false;
        while !done {
            println!("while{}", z);
            z = z + 1;
            if z == 5 { done = true; }
        }
    }

    pub fn for_in_is_for_each(){
        for x in 0i32..6i32{
            println!("for:{}", x);
        }
    }

