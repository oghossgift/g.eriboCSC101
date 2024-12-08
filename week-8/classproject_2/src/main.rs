use std::io;
fn main() {
    let mut candidates: Vec<(String,i32)> = Vec::new();

    let mut input1 = String::new();
    println!("Enter the number of candidates");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let cand_num:i32=input1.trim().parse().expect("Invalid input");

    for i in 0..cand_num {
        let mut name = String::new();
        let mut experience = String::new();
    
        println!("Enter the name of candidate {}:", i + 1); // Use `i + 1` for user-friendly numbering
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name= name.trim().to_string();

        println!("Enter the years of programming experience for {}:", name);
        io::stdin().read_line(&mut experience).expect("Failed to read input");
        let experience: i32 = experience.trim().parse().expect("Please enter a valid number");
    
        candidates.push((name, experience));
    }
    let mut top_name = String::new();
    let mut top_experience = 0;

    for candidate in candidates {
        if candidate.1 > top_experience {
            top_name = candidate.0;
            top_experience = candidate.1;
        }
    }

   
    println!(
        "The candidate with the highest programming experience is {} with {} years of experience.",
        top_name, top_experience
    );
}
