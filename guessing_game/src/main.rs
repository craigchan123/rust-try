// from the standard library, bring in the io(input output) module
use std::io;

// from the rand crate, bring in the Rng module, it generates random numbers
use rand::Rng;

// from the std library, bring in the cmp(compare) module, then bring in the Ordering enum
// Ordering is an enum used to compare values, it has three variants(outcomes): Less, Greater, and Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //we call the thread_rng() function from the rand crate to get a random number generator
    //gen_range(start..=end) generates a random number in the range, lower and upper bounds inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //unused println!("The secret number is: {secret_number}");

loop {
    println!("Please input your guess.");

    // variable is immutable by default, so we need to make it mutable by adding 'mut'
    // String::new() creates a new empty String, it is growable
    // ::new()is a common name for a function that makes a new value of somekind
    let mut guess = String::new();

    //here we call the stdin() function from the io module
    io::stdin()
        // here we call the .read_line() method from stdin, to take user input
        // and it'll pass the input into 'guess' variable
        // & indicates that this is a reference
        .read_line(&mut guess)

        // as read_line() returns a Result type, either 'ok' or 'err', we need to handle the error case here
        // .expect will print the error message and exit the program if read_line returns an error
        .expect("Failed to read line");

    // having another guess variable here is called shadowing
    // and is often used to change the type of a variable
    // the guess on the right side is the string guess
    // the trim() method removes any whitespace from the beginning and end of the string(the \n or \r\n), and is a must before converting to a number
    // the parse() method converts the string to a number, and it returns a Result type
    // with match, we handle the result type instead of using expect(), which would crash on error
    //and the result of parse() is enum, either Ok(num) or Err(_)
    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,  //if parse successfully turned a string into a number, it returns Ok with the number
            Err(_) => continue,  //the _ means that we catch all values of Err, and if it fails to parse, it continues the loop
    };

println!("You guessed: {guess}");

// comparing guess with secret_number
// match expression decides what to do based on the outcome of the comparison
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;   //break makes the loop stop when win
        }
    }
}
}