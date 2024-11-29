use std::io::{self, Write};

fn main() {
    println!("Temperature Converter");
    println!("--------------------");

    loop {
        println!("\nSelect conversion type:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Celsius to Kelvin");
        println!("4. Kelvin to Celsius");
        println!("5. Exit");

        print!("\nEnter your choice (1-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if choice == 5 {
            println!("Goodbye!");
            break;
        }

        print!("Enter temperature value: ");
        io::stdout().flush().unwrap();

        let mut temp: String = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => println!("{}°C = {}°F", temp, celsius_to_fahrenheit(temp)),
            2 => println!("{}°F = {}°C", temp, fahrenheit_to_celsius(temp)),
            3 => println!("{}°C = {}K", temp, celsius_to_kelvin(temp)),
            4 => println!("{}K = {}°C", temp, kelvin_to_celsius(temp)),
            _ => println!("Invalid choice!")
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        assert_eq!(celsius_to_kelvin(0.0), 273.15);
        assert_eq!(celsius_to_kelvin(-273.15), 0.0);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_eq!(kelvin_to_celsius(273.15), 0.0);
        assert_eq!(kelvin_to_celsius(0.0), -273.15);
    }
}