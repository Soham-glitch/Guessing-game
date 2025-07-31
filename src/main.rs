
use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main(){
        println!("guess the number");
        loop{   
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("the secret number is: {secret_number}");
    
    println!("input the number");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read");
    
    let guess:u32 =match guess.trim().parse() {
        Ok(num)=>num,
    Err(_)=>continue,
    };
    println!("you have guessed {guess}");
     
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too Biggg!"),
        Ordering::Equal =>{
            println!("that's perfect..!");
            break;
        },

    }
}
}