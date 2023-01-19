use std::collections::{HashMap};

struct Corporate {
    employees: HashMap<String, i32>,
    departments: HashMap<String, i32>,
    allocation: HashMap<String, Vec<String>>,
}

impl Corporate {
    fn new() -> Corporate {
        Corporate {
            employees: HashMap::new(),
            departments: HashMap::new(),
            allocation: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: String) {
        self.employees.insert(name, 0);
    }

    fn add_department(&mut self, name: String) {
        self.departments.insert(name, 0);
    }

    fn allocate(&mut self, employee: String, department: String) {
        let employees = self.allocation.entry(department).or_insert(Vec::new());
        employees.push(employee);
    }

    fn print_all(&self) {
        println!("\n[Corporate employees]");
        let deps = &mut self.departments.keys().cloned().collect::<Vec<String>>();
        deps.sort();
        for department in deps {
            print!("{} department :", department);
            let mut buf = String::new();
            let mut emps = match self.allocation.get(department) {
                Some(emps) => emps.clone(),
                None => {
                    let mut vec = Vec::new();
                    vec.push("None".to_string());
                    vec
                },
            };
            emps.sort();
            for employee in emps {
                buf.push_str(&format!(" {},", &employee));
            }
            buf.pop();
            println!("{}", buf);
        }
    }

    fn print(&self, department: String) {
        println!("\n[{} department]", department);
        let mut buf = String::new();
        let mut emps = match self.allocation.get(&department) {
            Some(emps) => emps.clone(),
            None => {
                let mut vec = Vec::new();
                vec.push("None".to_string());
                vec
            },
        };
        emps.sort();
        for employee in emps {
            buf.push_str(&format!("{}, ", &employee));
        }
        buf.pop();
        buf.pop();
        println!("{}", buf);
    }
}

use std::io::{self, Write};
pub fn interface(){
    let mut corporate = Corporate::new();

    println!("Corporate management interface");
    loop {
        println!("\nCommands: Add [employee] to [department], Print all, Print [department], Exit");
        print!("Type: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        }

        let mut input = input.trim().split_whitespace();
        let command = match input.next() {
            Some(command) => command,
            None => {
                println!("Invalid input!");
                continue;
            }
        };

        match command.to_lowercase().as_str() {
            "add" => {
                let employee = match input.next() {
                    Some(employee) => employee,
                    None => {
                        println!("Invalid input!");
                        continue;
                    }
                };
                let department = match input.next() {
                    Some(department) => {
                        if department == "all" {
                            println!("Invalid input!");
                            continue;
                        }
                        department
                    },
                    None => {
                        println!("Invalid input!");
                        continue;
                    }
                };
                if department == "to" {
                    let department = match input.next() {
                        Some(department) => department,
                        None => {
                            println!("Invalid input!");
                            continue;
                        }
                    };
                    corporate.add_employee(employee.to_string());
                    corporate.add_department(department.to_string());
                    corporate.allocate(employee.to_string(), department.to_string());
                } else {
                    println!("Invalid input!");
                    continue;
                }
            },
            "print" => {
                let department = match input.next() {
                    Some(department) => department,
                    None => {
                        println!("Invalid input!");
                        continue;
                    }
                };
                if department == "all" {
                    corporate.print_all();
                } else {
                    corporate.print(department.to_string());
                }
            },
            "exit" => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}