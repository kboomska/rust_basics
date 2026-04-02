use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    add_employee(&mut company, "Sally to Engineering");
    add_employee(&mut company, "Amir to Sales");

    println!("{company:#?}");
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, employee: &str) {
    let employee: Vec<&str> = employee.split(" to ").collect();
    let name = employee[0].trim().to_string();
    let department = employee[1].trim().to_string();

    let entry = company.entry(department).or_default();
    entry.push(name);
}
