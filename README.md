# LEARN RUST FROM FREE-CODE-CAMP

### INITIALIZATION

1. First I was click this link https://github.com/freeCodeCamp/Rust-In-Replit/ then go down and click "Made With Replit".
2. After click, create account on replit and choose "Bash" language from language drop down.
3. Then click the "Import from GitHub" button in the lower right to import the boilerplate code into Replit.
4. Finally, to start the course, click the Run button at the top of the screen and follow the instructions in the console on the right.

## Basics of Rust

### Variables in Rust

- You can declare variables using the "let", "const", or "static" keywords:

```
let my_variable = 0;
const MY_CONSTANT: u8 = 0;
static MY_STATIC: u8 = 0;
```

- By default, all variables are immutable. You can make a variable mutable by using the "mut" keyword:

```
let mut my_mutable_variable = 0;
```

### Rust convention relies on the following casing conventions:

```
OBJECT	      CASING
Variables     snake_case
Functions     snake_case
Files	        snake_case
Constants     SCREAMING_SNAKE_CASE
Statics	      SCREAMING_SNAKE_CASE
Types	        PascalCase
Traits	      PascalCase
Enums	        PascalCase
```

- Since Rust is statically typed, you'll need to explicitly type variables – unless the variable is declared with "let" and the type can be inferred.

### Functions in Rust

- You declare functions using the fn keyword:

```
fn first_function() {
  // this is function body
}
```

- Functions return using the "return" keyword, and you need to explicitly specify the return type of a function, unless the return type is an empty tuple "()":

```
fn second_function() -> () { // Unnecessary return type
  first_function();
}

fn third_function() -> u8 {
return 0;
}
```

- Functions also return an expression missing the semi-colon:

```
fn fourth_function() -> u8 {
  0
}
```

- Function parameters are typed using the : syntax:

```
fn main() {
  let unused_variable = fourth_function(10);
}
```

```
fn my_fuc( x: u8) -> i32 {
  x as i32
}
```

- The underscore before a variable name is a convention to indicate that the variable is unused. The as keyword asserts the type of the expression, provided the type conversion is valid.

### Strings and Slices in Rust

- A common point of confusion for beginner Rustacians is the difference between the String struct and the str type.

```
let my_str: str& = "Hello World";
let my_string: String = String:from("Hello World");
```

- In the above example, my_str is a reference to a string literal, and my_string is an instance of the String struct.
- An important distinction between the two is that my_str is stack stored, and my_string is heap allocated. This means my_str's value cannot change, and its size is fixed, whilst my_string can have an unknown size at compile time.
- The string literal is also known as a string slice. This is because a &str refers to part of a string. Generally, this is how arrays and strings are similar:

```
let my_string = String::from("The quick brown fox");
let my_str: &str = &my_string[4..9]; //quick

let my_arr: [usize; 5] = [1, 2, 3, 4, 5];
let my_arr_slice: &[usize] = &my_arr[0..3]; // [1, 2, 3]
```

- The [T; n] notation is used to create an array of n elements of type T.

### The char Type in Rust

- A char is a USV (Unicode Scalar Value), which is represented in unicode with values like U+221E – the unicode for '∞'. You can think of a collection or array of chars as a string:

```
let my_str: &str = "Hello World";
let collection_of_char: &str = my_str.chars().as_str();
```

### Number Types in Rust

- There are many types of numbers in Rust:

```
1. Unsigned integers: u8, u16, u32, u64, u128
2. Signed integers: i8, i16, i32, i64, i128
3. Floating numbers: f32, f64
```

- Unsigned integers only represent positive whole numbers.
- Signed integers represent both positive and negative whole numbers.
- And floats only represent positive and negative fractions.

### Structs in Rust

- A struct is a custom data type used to group related data. You have already come across a struct in the Strings and Slices section:

```
struct String {
  vec: Vec<u8>,
}
```

- The String struct consists of a vec field, which is a Vec of u8s. The Vec is a dynamically-sized array.

