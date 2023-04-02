fn main() {
    // scalar and compound data types
    // rust is a statically typed language, which means that it must know the types of all variables at compile time

    let guess: u32 = "32".parse().expect("Not a number!");

    /* SCALAR TYPES */

    // integers, floating-point numbers, booleans, and characters

    // integers can be signed or unsigned
    // signed integers can be positive or negative

    // integer literals can be written as decimal, hexadecimal, octal, or binary
    // default integer type is i32
    // isize and usize depend on the kind of computer your program is running on: 
    // 64 bits if you’re on a 64-bit architecture and 32 bits with a 32-bit architecture

    // floating-point types are f32 and f64
    let a = 2.0; // f64
    let b: f32 = 3.0; // f32

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    // characters
    let c = 'z'; // char type is four bytes in size and represents a Unicode Scalar Value, meaning it can represent a lot more than just ASCII
    let z: char = 'ℤ'; // with explicit type annotation
    let rainbow = '🌈'; // with explicit type annotation

    /* COMPOUND TYPES */

    // tuples
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with explicit type annotation
    let (x, y, z) = tup; // destructuring
    println!("print tuple: {x}");

    // accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // arrays must have the same type
    let arr = [1,2,3,4,5]; 
    let arr: [i32; 5] = [1,2,3,4,5]; // with explicit type annotation
    let arr = [3; 5]; // [3,3,3,3,3]

    // accessing array elements
    let first = arr[0];
    let second = arr[1];
    
}
