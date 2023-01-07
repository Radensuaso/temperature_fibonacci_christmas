use std::io;

pub(crate) fn convert_temperature() {
    loop {
        println!("You need to...");
        println!("1. ...convert from Celsius to Fahrenheit.");
        println!("2. ...convert from Fahrenheit to Celsius.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: &str = choice.trim();

        match choice {
            "1" => {
                let user_value: f64 = user_input_value();
                let fahrenheit_value: f64 = (user_value * 1.8) + 32.0;
                println!("{}ºC is {}ºF.", user_value, fahrenheit_value);
                break;
            }
            "2" => {
                let user_value: f64 = user_input_value();
                let celsius_value: f64 = (user_value - 32.0) * 0.5556;
                println!("{}ºF is {}ºC.", user_value, celsius_value);
                break;
            }
            _ => continue,
        }
    }
}

fn user_input_value() -> f64 {
    let value = loop {
        let mut value = String::new();

        println!("Type the value you want to convert.");

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break value;
    };
    value
}
