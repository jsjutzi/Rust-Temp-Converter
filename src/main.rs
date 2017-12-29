use std::io;

fn main(){
    println!("Enter degrees in Farenheit:");
    let mut guess = String::new();
    

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess = guess.trim().parse::<i32>().expect("invalide input");
    
    let celsius = transfer_temp(guess);

    println!("{} degress in Farenheit is equal to {} degress in Celsius", guess, celsius);
}

fn transfer_temp(farenheit: i32) -> i32 {
    return (farenheit - 32)/(9/5)
   
}
