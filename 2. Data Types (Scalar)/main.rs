
//! DATA TYPES

//% Rust is a statically typed language, which means that it must know the types of all variables at compile time.

//# SCALAR TYPES
//% A scalar type represents a single value.
//% Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

//# INTEGERS
//^ Length	Signed	Unsigned
//> 8-bit	  i8	    u8
//* 16-bit	i16	    u16
//> 32-bit	i32	    u32
//* 64-bit	i64	    u64
//> 128-bit	i128	  u128
//* arch	  isize	  usize

//? Signed and unsigned refer to whether it’s possible for the number to be negative.
//> integer types default to i32

//# FLOATING-POINT NUMBERS
//% Floating-point types in Rust are f32 and f64, which are 32 bits and 64 bits in size, respectively. 
//% The default type is f64.

//# BOOLEAN TYPES
//% two possible values: true and false. Booleans are one byte in size. 
//% The Boolean type in Rust is specified using 'bool'.


//# CHARACTER TYPE
//% Specified with char literals with single quotes, as opposed to string literals, which use double quotes. 
//% Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it 
//% can represent a lot more than just ASCII. 

fn main() {
  //* Unsigned Integer
  let x: u32 = 108;
  
  //> Signed Integer
  let y: i32 = -273;

  //* Floating-point number
  let pi: f64 = 3.14;

  //> Boolean
  let is_raining = true;

  //* Character
  let z: char = 'c';
}

