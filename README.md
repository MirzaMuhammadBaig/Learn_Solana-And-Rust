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
OBJECT	     CASING
Variables	snake_case
Functions	snake_case
Files	    snake_case
Constants	SCREAMING_SNAKE_CASE
Statics	    SCREAMING_SNAKE_CASE
Types	    PascalCase
Traits	    PascalCase
Enums	    PascalCase
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
