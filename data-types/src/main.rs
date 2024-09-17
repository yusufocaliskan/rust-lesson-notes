use std;
fn main() {
    // Every value in Rust is of a certain data type,
    println!("1. ------ Data types");

    //Setting a type
    // #[allow(warnings)]
    let guess: &str = "32";

    //expect() must be used why type conversion
    //STR to int ==> type casting
    let guess: u32 = guess.parse().expect("Not a number");

    println!(
        "Type Of Guess is {} with value {} ",
        std::any::type_name_of_val(&guess),
        guess
    );

    //1.Scallar types
    //Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    println!("2. ------ Scalar Types");

    println!(" --- boolean --- ");

    let is_logged_in: bool = false;
    if is_logged_in {
        println!("LoggedIn!");
    } else {
        println!("Logged Out");
    }
    println!(" --- boolean --- ");

    #[allow(warnings)]
    let f = 442.4343;

    println!(" == Char ==");

    // Mostly one character
    #[allow(warnings)]
    let z: char = '⌘';

    println!(" ==== Tuple Types ===");

    //a tuple
    let tup = (500, 5.4, 1);

    //debugger ===> {:?}
    println!("{:?}", tup);
    println!("{:.2} {:.1}", tup.2, tup.1);

    println!("< === Extraction === >");
    let (x, y, z) = tup;
    println!("Z: {z} Y: {y} x: {x}");

    println!("Tuple.3 {}", tup.2);

    println!("Array ===");
    let months = ["January", "February"];
    println!("{:?}", months);

    //an array with 5 elements in type of i32
    println!("an array with 5 elements in type of i32");
    let months_with_types: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", months_with_types);

    println!("To initialize an array to contain the same value for each element");

    let the_same_elements = [3; 5];
    println!("{:?}", the_same_elements);

    println!(
        "First element of the_same_elements: {}",
        the_same_elements[0]
    );
}
