use rand::Rng;
use std;
fn main() {
    //Chapter 1
    //guessing_game()
}

//Create a guessing game! :))
#[allow(dead_code)]
fn guessing_game() {
    println!("Guess the number!");
    //geneate a number between 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("-------- Start ------ ");
        println!("Input your guess.: ");
        //Create a string
        //make it a multible so then we can set a number
        let mut guess = String::new();

        //Get the input from user
        //This will return a string, even user type number
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //convert the string number to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Check if the number lss, greater or equtal to the seecreet number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Greater => println!("Too Big"),
            std::cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
        //display the result
        println!("You guessed: {guess}");
    }
}
