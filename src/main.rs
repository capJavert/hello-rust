use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Utils;

impl Utils {
     // i know there is a list.sort, but for practice sake
    fn sort(list: &mut Vec<i64>) -> Vec<i64> {
        for index in 0..list.len() {
            for index2 in 0..list.len() - 1 - index {
                if list[index2] > list[index2 + 1] {
                    let temp = list[index2];
                    list[index2] = list[index2 + 1];
                    list[index2 + 1] = temp;
                }
            }
        }   

        list.to_vec()
    }
}

struct Program {
    math: ProgramMath
}

struct ProgramMath;

impl ProgramMath {
    fn mean(&self, list: &Vec<i64>) -> f64 {
        let mut total = 0;

        for item in list {
            total += item
        }

        total as f64 / 2.0
    }

    fn median(&self, list: &Vec<i64>) -> f64 {
        let sorted = Utils::sort(&mut list.to_vec());

        match sorted.len() {
            0 => 0.0,
            1 => sorted[0] as f64,
            _ => {
                let half = (((sorted.len() / 2) as f64).floor()) as usize;

                if sorted.len() % 2 != 0 {
                    sorted[half] as f64
                } else {
                    (sorted[half - 1] + sorted[half]) as f64 / 2.0
                }
            }
        }
    }
}

impl Program {
    fn init() -> Program {
        Program {
            math: ProgramMath
        }
    }

    fn fahrenheit_to_celsius(&self, value: f64) -> f64 {
        const RATIO: f64 = 5.0 / 9.0;
        let celsius = value - 32.0;
        let celsius = celsius * RATIO;
    
        return celsius
    }
    
    fn guessing_game(&self, ) {
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
    
    fn fibonacci(&self, num: u128, memo: &mut HashMap<u128, u128>) -> u128 {
        if memo.contains_key(&num) {
            memo[&num]
        } else if num <= 1 {
            1
        } else {
            let result = self.fibonacci(num - 1, memo) + self.fibonacci(num - 2, memo);
            memo.insert(num, result);
    
            result
        }
    }
    
    fn twelve_days_of_xmas(&self) {
        for day in 0..12 {
            println!("On the {}. day of Christmas, my true love sent to me...", day + 1)
        }
    }

    fn rectangle(&self) {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rectangle = Rectangle {
            width: 69,
            height: 69
        };

        println!("Area of {}x{} rectangle is {}", rectangle.width, rectangle.height, rectangle.area())
    }

    fn pig_latin(&self, text: String) -> String {
        let vowels = ["a", "e", "i", "o", "u"].map(|s| String::from(s));

        let words: Vec<&str> = text.trim().split(" ").collect();
        let mut pig_words: Vec<String> = Vec::new();

        for word in words.iter() {
            let mut pig_word = String::from("");
            let mut extension = String::from("");

            for (index, letter) in word.to_ascii_lowercase().chars().enumerate() {
                if index == 0 {
                    if vowels.contains(&letter.to_string()) {
                        extension.push_str("hey");
                    } else {
                        extension.push(letter);
                        extension.push_str("ay");

                        continue
                    }
                }

                pig_word.push(letter);
            }

            pig_word.push_str(&extension);
            pig_words.push(pig_word);
        }

        pig_words.join(" ")
    }
}

fn main() {
    let program = Program::init();

    println!("Select program");
    let mut selection = String::new();
    
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    match selection.trim() {
        "GuessingGame" => program.guessing_game(),
        "FahrenheitToCelsius" => {
            let mut value = String::new();
    
            println!("Input celsius value");

            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
        
                match value.trim().parse() {
                    Ok(num) => {
                        println!("Celsius {}", program.fahrenheit_to_celsius(num))
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
                        println!("{}", program.fibonacci(num, &mut HashMap::new()))
                    },
                    Err(_) => {
                        println!("Not a number")
                    },
                };
        }
        "12DaysOfXmas" => program.twelve_days_of_xmas(),
        "Rectangle" => program.rectangle(),
        "Mean" => {
            let mut value = String::new();
    
            println!("Input Comma-separated list");

            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
        
            let list: Vec<i64> = value.trim().split(",").map(|s| s.parse().expect("Parse failed")).collect();
            println!("{}", program.math.mean(&list));
        },
        "Median" => {
            let mut value = String::new();
    
            println!("Input Comma-separated list");

            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
        
            let list: Vec<i64> = value.trim().split(",").map(|s| s.parse().expect("Parse failed")).collect();
            println!("{}", program.math.median(&list));
        },
        "PigLatin" => {
            let mut value = String::new();
    
            println!("Input some text");

            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
        
            println!("{}", program.pig_latin(String::from(value.trim())));
        },
        _ => {
            println!("404");
        }
    }
}
