use std::io::{self};

mod calculator;

fn main() {
    let mut my_calc = calculator::Calculator::init();
    println!("Please select operation:");
    println!("1 = Add");
    println!("2 = Substract");
    println!("3 = Multiply");
    println!("4 = Divide");

    let mut input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut input).expect("Failed to read line");
    // let string = std::io::stdin()
    //     .read_line(&mut input)
    //     .ok()
    //     .expect("Failed to read line");
    let trimmed_input = input.trim();

    match trimmed_input {
        "1" | "2" | "3" | "4" => {
            println!("Input first number:");
            let mut input_first_num = String::new();
            stdin
                .read_line(&mut input_first_num)
                .expect("Failed to read line for first number");
            let trimmed_input_first_num = input_first_num.trim();
            let raw_f64_first_num = trimmed_input_first_num
                .parse::<f64>()
                .expect("Failed converting to f64!");
            let rounded_first_num = format!("{:.2}", raw_f64_first_num);
            let rounded_f64_first_num = rounded_first_num
                .parse::<f64>()
                .expect("Failed converting to f64!");
            my_calc.set_first_num(rounded_f64_first_num);

            println!("Input second number:");
            let mut input_second_num = String::new();
            stdin
                .read_line(&mut input_second_num)
                .expect("Failed to read line for second number");
            let trimmed_input_second_num = input_second_num.trim();
            let raw_f64_second_num = trimmed_input_second_num
                .parse::<f64>()
                .expect("Failed converting to f64!");
            let rounded_second_num = format!("{:.2}", raw_f64_second_num);
            let rounded_f64_second_num = rounded_second_num
                .parse::<f64>()
                .expect("Failed converting to f64!");
            my_calc.set_second_num(rounded_f64_second_num);

            match trimmed_input {
                "1" => {
                    my_calc.add();
                }
                "2" => {
                    my_calc.substract();
                }
                "3" => {
                    my_calc.multiply();
                }
                "4" => {
                    my_calc.divide();
                }
                _ => {
                    println!("Wrong operation code!");
                    return;
                }
            }
            println!("Result: {:.2}", my_calc.get_result());
            return;
        }
        _ => {
            println!("Wrong operation code!");
            return;
        }
    }
}
