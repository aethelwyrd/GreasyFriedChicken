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
