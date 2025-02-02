use std::io;

fn main() {
    println!("Welcome to Group 14's Carbon Footprint Calculator!");

    // Car Carbon Footprint
    println!("Enter the distance driven (in kilometers): ");
    let mut distance1 = String::new();
    io::stdin().read_line(&mut distance1).expect("Failed to read input");
    let distance: f64 = distance1.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Indicate fuel type using numbers (1: Petrol, 2: Diesel): ");
    let mut fuel_type = String::new();
    io::stdin().read_line(&mut fuel_type).expect("Failed to read input");
    let fuel_type: u32 = fuel_type.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter your car's fuel efficiency (liters per 100 km): ");
    let mut fuel_efficiency = String::new();
    io::stdin().read_line(&mut fuel_efficiency).expect("Failed to read input");
    let fuel_efficiency: f64 = fuel_efficiency.trim().parse().expect("Invalid input. Please enter a number.");

    let mut emission_factor: f64 = 0.0;

    if fuel_type == 1 {
        emission_factor = 2.31; // Petrol (kg CO2 per liter)
    } else if fuel_type == 2 {
        emission_factor = 2.68; // Diesel (kg CO2 per liter)
    } else {
        println!("Invalid fuel type selected.");
        return;
    }

    let fuel_used = (distance * fuel_efficiency) / 100.0;
    let car_carbon_footprint = fuel_used * emission_factor;

    println!("\nCar Carbon Footprint:");
    println!("Distance Driven: {:.2} km", distance);
    println!("Fuel Used: {:.2} liters", fuel_used);
    println!("Carbon Footprint: {:.2} kg CO2", car_carbon_footprint);

    // Phone Carbon Footprint
    println!("\nEnter your daily phone usage (in hours): ");
    let mut phone_usage = String::new();
    io::stdin().read_line(&mut phone_usage).expect("Failed to read input");
    let phone_usage: f64 = phone_usage.trim().parse().expect("Invalid input. Please enter a number.");

    let phone_emission_factor = 0.018; // kg CO2 per hour (typical smartphone)
    let yearly_phone_emissions = phone_usage * phone_emission_factor * 365.0;

    println!("\nPhone Carbon Footprint:");
    println!("Daily Usage: {:.2} hours", phone_usage);
    println!("Yearly Carbon Footprint: {:.2} kg CO2", yearly_phone_emissions);

    // Laptop Carbon Footprint
    println!("\nEnter your daily laptop usage (in hours): ");
    let mut laptop_usage = String::new();
    io::stdin().read_line(&mut laptop_usage).expect("Failed to read input");
    let laptop_usage: f64 = laptop_usage.trim().parse().expect("Invalid input. Please enter a number.");

    let laptop_emission_factor = 0.05; // kg CO2 per hour (typical laptop)
    let yearly_laptop_emissions = laptop_usage * laptop_emission_factor * 365.0;

    println!("\nLaptop Carbon Footprint:");
    println!("Daily Usage: {:.2} hours", laptop_usage);
    println!("Yearly Carbon Footprint: {:.2} kg CO2", yearly_laptop_emissions);

    // Summary
    println!("\n--- Summary of Carbon Footprints ---");
    println!("Car Carbon Footprint (per {} km): {:.2} kg CO2", distance1, car_carbon_footprint);
    println!("Phone Carbon Footprint (Yearly): {:.2} kg CO2", yearly_phone_emissions);
    println!("Laptop Carbon Footprint (Yearly): {:.2} kg CO2", yearly_laptop_emissions);
    println!("-------------------------------------");
}
