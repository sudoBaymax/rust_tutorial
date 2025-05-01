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

    // All data types in rust are by default immutable (meaning you can't change the value)
    // To change this, we can do the following:
    let mut stone_cold: String = String::from("Hell, "); // must have "mut" included to make it mutable
    println!("Stone COld Says: {}", stone_cold); 
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    //  B- &str (String Slice)
    let string: String = String::from("Hello, World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
    even_or_odd(&[1,2,3,4,5]);

}

fn even_or_odd(numbers: &[i32]) {
    for number in numbers {
        if number % 2 == 0 {
            println!("{} is Even", number);
        }
        else {
            println!("{} is odd", number);
        }

    }
}


// Generate Report(profiles: &[(NAME, [Grades])]);
// for profile in profiles:
//          average = sum(Grades) / 2
            // student = profile[0]
            // Grades = profile[1]

            // if average >= 90: A
            // if average >= 80 and grade <= 89: B
            // if average >= 70 and grade <= 79: C
            // if average >= 60 and grade <= 69: D
            // if average < 60: F

        // result: String = String::from("Student: {} \nGrades: {} \nAverage: {} \nLetterGrade", student, Grades, average, letter);    