use crate::employee_directory::EmployeeDirectory;
use crate::parser::*;
use crate::print::*;
use std::collections::HashMap;
use std::io::{self, Write};

mod department;
mod employee;
mod employee_directory;
mod parser;
mod print;

fn main() {
    println!("Welcome to the Employee Directory!");

    let mut employee_directory = EmployeeDirectory {
        employees_to_directory: HashMap::new(),
    };

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim() == "quit" {
            break;
        }

        match parse_command(&input) {
            Some(Command::Add(employee, department)) => {
                employee_directory.add_employee_to_department(employee, department)
            }
            Some(Command::Remove(employee, department)) => {
                employee_directory.remove_employee_from_department(&employee, &department)
            }
            Some(Command::Print(PrintOption::Department(department))) => {
                let department_name = &department.name;
                let employees = employee_directory.get_department(&department);
                print_department(&department_name, &employees);
            }
            Some(Command::Print(PrintOption::Company)) => {
                let company = employee_directory.get_all_departments();
                print_company(&company);
            }
            None => (),
        }
    }
}
