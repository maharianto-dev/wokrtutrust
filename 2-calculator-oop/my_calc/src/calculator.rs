use std::error::Error;

pub struct Calculator {
    num_list: Vec<f64>,
    operator_list: Vec<String>,
    result: f64,
}

impl Calculator {
    pub fn init() -> Calculator {
        Calculator {
            num_list: vec![],
            operator_list: vec![],
            result: 0.0,
        }
    }

    pub fn set_num_list(&mut self, num_list: &str) -> Result<(), Box<dyn Error>> {
        let temp_vec = num_list
            .split_ascii_whitespace()
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;
        let rounded_temp_vec = temp_vec
            .iter()
            .map(|x| format!("{:.2}", x).parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;
        self.num_list = rounded_temp_vec;
        Ok(())
    }

    pub fn get_num_list(&self) -> &Vec<f64> {
        &self.num_list
    }

    pub fn set_operator_list(&mut self, operator_list: &str) -> Result<(), Box<dyn Error>> {
        let temp_vec = operator_list
            .split_ascii_whitespace()
            .map(|x| match x {
                "+" | "-" | "*" | "/" => Ok(x.to_string()),
                _ => Err(format!(
                    "Wrong operators! Only \"+\", \"-\", \"*\", \"/\" are accepted!"
                )),
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.operator_list = temp_vec;
        Ok(())
    }

    pub fn get_operator_list(&self) -> &Vec<String> {
        &self.operator_list
    }

    pub fn find_result(&mut self) {
        println!("Calculating result for:");
        let mut result = 0.0;
        let mut next_operator = String::new();
        for (index, number) in self.num_list.iter().enumerate() {
            if index == &self.num_list.len() - 1 {
                print!("{:.2}\n", number);
                if index == 0 {
                    result = number.clone();
                } else {
                    result = self.do_operation(&result, &number, &next_operator)
                }
            } else {
                print!("{:.2} {} ", number, &self.operator_list[index]);
                if index == 0 {
                    result = number.clone();
                } else {
                    result = self.do_operation(&result, &number, &next_operator)
                }
                next_operator = self.operator_list[index].clone();
            }
        }
        self.result = result;
    }

    fn do_operation(&self, n1: &f64, n2: &f64, op: &str) -> f64 {
        let mut result = 0.0;
        match op {
            "+" => {
                result = n1 + n2;
            }
            "-" => {
                result = n1 - n2;
            }
            "*" => {
                result = n1 * n2;
            }
            "/" => {
                result = n1 / n2;
            }
            _ => {}
        }
        result
    }

    pub fn get_result(&self) -> &f64 {
        &self.result
    }
}
