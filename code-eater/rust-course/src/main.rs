use rand::{self, Rng}; // import library / crate
use std::{cmp::Ordering, io};

fn main() {
    let num = 1; // num is mutable

    const PIE: f32 = 3.14; // const is faster than let

    let x = 10;
    let y = 6;
    let z = 10 + 6;

    println!("{}", num);
    println!("{}", PIE * 10.0 * 10.0);
    println!("x = {} and y = {}", x, y);
    println!("The result value is {}", z);
    add(6, 8);
    println!("{}", sub(2, 3, 4));
    println!("{}", integer_data_type());
    println!("{}", bool_data_type());
    println!("{}", char_data_type());
    println!("{}", floating_data_type());
    array_data_type();
    tuple_data_type();
    if_else();
    _loop();
    take_input();
    create_random_number();
    println!("Hello, world!");
    println!("/////////////////////\n/////////////////////\n/////////////////////\n");
    guessing_game();
}

fn add(a: u32, b: u32) {
    // let a = 6;
    // let b = 8;
    println!("Multiple of 6 and 8 is {}", a * b);
}

fn sub(a: i32, b: i32, c: i32) -> i32 {
    // return a - b - c; // you can return this syntax
    a - b - c // you can also return this syntax
}

///////////////////////////////////////////// Data Types:

//////////////////// Scalar data types:
// 1. Integer
// 2. Boolean
// 3. Char
// 4. Floating Point Number

fn integer_data_type() -> i32 {
    let a: i32 = 16 - 32;
    a
}

fn bool_data_type() -> bool {
    let a: bool = true;
    a
}

fn char_data_type() -> char {
    let a: char = 'a';
    a
}

fn floating_data_type() -> f32 {
    let a: f32 = 10.111;
    a
}

///////////// Compund data types
// 1. Array
// 2. Tuple

fn array_data_type() {
    let mut arr = [0, 1, 2, 3, 4, 5, 6, 7];
    arr[1] = 11;
    println!("{}", arr[1]);
}

fn tuple_data_type() {
    let mut tup = (7, 6.8, 'm', "u", true);
    println!("{}", tup.4);
    tup.4 = false;
    println!("{}", tup.4);
}

/////////////////////////////////// Flow control
// 1. if else
// 2. loop

fn if_else() {
    let mut a: bool = true;

    println!("{}", a);

    if a == true {
        println!("a is true");
        a = false
    } else {
        println!("a is false")
    }

    println!("a is {}", a);
}

fn _loop() {
    let mut counter: u32 = 0;
    loop {
        println!("{}", counter);
        counter += 1;
        if counter == 7 {
            break;
        }
    }
}

//////////////////////// use librry
fn take_input() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin()
        .read_line(&mut buffer) // user input
        .expect("Failed to read line"); // handle possible error
    println!("Buffer is {}", buffer);
}

fn create_random_number() {
    // let num: f32 = rand::random(); // u can also use this syntax
    let num = rand::random::<f32>();
    println!("Random number is {}", num);
}

/////////////////////// guessing game

fn guessing_game() {
    println!("Welcome to the guessing game");
    let secret_number = rand::thread_rng().gen_range(1..101); // 1 TO 100
    println!("Secret number is : {}", secret_number);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Your guess is : {}", guess);
        let guess: u32 = guess.trim().parse().expect("Type an integer");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
