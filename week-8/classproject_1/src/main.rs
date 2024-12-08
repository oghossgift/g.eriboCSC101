
use std::io;

fn main() {
   
    let public_servant = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let office_admin = ["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = ["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = ["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = ["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    // Get user input for role
    let mut role = String::new();
    println!("Enter your role:");
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim(); // Remove extra spaces or newlines

    // Get user input for years of experience
    let mut experience_input = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience: i32 = experience_input.trim().parse().expect("Invalid input");

    // Default position
    let mut position = "Not Found";

    for i in 0..office_admin.len() {
        if role == office_admin[i] {
            if i == 0 && experience >= 1 && experience <= 2 {
                position = public_servant[0];
            } else if i == 1 && experience >= 3 && experience <= 5 {
                position = public_servant[1];
            } else if i == 2 && experience >= 5 && experience <= 8 {
                position = public_servant[2];
            } else if i == 3 && experience >= 8 && experience <= 10 {
                position = public_servant[3];
            } else if i == 4 && experience >= 10 && experience <= 13 {
                position = public_servant[4];
            } else if i == 5 && experience > 13 {
                position = public_servant[5];
            }
        }
    }

    for i in 0..academic.len() {
        if role == academic[i] {
            if i == 0 && experience >= 1 && experience <= 2 {
                position = public_servant[0];
            } else if i == 1 && experience >= 3 && experience <= 5 {
                position = public_servant[1];
            } else if i == 2 && experience >= 5 && experience <= 8 {
                position = public_servant[2];
            } else if i == 3 && experience >= 8 && experience <= 10 {
                position = public_servant[3];
            } else if i == 4 && experience >= 10 && experience <= 13 {
                position = public_servant[4];
            } else if i == 5 && experience > 13 {
                position = public_servant[5];
            }
        }
    }

    for i in 0..lawyer.len() {
        if role == lawyer[i] {
            if i == 0 && experience >= 1 && experience <= 2 {
                position = public_servant[0];
            } else if i == 1 && experience >= 3 && experience <= 5 {
                position = public_servant[1];
            } else if i == 2 && experience >= 5 && experience <= 8 {
                position = public_servant[2];
            } else if i == 3 && experience >= 8 && experience <= 10 {
                position = public_servant[3];
            } else if i == 4 && experience >= 10 && experience <= 13 {
                position = public_servant[4];
            } else if i == 5 && experience > 13 {
                position = public_servant[5];
            }
        }
    }

    for i in 0..teacher.len() {
        if role == teacher[i] {
            if i == 0 && experience >= 1 && experience <= 2 {
                position = public_servant[0];
            } else if i == 1 && experience >= 3 && experience <= 5 {
                position = public_servant[1];
            } else if i == 2 && experience >= 5 && experience <= 8 {
                position = public_servant[2];
            } else if i == 3 && experience >= 8 && experience <= 10 {
                position = public_servant[3];
            } else if i == 4 && experience >= 10 && experience <= 13 {
                position = public_servant[4];
            } else if i == 5 && experience > 13 {
                position = public_servant[5];
            }
        }
    }

    // Display the result
    if position == "Not Found" {
        println!(
            "The role '{}' with {} years of experience does not match any defined category.",
            role, experience
        );
    } else {
        println!(
            "\nThe role '{}' with {} years of experience corresponds to position {}.",
            role, experience, position
        );
    }
}
