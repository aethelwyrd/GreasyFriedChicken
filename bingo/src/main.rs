//rusty.rs
//Copyright 2016 David Huddle
//Create bingo cards 
//text file should look like
//1 2 3 4 5 16 17 18 19 20 31 32 00 34 35 46 47 48 49 50 65 66 67 68 69 70
//with one bingo card as above per line

extern crate rand;

use rand::{thread_rng, Rng};

type BigInt = i64;

fn main() {
    let ball_call = get_ball_call2();
    println!("****ball call");
    println!("{:?}", ball_call);
    println!("****bingo card");
    let bingo_card = get_bingo_card();
    for x in 0..5{
        println!("{:?}", bingo_card[x]);
    }

    let x:i64 = 12345;
    let y:BigInt = 5623456;

    let z = x + y;
}

fn get_ball_call() -> Vec<i32>{
    let mut rng = thread_rng();
    let mut ordered_balls:Vec<i32> = (1..76).collect();
    let ball_call: &mut [_] = &mut ordered_balls;
    rng.shuffle(ball_call);
    ball_call.to_vec()
}

fn get_ball_call2() -> Vec<i32>{
    let mut rng = thread_rng();
    let mut ordered_balls:Vec<i32> = (1..76).collect();
    rng.shuffle(&mut ordered_balls);
    ordered_balls.to_vec()
}

fn get_bingo_card_column(column:i32) -> Vec<i32>{
    let mut rng = thread_rng();
    let mut col = Vec::new();
    let mut balls:Vec<i32> = (1..15).collect();
    let mixed_balls: &mut [_] = &mut balls;
    rng.shuffle(mixed_balls);
    let mut mballs:Vec<i32> = mixed_balls.to_vec();

    let offset = column - 1;
    
    for x in 0..5{
        col.push(mballs.pop().expect("No numbers available for bingo card") + offset * 15);
    }
    col
}

fn get_bingo_card() -> Vec<Vec<i32>>{
    let mut card = Vec::new();
    for x in 1..6{
        card.push(get_bingo_card_column(x));
    }
    //Free space
    //card[2][2] = 0;
    card
}
