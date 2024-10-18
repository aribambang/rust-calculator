use std::io;

fn display_menu() {
    println!("==== Menu ====");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Exit");
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn operation(choice: u32, num1: f64, num2: f64) -> Result<f64, String> {
    match choice {
        1 => Ok(num1 + num2),
        2 => Ok(num1 - num2),
        3 => Ok(num1 * num2),
        4 => {
            if num2 != 0.0 {
                Ok(num1 / num2)
            } else {
                Err(String::from("Error: Division by zero is not allowed."))
            }
        },
        _ => Err(String::from("Invalid choice. Please enter a number from 1 to 4.")),
    }
}


fn main() {
    loop {
        display_menu();

        let choice_input = get_input("Enter your choice:");
        let choice = match choice_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number from 1 to 5.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting the program. Goodbye!");
            break;
        }

        let num1_input = get_input("Enter the first number:");
        let num2_input = get_input("Enter the second number:");

        let num1: f64 = match num1_input.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number. Please enter valid numbers.");
                continue;
            }
        };

        let num2: f64 = match num2_input.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number. Please enter valid numbers.");
                continue;
            }
        };

        match operation(choice, num1, num2) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(operation(1, 5.0, 3.0).unwrap(), 8.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(operation(2, 10.0, 4.0).unwrap(), 6.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(operation(3, 6.0, 7.0).unwrap(), 42.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(operation(4, 8.0, 2.0).unwrap(), 4.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(
            operation(4, 10.0, 0.0),
            Err(String::from("Error: Division by zero is not allowed."))
        );
    }

    #[test]
    fn test_invalid_choice() {
        assert_eq!(
            operation(99, 10.0, 5.0),
            Err(String::from("Invalid choice. Please enter a number from 1 to 4."))
        );
    }
}