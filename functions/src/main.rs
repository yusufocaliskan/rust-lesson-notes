fn main() {
    println!(" === Function ===");

    function_with_params("Test");
    let y = {
        let x = 3;
        x + 3 // <<=== the means return..
    };
    let five_in_number = five();

    println!(" Five:  {five_in_number}");

    println!("xxxxyyyy {:?}", y);
}

fn function_with_params(x: &str) {
    println!("Value of the x : {x}");
}

//return 5
fn five() -> i32 {
    5
}
