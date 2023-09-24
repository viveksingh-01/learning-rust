
//! COMPOUND DATA TYPES

//% Compound types can group multiple values into one type. 
//% Rust has two primitive compound types: tuples and arrays.

//# TUPLES
//% A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
//% Tuples have a fixed length: once declared, they cannot grow or shrink in size.

//^ We can create a tuple by writing a comma-separated list of values inside parentheses. 
//^ Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

//> The tuple without any values has a special name, "unit". 
//> This value and its corresponding type are both written () and represent an empty value or an empty return type. 
//> Expressions implicitly return the unit value if they don’t return any other value.


//# ARRAY
//% An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. 
//% Unlike a tuple, every element of an array must have the same type. 
//% Unlike arrays in some other languages, arrays in Rust have a fixed length.

//^ Arrays are useful when we want our data allocated on the stack rather than the heap.
//^ Or when we want to ensure we always have a fixed number of elements.

fn main() {
  //# Tuple with multiple data-types
  let tup: (u16, f64, i32, char, bool) = (155, 3.14, -3800, 'z', true);

  //? To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
  let (a, b, c, d, e) = tup;
  println!("{a}, {b}, {c}, {d}, {e}");

  //* We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
  let _pi = tup.1;
  let _its_sunny_today = tup.4;

  //# Array
  //> We write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array
  let arr: [i32; 5] = [1, 2, 3, 4, 5];

  //* We can also initialize an array to contain the same value for each element by specifying the initial value, 
  //* followed by a semicolon, and then the length of the array in square brackets
  let _same_val_arr: [i32; 5] = [3; 5]; //? this is same as writing let _same_val_arr = [3, 3, 3, 3, 3]

  //> We can access elements of an array using indexing
  let _first = arr[0];
  let _second = arr[1];
}