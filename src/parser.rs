use crate::department::Department;
use crate::employee::Employee;
use std::str::SplitWhitespace;

pub enum PrintOption {
    Department(Department),
    Company,
}

pub enum Command {
    Add(Employee, Department),
    Remove(Employee, Department),
    Print(PrintOption),
}

// Attempts to parse an add or remove command.
//
// Parses the provided tokens to determine whether or not a valid print command was provided.
//
// * `tokens` - The tokens following the command introducer.
//
// Command syntax:
//      (Add|Remove) <employee> (to|from) <department>
// Note:
// In the syntax above, "Add" must be combined with "to" and "Remove" must be combined with "from".
fn parse_add_remove(tokens: &mut SplitWhitespace, command: &str) -> Option<Command> {
    if let Some(employee) = tokens.next() {
        if let Some(preposition) = tokens.next() {
            let expected_preposition = match command {
                "Add" => "to",
                "Remove" => "from",
                _ => "",
            };
            if preposition != expected_preposition {
                println!(
                        "Invalid command syntax: Expected \"{expected_preposition}\" but got \"{preposition}\" instead."
                    );
                return None;
            }
            if let Some(department) = tokens.next() {
                let command = match command {
                    "Add" => Some(Command::Add(
                        Employee {
                            name: employee.to_string(),
                        },
                        Department {
                            name: department.to_string(),
                        },
                    )),
                    "Remove" => Some(Command::Remove(
                        Employee {
                            name: employee.to_string(),
                        },
                        Department {
                            name: department.to_string(),
                        },
                    )),
                    _ => {
                        println!("Unrecognized command: {command}");
                        None
                    }
                };
                return command;
            }
        }
    }
    None
}

// Attempts to parse a print command.
//
// Parses the provided tokens to determine whether or not a valid print command was provided.
//
// * `tokens` - The tokens following the command introducer.
//
// Command syntax:
//      Print (department <department>|company)
fn parse_print(tokens: &mut SplitWhitespace) -> Option<Command> {
    if let Some(option) = tokens.next() {
        match option {
            "department" => {
                if let Some(name) = tokens.next() {
                    return Some(Command::Print(PrintOption::Department(Department {
                        name: name.to_string(),
                    })));
                } else {
                    println!("Invalid syntax: Expected department name after \"department\".");
                    return None;
                }
            }
            "company" => return Some(Command::Print(PrintOption::Company)),
            _ => {
                println!("Unexpected option encountered: {option}");
                return None;
            }
        }
    } else {
        println!("Invalid syntax: Expected \"department\" or \"company\" after \"Print\".");
        return None;
    }
}

// Attempts to parse a command.
//
// * `input` - The input string containing text to attempt to parse as a command.
pub fn parse_command(input: &str) -> Option<Command> {
    let mut tokens = input.split_whitespace();
    let command = tokens.next().unwrap_or("");
    match command {
        "Add" => parse_add_remove(&mut tokens, command),
        "Remove" => parse_add_remove(&mut tokens, command),
        "Print" => parse_print(&mut tokens),
        _ => {
            println!("Unrecognized command: {command}");
            None
        }
    }
}
