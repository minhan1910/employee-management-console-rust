use crate::domain::Company;
use std::collections::HashMap;

pub struct Controller {
    company: Company,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            company: Company::new(),
        }
    }

    pub fn add_employee(&mut self, employee: &str, department: &str) -> () {
        self.company.add_employee(employee, department);
    }

    pub fn get_employees_by_department(&self, department: &str) -> Vec<String> {
       self.company.get_employees_by_department(department)
    }

    pub fn get_all_employees(&self) -> HashMap<String, Vec<String>> {
       self.company.get_all_employees()
    }
}



