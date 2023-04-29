#![allow(unused)]
#![feature(repr128)]
#![feature(core_intrinsics)]

use std::intrinsics::const_eval_select;
use std::io;
use rand::{Rng, random};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn excercise_one() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you.";
    io::stdin().read_line(&mut name)
        .expect("Didnt Receive Input");
    
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn excercise_two() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn data_types() {
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
    let is_true = true;
    let my_grade = 'A';
}

fn number_accuracy() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
}

fn math_operators(){
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3  + num_4);
    println!("5 - 4 = {}", num_3  - num_4);
    println!("5 * 4 = {}", num_3  * num_4);
    println!("5 / 4 = {}", num_3  / num_4);
    println!("5 % 4 = {}", num_3  % num_4);
    num_3 += 1;
}

fn random_excercise() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
}

fn if_exercises() {
    let age = 32;
    if (age >= 1) && (age  <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }
}

fn ternary_example () {
    let mut my_age = 15;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can Vote : {}", can_vote);
}

fn match_example () {
    let age2: i32 = 32;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

   let my_age = 17;
   let voting_age = 18;
   match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote."),
   };

}

fn array_example () {
    let arr_1 = [1,2,3,4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let arr_2 = [1,2,3,4,5,6,7,8,9];

    let mut loop_idx: usize = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }
    loop_idx = 0;

    while loop_idx < arr_2.len() {
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}

fn main () {
    array_example();
}