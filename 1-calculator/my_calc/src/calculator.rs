pub struct Calculator {
    first_num: f64,
    second_num: f64,
    result_num: f64,
}

impl Calculator {
    pub fn init() -> Calculator {
        Calculator {
            first_num: 0.0,
            second_num: 0.0,
            result_num: 0.0,
        }
    }

    pub fn set_first_num(&mut self, num: f64) {
        self.first_num = num;
    }

    pub fn set_second_num(&mut self, num: f64) {
        self.second_num = num;
    }

    pub fn add(&mut self) {
        self.result_num = self.first_num + self.second_num;
    }

    pub fn substract(&mut self) {
        self.result_num = self.first_num - self.second_num;
    }

    pub fn multiply(&mut self) {
        self.result_num = self.first_num * self.second_num;
    }

    pub fn divide(&mut self) {
        self.result_num = self.first_num / self.second_num;
    }

    pub fn get_result(&self) -> f64 {
        self.result_num
    }
}
