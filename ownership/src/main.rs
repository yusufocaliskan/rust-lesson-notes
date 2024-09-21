fn main() {
    //=== Stack === :

    //Statick, fast but small data
    //Rust removes that from stack when the execution is done.
    // Data type like : bool, i32 , f64

    //=== Heap === :

    //Dinamick, big data but relatively slow
    //Data type like: Vec<T>, Box<T>, String
    //Referance of them would be saved in stakc by rust.

    //=== Ownership Rulse === :

    //1. Each value in Rust has an owner.
    //2. There can be only one owner at a time.
    //3. When the owner goes out of scope, the value will be dropped.

    // let s1 = String::from("Silav");
    // let s2 = s1;

    // println!("{s1}");

    let silav = String::from("Silav");
    let i = set_with_ref(&silav);

    println!("{i}");
    println!("{silav}");
}

fn set_with_ref(silav: &String) -> String {
    // let silav = String::from("43243434");

    print!("-----{}", silav);
    let testii = String::from("tests");

    return testii;
}
