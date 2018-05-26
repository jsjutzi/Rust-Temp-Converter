//import traites into scope

use std::io;
use std::io::Write;
use std::str::FromStr;

//---------------------------------------Function that converts farenheit temp into celsius.  
fn main(){
    println!("Enter degrees in Farenheit:");
    let mut guess = String::new();
    

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess = guess.trim().parse::<i32>().expect("invalid input");
    
    let celsius = transfer_temp(guess);

    println!("{} degress in Farenheit is equal to {} degrees in Celsius", guess, celsius);

    let denominator = gcd(4, 8);
    let checker = check_number();

    println!("The greates common denominator of 4 and 8 is {}", denominator, checker)
}

fn transfer_temp(farenheit: i32) -> i32 {
    return (farenheit - 32)/(9/5)
   
}

//Function that will return the greatest common denominator of two arguments.  N and M are declared as mutable unsigned 64 bit integers, the
//"u64" following the arrow declares the return type for the function, which in this case will also be an unsigned 64 bit integer

fn gcd (mut n: u64, mut m: u64) -> u64 {

    /*assert is a macro, indicated by !, which will check to see if the statement is true.  If it is not, Rust will terminate the program
    with an error message. */

    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {

            /* let keyword can be used to define variables ONLY within function bodies, assuming Rust can infer it's type based on output*/

            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    /* return keyword isn't necessary in Rust, the last statement in a function that isn't followed by a semicolon is the default return value*/
    n
}

/*Testing is built into rust and can be executed using the #[test] notation as below.  Upon compiling, any function with the #[test] header
  is ignored.  However, running the cargo test command anywhere in the directory will execute all notated test in the program. SSH
*/

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

}

fn check_number() {
    //Vec declares a Vector type, comparable to Array in JS or list in Python
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
        .expect("error passing argument"));
    }
    //Check to see if number has at least one value
    if number.len() == 0 {
        //"std::io::stderr()" provides standard error output stream to writeln! macro, unwrap() checks to see that the error message itself prints
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }
}