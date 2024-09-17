fn main() {
    println!(" ==== Variables! ==== ");

    //1. ---------- variables
    //To be able to change a variable value,
    // it should be declared as mutable (shortcut: mut)
    println!("1. ---------- variable");
    let mut x = 5;
    println!("Valu of x:  {x}");

    x = 4;
    println!("New value of x:  {x}");

    //2. ---------- constants
    //Note: Constant connot be a mutable variable.
    println!("2. ---------- constants");
    #[allow(dead_code)]
    const BASE_URL: &str = "https://pirevook.com";
    println!("{BASE_URL}");

    //3. ---------- shadowing
    //a. we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
    println!("3. ---------- shadowing");

    let y = 3;
    {
        let y = 9 + y;
        println!("Y in the another scope : {y}");
    }
    let y = x + 4;
    println!("{y}");

    //b. we can change the type of the value but reuse the same name.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spacess ==> {spaces}");
}
