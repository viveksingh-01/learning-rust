
//! VARIABLES and MUTABILITY

//% In Rust, the variables are immutable by default.
//% that means, once we give the variable a value, the value won’t change.

fn main() {
  //> This line creates a new variable named x and binds it to the value 5.
  let x = 5; 
  println!("The value of x is {x}");

  //^ On compilation, we get error - cannot assign twice to immutable variable `x` 
  //^ because we tried to assign a second value to the immutable x variable.
  x = 6;
  
  //% We can make the variable mutable by adding 'mut' in front of the variable name
  let mut y = 5;
  println!("The value of y is {y}");
  y = 6;
  println!("The value of y is {y}");


  //# CONSTANTS
  //% Like immutable variables, constants are values that are bound to a name and are 
  //% not allowed to change, but there are a few differences between constants and variables.
  //* 1.  We can declare constants using the 'const' keyword instead of the let keyword, 
  //*     and the type of the value must be annotated.
  //> 2.  We aren’t allowed to use mut with constants. 
  //>     Constants aren’t just immutable by default — they’re always immutable.
  //? 3.  Constants may be set only to a constant expression,
  //?     not the result of a value that could only be computed at runtime.
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

  //# SHADOWING
  //% We can declare a new variable with the same name as a previous variable.
  //% We can shadow a variable by using the same variable’s name and repeating the use of the let keyword.
  let z = 5;
  //> Create new variable z, taking the original value and adding 1 so the value of z is then 6
  let z = z + 1;
  {
      //> Shadow z and create a new variable, multiplying the previous value by 2 to give z a value of 12
      let z = z * 2;
      println!("The value of z in the inner scope is: {z}");
  }
  //> When that scope is over, the inner shadowing ends and z returns to being 6
  println!("The value of z is: {z}");

  //* Shadowing is different from marking a variable as mut because we’ll get a compile-time error if
  //* we accidentally try to reassign to the variable without using the let keyword.
  //> By using let, we can perform a few transformations on a value but have the variable be immutable 
  //> after those transformations have been completed.
  //? The other difference between mut and shadowing is that because we’re effectively creating a new variable when we 
  //? use the let keyword again, we can change the type of the value but reuse the same name.
  //? Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num
  let spaces = "   ";
  let spaces = spaces.len();
}