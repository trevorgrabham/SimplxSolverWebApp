use crate::tableau::Tableau;

impl Tableau {
    pub fn iterate(&mut self) {
        match self.solve_algorithm.as_str() {
            "standard" => {
                self.select_entering_var(); 
                if self.solved || self.error {
                    return;
                }
                self.select_leaving_var(); 
                if self.solved || self.error {
                    return;
                }
                if self.DEBUG {
                    println!("Entering index: {:?}\nLeaving index: {:?}\n", self.entering_var_index, self.leaving_var_index);
                }
                self.pivot();
            },
            "dual" => {
                self.select_leaving_var(); 
                if self.solved || self.error {
                    return;
                }
                self.select_entering_var(); 
                if self.solved || self.error {
                    return;
                }
                self.pivot();
            },
            _ => { 
                self.error = true;
                self.error_message = String::from("Invalid solve algorithm selection.");
            }
        }
    }
}