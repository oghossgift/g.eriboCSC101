use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Enter your role (admin, project_manager, employee, customer, vendor):");

    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim();

    if role == "admin" {
        show_database_structure();
    } else if role == "project_manager" {
        show_projects_table();
    } else if role == "employee" {
        show_staff_table();
    } else if role == "customer" {
        show_customer_table();
    } else if role == "vendor" {
        show_dataplan_table();
    } else {
        println!("Invalid role! Please enter a valid role.");
    }
}

fn show_database_structure() {
    read_file("globacom_dbase.sql");
}

fn show_projects_table() {
    read_file("projects_tb.sql");
}

fn show_staff_table() {
    read_file("staff_tb.sql");
}

fn show_customer_table() {
    read_file("customer_tb.sql");
}

fn show_dataplan_table() {
    read_file("dataplan_tb.sql");
}

fn read_file(filename: &str) {
    let mut file = File::open(filename).unwrap_or_else(|_| {
        println!("Error: Could not open file {}", filename);
        std::process::exit(1);
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_| {
        println!("Error: Could not read file {}", filename);
        std::process::exit(1);
    });

    print!("{}", contents);
}
