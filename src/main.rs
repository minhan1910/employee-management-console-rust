use std::io;
use std::collections::HashMap;

use EmployeeManagementProject::domain::Company;
use EmployeeManagementProject::utils::{Menu, pause, clear_screen};

fn main() {
    println!("Welcome to employee app!");

    let mut input = String::new();
    let mut company: Company = Company::new();

    loop {
        clear_screen();

        Menu::new();
    
        input.clear();

        // Enter input (option)
        io::stdin().read_line(&mut input).expect("Failed to input");

        // Parse option
        let option: i32 = match input.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please enter number for option!");
                continue;
            }
        };

        // match option to execute domain logic
        clear_screen();

        match option {
            1 => {
                add_new_employee_to_department(&mut company);
            }
            2 => {
                println!("There is the option 2.");
            }
            3 => {
                let mut deparment = String::new();

                println!("Please enter your deparment: ");
                io::stdin()
                    .read_line(&mut deparment)
                    .expect("Failed to input");
                let deparment = deparment.trim().to_string();

                list_all_employee_in_company_by_department_v2(&company, &deparment[..]);
            }
            4 => {
                list_all_employee_and_deparment_in_company_v2(&company);
            }
            5 => {
                println!("See you again!");
                break;
            }
            _ => {
                println!(
                    "You must enter from 1 to 4 options. Your option {option} is not supported."
                );
            }
        }

        pause();
    }
}


fn add_new_employee_to_department(company: &mut Company) -> () {
    let mut name = String::new();
    let mut deparment = String::new();

    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to input");
    let name = name.trim();

    println!("Please enter your deparment: ");
    io::stdin()
        .read_line(&mut deparment)
        .expect("Failed to input");
    let deparment = deparment.trim();

    // match company.get_mut(&deparment) {
    //     Some(list_employee) => {
    //         list_employee.push(name);
    //     }
    //     None => {
    //         company.insert(deparment, vec![name]);
    //     }
    // }    

    company.add_employee(name, deparment);
}

fn list_all_employee_in_company_by_department_v2(
    company: &Company, 
    department_to_filter: &str
) -> () {
    let employees = company.get_employees_by_department(department_to_filter);
    
    println!("{}:", department_to_filter);
    if !employees.is_empty() {
        for employee in &employees {
            println!("-- {}", employee);
        }
    } else {
        println!("Department has not employees");
    }
    println!(); // Print a newline for better readability
}


fn list_all_employee_in_company_by_department(
    company: &HashMap<String, Vec<String>>,
    department_to_filter: &str,
) -> () {
    // for (deparment, list_employee) in company {
    //     if deparment == department_to_filter {
    //         println!("{deparment}:");
    //         for employee in list_employee {
    //             println!("--{employee}");
    //         }
    //         println!();
    //         break;
    //     }
    // }    

    if let Some(list_employee) = company.get(department_to_filter) {
        println!("{}:", department_to_filter);
        for employee in list_employee {
            println!("-- {}", employee);
        }
        println!(); // Print a newline for better readability
    } else {
        println!("No employees found in the department: {}", department_to_filter);
    }
}

fn list_all_employee_and_deparment_in_company_v2(company: &Company) -> () {
    for (deparment, employees) in company.get_all_employees() {
        println!("{deparment}:");
        if !employees.is_empty() {
            for employee in employees {
                println!("--{employee}");
            }
        } else {
            println!("Deparment( {deparment} ) has not employees");
        }
        println!();
    }
}

fn list_all_employee_and_deparment_in_company(company: &HashMap<String, Vec<String>>) -> () {
    for (deparment, list_employee) in company {
        println!("{deparment}:");
        for employee in list_employee {
            println!("--{employee}");
        }
        println!();
    }
}
