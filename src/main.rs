// Just to hide warnings, don't use in production
#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Read name from command line, and greet

    /*
    // Reading input, printing basic stuff
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello {}! {}", name.trim_end(), greeting);
    */

    /*
    // Constants, mutable and immutable variables
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592753589;
    let age = "41";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
    */

    /*
    // Max values for types
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
    */
    
    /*
    // Types
    let is_true = true; //boolean
    let my_grade = 'A'; //char

    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
    num_3 += 1;
    println!("{}", num_3);
    */

    /*
    // Random numbers
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
    */

    /*
    // If-else statement
    let age: i32 = 66;
    if age >= 1 && age <= 18 {
        println!("Important Birthday");
    } else if age == 21 || age == 50 {
        println!("(also) Important Birthday");
    } else if age >= 65 {
        println!("(yet another) Important Birthday");
    } else {
        println!("Not so Important Birthday");
    }
    */
    
    /*
    // (almost) Ternary operator
    let my_age = 42;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can Vote: {}", can_vote);
    */

    /*
    // Match statement
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        64..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not (so) Important Birthday"),
    };
    */

    /*    
    // March plus the Ordering library
    let my_age = 18;
    let voting_age = 18;
    
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
    */

    /*
    // Simple array
    let arr_1 = [1, 2, 3, 4, 5];
    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());
    */

    /*    
    // Looping through an array, to print odd numbers below 9
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("[Loop] Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // While iteration over array
    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("[While] Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
    
    // For iteration over array
    loop_idx = 0;
    for val in arr_2.iter() {
        println!("[For] Val: {}", val);
    }
    */

    /*
    // Tuples
    let my_tuple: (u8, String, f64) = (42, "Marcin".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
    */

    /*
    // Working with Strings
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    
    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("m a a r c i i n");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();

    // reusing st6 - seems strange to me
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.bytes() {
        println!("{}", char);
    }
    */

    // Casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    //Enums
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true, 
                _ => false
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Mondays"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is it weekend yet? {}", today.is_weekend());
}
