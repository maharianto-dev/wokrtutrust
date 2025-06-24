use std::{
    error::Error,
    io::{self},
};

use crate::calculator::Calculator;

mod calculator;

fn do_calculation(my_calc: &mut Calculator) {
    println!();
    my_calc.find_result();
    println!();
    println!("Result:");
    println!("{:.2}", my_calc.get_result());
}

fn validate_numbers_and_operators_count(my_calc: &Calculator) -> Result<(), Box<dyn Error>> {
    let number_list_length = my_calc.get_num_list().len();
    let operator_list_length = my_calc.get_operator_list().len();
    match number_list_length - operator_list_length {
        1 => Ok(()),
        _ => {
            if number_list_length == 1 && operator_list_length == 0 {
                return Ok(());
            }
            Err(format!("number and operator count does not match!").into())
        }
    }
}

fn read_operators(my_calc: &mut Calculator) -> Result<(), Box<dyn Error>> {
    println!();
    println!("Please input your list of operators (divided by space \" \"):");

    let mut input_operator_list = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut input_operator_list)
        .expect("Failed to read line");
    let trimmed_input_operator_list = input_operator_list.trim();
    my_calc.set_operator_list(trimmed_input_operator_list)?;
    Ok(())
}

fn read_numbers(my_calc: &mut Calculator) -> Result<(), Box<dyn Error>> {
    println!();
    println!("Please input your list of numbers (divided by space \" \"):");

    let mut input_num_list = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut input_num_list)
        .expect("Failed to read line");
    let trimmed_input_num_list = input_num_list.trim();
    my_calc.set_num_list(trimmed_input_num_list)?;
    Ok(())
}

fn start_calculator() -> Result<(), Box<dyn Error>> {
    let mut my_calc = calculator::Calculator::init();
    read_numbers(&mut my_calc)?;
    read_operators(&mut my_calc)?;
    validate_numbers_and_operators_count(&my_calc)?;
    do_calculation(&mut my_calc);
    Ok(())
}

fn show_restart_prompt() -> Result<String, Box<dyn Error>> {
    println!("Please input \"q\" to exit or input \"c\" to continue:");
    let mut input_init = String::new();
    let stdin = io::stdin();
    let result = stdin.read_line(&mut input_init);
    match result {
        Ok(_) => {
            let trimmed_input_init = input_init.trim();
            Ok(trimmed_input_init.to_string())
        }
        Err(_e) => Err(format!("wrong input! Please try again!").into()),
    }
}

fn main() {
    'calculator: loop {
        match start_calculator() {
            Ok(_) => {
                println!("Restarting application...");
                println!();
                loop {
                    match show_restart_prompt() {
                        Ok(trimmed_input_init) => match trimmed_input_init.as_str() {
                            "c" => continue 'calculator,
                            "q" => {
                                println!("Goodbye!");
                                break 'calculator;
                            }
                            _ => {
                                eprintln!("Wrong input! Please try again!");
                                println!();
                            }
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e.to_string());
                            println!();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e.to_string());
                println!("Restarting application...");
                println!();
                loop {
                    match show_restart_prompt() {
                        Ok(trimmed_input_init) => match trimmed_input_init.as_str() {
                            "c" => continue 'calculator,
                            "q" => {
                                println!("Goodbye!");
                                break 'calculator;
                            }
                            _ => {
                                eprintln!("Wrong input! Please try again!");
                                println!();
                            }
                        },
                        Err(e) => {
                            eprintln!("Error: {}", e.to_string());
                            println!();
                        }
                    }
                }
            }
        }
    }
}
