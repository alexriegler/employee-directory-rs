use crate::department::Department;
use crate::employee::Employee;
use std::collections::BTreeMap;

pub fn print_department(department: &str, employees: &Vec<&Employee>) {
    println!("Department {department}:");
    if employees.is_empty() {
        println!("No employees found!");
    } else {
        for employee in employees {
            println!("{}", employee.name);
        }
    }
}

pub fn print_company(employees_by_department: &BTreeMap<&Department, Vec<&Employee>>) {
    for (department, employees) in employees_by_department {
        print_department(&department.name, employees);
    }
}
