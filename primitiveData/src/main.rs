// Primitive data types
// int, float, bool, char

//Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes
// i8, i16, i32, i64, i28: Signed integers.       
// u8, u16, u32, u64, u128: Unsigned integers
// smaller the i# or the u# means smaller ranges of integers
// larger the i# or the u# means larger ranges of integers

// Signed: Positive and Negative Integer Values (eg, -3, -2, -1, 0, 1, 2, 3, 4, ...)
// Unsigned Positive Integer Values Only (eg. 0, 1, 2, 3, 4, ...) 
fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

// diff bet i32 (32 bits) and i64 (64 bits)
// range : 
// i32 - 2147483647
// i64 - 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
// ==========================================
// Floats [Floating Point Types]
// f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);
// Boolean Values: true, false
    let is_snowing: bool = true;
    println!("is it snowing ? {}", is_snowing);
//  Character Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
    

//  i32, i64, u32, u64, f32, f64, char, bool

    let number_one: i64 = 124;
    let number_two: i64 = 24;

    println!("Number one is {} and number two is {}", number_one, number_two);
    println!("{} + {} = {}", number_one, number_two, number_one+number_two)

}


