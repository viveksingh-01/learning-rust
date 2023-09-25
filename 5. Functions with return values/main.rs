
//! FUNCTIONS WITH RETURN VALUES

//# Statements and Expressions
//% Rust is an expression-based language
//% Function bodies are made up of a series of statements optionally ending in an expression.

//> Statements - are instructions that perform some action and do not return a value.
//> Expressions - evaluate to a resultant value.


fn stmts_and_exprs() {
  //? Creating a variable and assigning a value to it with the let keyword is a statement.
  let z = 7;
  //* Expressions can be part of statements: the 7 in the statement let z = 7; is an expression that evaluates to the value 7. 
  
  //% Statements do not return values. Therefore, we can’t assign a let statement to another variable
  //% The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
  let x = (let y = 6);
  
  //* Calling a function, calling a macro, a new scope block created with curly brackets, all are expressions.
  //> In this case, the inner block evaluates to 6. That value gets bound to 'a' as part of the let statement. 
  //> Note that the b + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines we've seen so far. 
  let a = {
      let b = 5;
      b + 1
  };
  //# Expressions do not include ending semicolons. 
  //# If we add a semicolon to the end of an expression, we turn it into a statement, and it will then not return a value.
}

//_ Functions with return values 
//% Functions which can return values must declare their type after an arrow (->). 
//% In Rust, the return value of the function is synonymous with the value of the final expression in the block of the function's body. 
//> We can return early from a function by using the return keyword and specifying a value, 
//> but most functions return the last expression implicitly.
fn seven() -> i32 {
  7  //? the 7 in seven is the function’s return value, which is why the return type is i32
}

fn plus_one(x: i32) -> i32 {
  x + 1   //* Without semicolon, it's an expression, therefore returns value of x + 1
}

//> The definition of the function plus_five says that it will return an i32, 
//> but statements don’t evaluate to a value, which is expressed by (), the unit type. 
//> Therefore, nothing is returned, which contradicts the function definition and results in an error. 
fn plus_five(x: i32) -> i32 {
  x + 5;  //? With semicolon, it becomes a statement, therefore doesn't return
  //^ we get a compilation error - 'mismatched types - expected i32, found ()'
}

fn main() {
  stmts_and_exprs();

  //? We’re using the return value of a function to initialize a variable
  let c = seven(); 
  println!("The value of c is {c}");

  let b = plus_one(6);
  println!("The value of b is {b}");

  let a = plus_five(2);
  println!("The value of a is {a}")
}

