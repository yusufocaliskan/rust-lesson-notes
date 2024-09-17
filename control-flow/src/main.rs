fn main() {
    let test: &str = "Test";

    if test == "Test" {
        println!("Yes!");
    } else {
        println!("No!");
    }

    let cond: bool = true;
    let is_open = if cond { true } else { false };
    println!("Is Open {is_open}");

    println!("===== Loop Conditions ====");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this will break the loop and return the result
        }
    };

    println!("Result {result}");

    println!("----labeled_loop----");
    labeled_loop();

    println!("----loop_with_while----");
    loop_with_while();

    println!("----while_with_array----");
    while_with_array();

    println!("----for_loop----");
    for_loop();

    println!("---- for_with_reversed_tuple ----");
    for_with_reversed_tuple();
}

fn labeled_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("Count {count}");
        let mut remaining = 10;
        loop {
            println!("Remainin {remaining}");

            if remaining == 9 {
                //break the inner loop
                break;
            }

            if count == 2 {
                //breaks the outter look
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count {count}");
}

fn loop_with_while() {
    let mut number = 4;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("End of loop_with_while");
}

fn while_with_array() {
    let a = [1, 2, 5, 2, 4];
    let mut ix = 0;

    while ix < 5 {
        println!("Index : {}", a[ix]);
        ix += 1;
    }
}

fn for_loop() {
    let a = [10, 40, 50, 60, 20];

    for el in a {
        println!("El--> {}", el);
    }
}

fn for_with_reversed_tuple() {
    for num in (1..5).rev() {
        println!("{num}");
    }
}