- An instance of a struct is then declared by giving values to the fields:

```
stuct myStruct {
  field_1: u8,
}

let my_struct = myStruct {field_1: 0,};
```

- Previously, the String struct was used with its from function to create a String from a &str. This is possible, because the from function is implemented for String:

```
impl String {
  fn from(s: &str) -> Self {
    String {
      vec: Vec::from(s.as_bytes()),
    }
  }
}
```

- You use the Self keyword in place of the type of the struct.

- Structs can also take other variants:

```
struct MyUintStruct;
struct MyTupleStruct(u8, u8);
```

### Enums in Rust

- Similar to other languages, enums are useful for acting as types and as values.

```
enum MyErrors {
  BrainTooTired,
  TimeOfDay(String),
  CofeeCupEmpty,
}

fn work() -> Result<(), MyErrors> { // Result is also an enum
  if state == "missing semi-colon" {
    Err(MyError::BrainTooTired)
  }else if state == "06:00" {
    Err(MyError::TimeOfDay("It's too early to work".to_String()))
  }else if state == "22:00" {
    Err(MyError::TimeOfDay("It's too late to work".to_String()))
  }else if state == "empty" {
    Err(MyError::CofeeCupEmpty)
  }else {
    OK(())
  }
}
```

### Macros in Rust

- A macro is similar to a function, but you can think of it as a piece of code which writes other code. For now, the main differences between a function and a macro to keep in mind are:
- Macros are called using a bang (!)
- Macros can take a variable number of arguments, while functions in Rust cannot
- One of the most common macros is the println! macro, which prints to the console:

```
let my_str = "Hello World";
println!("{}", my_str);
```

- You use the {} syntax to insert a variable into a string.

- Another common macro is panic!. Panicking is Rust's way of 'erroring out'. It is wise to think of a panic in Rust as a poorly-handled error. The macro accepts a string literal, and panics with that message.

```
let i_am_error = true;

if(i_am_error){
  panic!("There was an error");
}
```

### Ownership in Rust

- An important concept in Rust is ownership. There are three main ownership rules:

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

```
fn main() { // first_string is not declared yet -> has no value
  let first_string = String::from("free code camp");  // first_string is now owner of the value "freeCodeCamp"
  let second_string = first_string; // second_string takes ownership of the value "freeCodeCamp"

  println!("Hello, {}!", first_string); // first_string is NOT valid, because the value was moved to second_string
}
```

- As the println! macro tries to refer to an invalid variable, this code does not compile. To fix this, instead of moving the value of first_string into second_string, second_string can be assigned a reference to first_string:

```
fn main() {
  let first_string: String = String::from("freeCodeCamp);
  let second_string: &String = &first_string; // first_string is still the owner of the value "freeCodeCamp"

  println!("{}", first_string);
}
```

- The ampersand (&) indicates that the value is a reference. That is, second_string no longer takes ownership of "freeCodeCamp", but, instead, points to the same point in memory as first_string.

## Project #1 – Build a CLI Calculator in Rust

### Project Outcome

- At the end of this project, you will be able to perform basic arithmetic operations on numbers using the command line.

- Examples of expected input and output look like this:

```
$ calculator 1 + 1
$ 1 + 1 = 2

$ calculator 138 / 4
$ 138 / 4 = 34.5
```

## CLI Calculator Project Methodology

### Step 1 – Create a New Project

- Use Cargo to create a new project named calculator:

```
$ cargo new calculator
```

- This creates a new directory named calculator, initialises it as a Git repository, and adds useful boilerplate for your project.

- The boilerplate includes:

1. "Cargo.toml" – The manifest file used by Cargo to manage your project's metadata
2. "src/" – The directory where your project code should live
3. "src/main.rs" – The default file Cargo uses as your application entrypoint

### Step 2 – Understand the Syntax

- The calculator/Cargo.toml file contains the following:

