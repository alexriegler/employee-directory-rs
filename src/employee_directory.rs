use crate::department::Department;
use crate::employee::Employee;
use std::collections::{BTreeMap, HashMap};

pub struct EmployeeDirectory {
    pub employees_to_directory: HashMap<Employee, Department>,
}

impl EmployeeDirectory {
    pub fn add_employee_to_department(&mut self, employee: Employee, department: Department) {
        let success_message = format!(
            "Successfully added {} to {}.",
            employee.name, department.name
        );
        if let Some(current_department) = self.employees_to_directory.get(&employee) {
            println!(
                "Employee {} already belongs to department {}.",
                employee.name, current_department.name
            );
        } else if self.employees_to_directory.insert(employee, department) == None {
            println!("{success_message}");
        }
    }

    pub fn remove_employee_from_department(
        &mut self,
        employee: &Employee,
        department: &Department,
    ) {
        if let Some(current_department) = self.employees_to_directory.get(employee) {
            if current_department == department {
                if self.employees_to_directory.remove(employee) != None {
                    println!(
                        "Successfully removed {} from {}.",
                        employee.name, department.name
                    );
                } else {
                    println!(
                        "No employee named {} belongs to department {}.",
                        employee.name, department.name
                    );
                }
            } else {
                println!(
                    "Employee {} does not belong to department {}.",
                    employee.name, department.name
                );
            }
        }
    }

    // Gets a list of all employees in the specified department.
    pub fn get_department(&self, department: &Department) -> Vec<&Employee> {
        let mut employees_in_department = Vec::new();
        for (employee, dept) in &self.employees_to_directory {
            if dept == department {
                employees_in_department.push(employee);
            }
        }
        employees_in_department.sort();
        employees_in_department
    }

    // Gets a map of department names to all employees in the respective departments.
    pub fn get_all_departments(&self) -> BTreeMap<&Department, Vec<&Employee>> {
        let mut all_employees_by_department = BTreeMap::new();
        for (employee, department) in &self.employees_to_directory {
            all_employees_by_department
                .entry(department)
                .or_insert(Vec::new())
                .push(employee);
        }
        all_employees_by_department
    }
}
