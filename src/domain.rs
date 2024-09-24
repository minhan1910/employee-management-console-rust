use std::collections::HashMap;

// pub struct Employee {
//     name: String
// }

// impl Employee {
//     pub fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string()
//         }
//     }
// }

// pub struct Department {
//     name: String
// }

// impl Department {
//     pub fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string()
//         }
//     }
// }

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, employee: &str, department: &str) -> () {
        self.departments
            .entry(department.to_string())
            .or_insert_with(Vec::new)
            .push(employee.to_string());
    }

    pub fn get_employees_by_department(&self, department: &str) -> Vec<String> {
        if let Some(employees) = self.departments.get(department) {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            sorted_employees
        } else {
            Vec::new()
        }
    }

    pub fn get_all_employees(&self) -> HashMap<String, Vec<String>> {
        let mut all_employees = HashMap::new();
        for (dept, employees) in &self.departments {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            all_employees.insert(dept.clone(), sorted_employees);
        }
        all_employees
    }
}