```
[package]
name = "calculator"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- The "package" denotes your project's metadata.

- The "dependencies" heading denotes the crates your project depends on. Crates are like external libraries.

- The calculator/src/main.rs file contains the following:

```
fn main() {
  println!("Hello world!");
}
```

- This file contains a function declaration with the handle main. By default, rustc calls the main function first whenever the executable is run.

- println! is a built-in macro which prints to the console.

### Step 3 – Run the Project

- You can either use Cargo to run your project code:

```
# Within the calculator/ directory
$ cargo run
   Compiling fcc-rust-in-replit v0.1.0 (/ / /Rust-in-Replit-1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/calculator`
Hello, world!
```

- Or, you can use rustc to compile your project, then you can run the binary:

```
# Within the calculator/ directory
$ rustc src/main.rs
$ ./main
Hello, world!
```

### Step 4 – Command Line Arguments

- The Rust standard library comes with an env module, which allows access to the command line arguments passed when calling the program.

- The necessary exports from the env module are the args function, and the Args struct. The args function returns an instance of the Args struct, and is imported into the file scope with:

```
use std::env::{args, Args};
```

- To get an idea of what the Args struct looks like, the args variable is printed to the console:

```
fn main() {
  let args: Args = args();
  println!("{:?}", args);
}
```

```
$ cargo run -- fCC
   Compiling calculator v0.1.0 (/home/runner/Rust-in-Replit/calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 1.71s
     Running `target/debug/calculator`
Args { inner: ["target/debug/toto", "fCC"] }
```

- The above shows that the Args struct contains a field called inner which consists of the location of the compiled binary, and the command line arguments passed to the program.

- To access the argument values, you can use the nth method on the args variable. The nth method takes an index argument, and returns the value at that index wrapped in an Option. So, the value needs to be unwrapped.

```
fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
}
```

- The args variable needs to be declared as mutable, because the nth method mutable iterates over the elements, and removes the element accessed.

```
fn main() {
  let mut args: Args = args();

  // The first argument is the location of the compiled binary, so skip it
  let first: String = args.nth(1).unwrap();
  // After accessing the second argument, the iterator's next element becomes the first
  let operator: String = args.nth(0).unwrap();
  let second: String = args.nth(0).unwrap();

  println!("{} {} {}", first, operator, second);
}
```

```
$ cargo run -- 1 + 1
   Compiling calculator v0.1.0 (/home/runner/Rust-in-Replit/calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 1.71s
     Running `target/debug/calculator`
1 + 1
```

### Step 5 – Parse Strings into Numbers

- The first and second variables are strings, and you need to parse them into numbers. The String struct implements the parse method, which takes a type annotation, and returns a Result containing the parsed value.

```
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second: String = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();

  println!("{} {} {}",first_number, operator, second_number);
}
```

- The above parse method uses the turbofish syntax to specify the type to try to parse the string into.

### Step 6 – Perform Basic Arithmetic Operations

- Rust uses the standard operators for addition, subtraction, multiplication, and division.

- To handle the operations, you define a function named operate which will take three arguments: the operator as a char, and the two numbers as f32s. The function should also return an f32 representing the outcome of the operation.

```
fn operator(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator{
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'x' | 'X' => first_number * second_number,
    _ => panic!("Invalid operator used"),
  }
}
```

- The match expression works similarly to a switch statement in other languages. The match expression takes a value, and a list of arms. Each arm is a pattern and block. The pattern is a value to match against, and the block is the code to execute if the pattern matches. The _ pattern is a wildcard, acting like an else clause.

- The multiplication arm includes the OR comparison to allow cases for X and x to be handled.

- Now, to call operate with the operator, you need to converted it into a char first. You do this with the chars method on the String struct which returns an iterator over the characters in the string. Then, the first character is unwrapped:

```
fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second: String = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  let result = operator(operator, first_number, second_number);

  println!("{} {} {}", first_number, operator, second_number);
}
```

- The return of operate is stored in the result variable.

