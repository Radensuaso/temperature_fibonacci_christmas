mod christmas;
mod fib;
mod temp;

use std::io;
fn main() {
    loop {
        println!("You need to...");
        println!("1. ...convert temperature.");
        println!("2. ...know the nth Fibonacci number.");
        println!("3. ...listen to The Twelve Days of Christmas.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "1" => {
                temp::convert_temperature();
                break;
            }
            "2" => {
                fib::nth_fibonacci();
                break;
            }
            "3" => {
                christmas::the_twelve_days_of_christmas();
                break;
            }
            _ => continue,
        }
    }
}
