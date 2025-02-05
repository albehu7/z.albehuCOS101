use std::io;

fn database_structure() {
    println!("Database Structure: [Tables: users, projects, staff, customers, vendors, data_plans]");
}

fn project_table() {
    println!("Project Table Structure: [id, name, manager_id, start_date, end_date]");
}

fn staff_table() {
    println!("Staff Table Structure: [id, name, position, salary, department]");
}

fn customer_table() {
    println!("Customer Table Structure: [id, name, email, phone, purchase_history]");
}

fn vendor_table() {
    println!("Vendor Table Structure: [id, company_name, contact_person, service_type]");
}

fn main() {
    println!("Enter your role (admin, project_manager, employee, customer, vendor):");

    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");

    let role = role.trim(); // Remove trailing newline

    match role {
        "admin" => database_structure(),
        "project_manager" => project_table(),
        "employee" => staff_table(),
        "customer" => customer_table(),
        "vendor" => vendor_table(),
        _ => println!("Invalid role!"),
    }
}