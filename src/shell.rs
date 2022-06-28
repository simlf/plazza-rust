use core::num;
// Extern crate
use std::{io::Write, collections::VecDeque};

// Local crate
use crate::pizza::Pizza;

// regina XL x3 ; margarita S x6 ; americana M x2 ; fantasia L x1
pub struct Shell {
    pub pizzas: VecDeque<Pizza>
}

impl Shell {
    fn prompt(&self, name:&str) -> String {
        let mut line = String::new();

        print!("{}", name);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

        line.trim().to_string()
    }

    pub fn run(&mut self) {
        loop {
            let input = self.prompt("> ");
            match input.as_str() {
                "status" => println!("Status"),
                _ => {
                    self.check_orders(input);
                }
            }
        }
    }

    fn check_orders(&mut self, input: String) {
        if input.contains(";") {
            self.order_pizzas(input);
        } else {
            self.order_pizza(input);
        }
    }

    fn order_pizzas(&mut self, input: String) {
        let commands: Vec<&str> = input.split(" ; ").collect();
        for command in commands {
                self.order_pizza(command.to_string());
        }

    }

    fn order_pizza(&mut self, command: String) {
        let mut pizza_number = self.check_pizza_number(command.to_string());

        while pizza_number >= 1 {
            let pizza = Pizza::parse(&command);

            match pizza {
                Ok(pizza) => {
                    self.pizzas.push_back(pizza);
                    pizza_number -= 1;
                }
                Err(e) => {
                    println!("{} in '{}' command", e, command);
                    break;
                }
            }
        }
    }

    fn check_pizza_number(&self, number: String) -> usize {
        let iter: String = number.split_whitespace().last().unwrap().split("x").collect();

        match iter.parse() {
            Ok(number) => number,
            Err(_) =>{
                println!("Error: Invalid pizza number");
                0
            }
        }
    }
}

