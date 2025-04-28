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

    let my_mix_tuple = ("Kratos", 23, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5] => Contiguous Memory Locations
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", {number_slices});

    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", {animal_slices});

    let book_slices:&[&String] = &[&"IT".to_string(),
    &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", {book_slices});

    // Strings Vs String Slices (&str)
    // Strings [ growable, mutable, owned string type ] 


}