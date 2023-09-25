
//! FUNCTIONS

//% We define a function in Rust using "fn" keyword followed by a function name and a set of parentheses. 

//^ Rust code uses snake case as the conventional style for function and variable names, 
//^ in which all letters are lowercase and underscores separate words.
fn just_another_function() {
  println!("Just another function.");
}

//? One of the most important functions in the language: the main function is the entry point
fn main() {
  just_another_function();
  yet_another_function();
  print_temp(23, 'C');
}

//> Rust doesn’t care where we define our functions, 
//> only that they’re defined somewhere in a scope that can be seen by the caller.
fn yet_another_function() {
  println!("Yet another function");
}

//* By design, Rust requires us to declare the type of each parameter in function signatures.
fn print_temp(temp: i32, format: char) {
  println!("The temperature is {temp} {format}");
}