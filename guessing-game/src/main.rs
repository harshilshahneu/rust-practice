use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..=100);

   loop {

       println!("Enter the number : ");
    
       let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("some error");
    
        println!("you guessed : {guess}");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less than"),
            Ordering::Greater => println!("Greater than"),
            Ordering::Equal => {
                println!("correct");
                break;
            },
        }
   }
}
