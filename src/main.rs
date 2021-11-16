use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

fn fahrenheit_to_celsius(value: f64) -> f64 {
    const RATIO: f64 = 5.0 / 9.0;
    let celsius = value - 32.0;
    let celsius = celsius * RATIO;

    return celsius
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn fibonacci(num: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if memo.contains_key(&num) {
        memo[&num]
    } else if num <= 1 {
        1
    } else {
        let result = fibonacci(num - 1, memo) + fibonacci(num - 2, memo);
        memo.insert(num, result);

        result
    }
}

fn twelve_days_of_xmas() {
    for day in 0..12 {
        println!("On the {}. day of Christmas, my true love sent to me...", day + 1)
    }
}

fn main() {
    println!("Select program");
    let mut selection = String::new();
    
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    match selection.trim() {
        "GuessingGame" => guessing_game(),
        "FahrenheitToCelsius" => {
            let mut value = String::new();
    
            println!("Input celsius value");

            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
        
                match value.trim().parse() {
                    Ok(num) => {
                        println!("Celsius {}", fahrenheit_to_celsius(num))
                    },
                    Err(_) => {
                        println!("Not a number")
                    },
                };
        }
        "Fibonacci" => {
            let mut value = String::new();
    
            println!("How many steps?");

            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
        
                match value.trim().parse() {
                    Ok(num) => {
                        println!("{}", fibonacci(num, &mut HashMap::new()))
                    },
                    Err(_) => {
                        println!("Not a number")
                    },
                };
        }
        "12DaysOfXmas" => twelve_days_of_xmas(),
        _ => {
            println!("404");
        }
    }
}
