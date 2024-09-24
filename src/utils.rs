use std::io;
use std::process::Command;

pub struct Menu;
impl Menu {
    pub fn new() -> () {
        println!("---------------------------------------");
        println!("1. Add new employee to a department");
        println!("2. List all people in a department"); // -> choose the deparment for next step
        println!("3. List all people in the company by department"); // -> choose the department for next step
        println!("4. List all people with deparment in company");
        println!("5. Exit");
        println!("---------------------------------------");
    }
}

pub fn pause() {
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut String::new()); // Wait for user input
}

pub fn clear_screen() {
    Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .expect("Failed to clear console");
}
