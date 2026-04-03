use std::{collections::HashMap, io};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Enter \"Add <Name> to <Department>\" to add an employee");
    println!("Enter \"Employees <Department>\" to get a list of department employees");
    println!("Enter \"exit\" to close the app");

    loop {
        let mut raw_input = String::new();

        // Add Sally to Engineering
        // Add Amir to Sales
        // Add Max to Sales
        // Add Lisa to Sales
        io::stdin()
            .read_line(&mut raw_input)
            .expect("Failed to read line");

        let input = raw_input.trim().to_string();

        let input = match input.as_str() {
            "exit" => break,
            "" => continue,
            _ => input,
        };

        let command = input.split_whitespace().next();

        match command {
            None => continue,
            Some("Add") => {
                if let Some(rest) = input.strip_prefix("Add ") {
                    add_employee(&mut company, rest.trim());
                }
            }
            Some("Employees") => {
                if let Some(rest) = input.strip_prefix("Employees ") {
                    get_employees(&company, rest);
                }
            }
            Some(_) => {
                println!("You entered: {input}");
            }
        };
    }

    println!("{company:#?}");
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, employee: &str) {
    let parts: Vec<&str> = employee.split(" to ").collect();

    if parts.len() < 2 {
        println!("Invalid command. Use: Add <Name> to <Department>");
        return;
    }

    let name = parts[0].trim().to_string();
    let department = parts[1].trim().to_string();

    company.entry(department).or_default().push(name);
}

fn get_employees(company: &HashMap<String, Vec<String>>, department: &str) {
    match company.get(department) {
        Some(employees) => {
            println!("Employees in {department}:");
            let mut sorted = employees.clone();
            sorted.sort();
            for employee in &sorted {
                println!("  - {employee}");
            }
        }
        None => println!("The company does not have a {department} department."),
    }
}
