use std::io;
fn calculate_trapezium() {
    let mut input1 = String::new();
    println!("Enter the height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height: f64 = input1.trim().parse().expect("Failed to read number");

    let mut input2 = String::new();
    println!("Enter base1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1: f64 = input2.trim().parse().expect("Failed to read number");

    let mut input3 = String::new();
    println!("Enter base2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2: f64 = input3.trim().parse().expect("Failed to read number");

    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}
// Function to calculate the area of a rhombus
fn calculate_rhombus() {
    let mut input1 = String::new();
    println!("Enter diagonal1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1: f64 = input1.trim().parse().expect("Failed to read number");

    let mut input2 = String::new();
    println!("Enter diagonal2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2: f64 = input2.trim().parse().expect("Failed to read number");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {}", area);
}
// Function to calculate the area of a parallelogram
fn calculate_parallelogram() {
    let mut input1 = String::new();
    println!("Enter the base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Failed to read number");

    let mut input2 = String::new();
    println!("Enter the altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Failed to read number");

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

// Function to calculate the area of a cube
fn calculate_cube() {
    let mut input1 = String::new();
    println!("Enter the length of the side:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side: f64 = input1.trim().parse().expect("Failed to read number");

    let area = 6.0 * side.powi(2);
    println!("The area of the cube is: {}", area);
}

// Function to calculate the volume of a cylinder
fn calculate_cylinder() {
    let mut input1 = String::new();
    println!("Enter the radius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Failed to read number");

    let mut input2 = String::new();
    println!("Enter the height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Failed to read number");

    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("The volume of the cylinder is: {}", volume);
}
fn main(){
    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    
    //user choice
    let mut choice = String::new();
    println!("Enter your choice (1-5):");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: i32 = choice.trim().parse().expect("Not a valid number");

    if choice == 1 {
        calculate_trapezium();
    } else if choice == 2 {
        calculate_rhombus();
    } else if choice == 3 {
        calculate_parallelogram();
    } else if choice == 4 {
        calculate_cube();
    } else if choice == 5 {
        calculate_cylinder();
    }else {
        println!("Invalid choice. Please select a valid option.");
    }
 }




