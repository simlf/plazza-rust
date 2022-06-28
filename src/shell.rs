// Extern crate
use std::io::Write;

// Local crate
use crate::pizza::Pizza;

pub struct Shell;

impl Shell {
    pub fn run() {
        loop {
            let input = Self::prompt("> ");
            match input.as_str() {
                "status" => println!("Status"),
                _ => {
                    Self::check_orders(input);
                }
            }
        }
    }

    fn prompt(name:&str) -> String {
        let mut line = String::new();

        print!("{}", name);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

        line.trim().to_string()
    }

    fn check_orders(input: String){
        if input.contains(";") {
            let command: Vec<&str> = input.split(" ;").collect();
            println!("{:?}", command);
        } else {
            let command: Vec<&str> = input.split_whitespace().collect();
            let pizza: Pizza = Pizza::parse(&input).unwrap();
        }
    }
}
