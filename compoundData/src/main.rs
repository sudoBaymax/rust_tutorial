// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

// Arrays
fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    // let mix = [1,2,"apple",true];                 Can't mix different data types in an array
    // println!("Mix Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {:?}", fruits[0]);
    println!("Fruits Array: {:?}", fruits[1]);
    println!("Fruits Array: {:?}", fruits[2]);
    // ///////////////////////////////////////

    // Tuples
    // let human: (String, i32, bool) = ("Alice", 30, false);  // does not work because "Alice" is a string splice (not a string)
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);  //works because we converted "Alice" from a string splice to a string.

    println!("Human Tuple: {:?}", human);

    let integer_thing = 2;
    println!("Integer Thingy is {}", integer_thing)
}
