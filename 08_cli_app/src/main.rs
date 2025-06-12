use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    println!("welcome to the company employee management system!");
    println!("you can use the following commands:");
    println!("  - add [name] to [department]");
    println!("  - list [department]");
    println!("  - list all");
    println!("  - Quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse the command and arguments from the input
        let words: Vec<&str> = input.trim().split_whitespace().collect();

        match words.as_slice() {
            ["add", name, "to", department] => {
                let name_str = name.to_string();
                let dept_str = department.to_string();

                println!("added {} to {}", name_str, dept_str);

                departments
                    .entry(dept_str)
                    .or_insert_with(Vec::new)
                    .push(name_str);
            }

            ["list", department] => match departments.get(*department) {
                Some(employees) => {
                    println!("list of {}:", department);
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    for employee in sorted_employees {
                        println!("- {}", employee);
                    }
                }
                None => println!("{} is not found", department),
            },
            ["list", "all"] => {
                if departments.is_empty() {
                    println!("no employees found");
                    continue;
                }

                println!("list of all employees (by department):");
                let mut sorted_depts: Vec<_> = departments.keys().collect();
                sorted_depts.sort();

                for dept in sorted_depts {
                    println!("\n-- {} --", dept);
                    if let Some(employees) = departments.get(dept) {
                        let mut sorted_employees = employees.clone();
                        sorted_employees.sort();
                        for employee in sorted_employees {
                            println!("- {}", employee);
                        }
                    }
                }
            }
            ["quit"] => {
                println!("Quitting the program.");
                break;
            }
            _ => {
                println!("Invalid command. Please use the following format:");
                println!("  - add [name] to [department]");
                println!("  - list [department]");
                println!("  - list all");
                println!("  - quit");
            }
        }
    }
}
