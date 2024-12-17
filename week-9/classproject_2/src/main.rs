use std::fs::File;
use std::io::Write;

fn main() {
    let header = "PAU SMIS\n\nStudent Name\tMatric. Number\tDepartment\tLevel\n";

    // Students' details as a 2D array
    let students = [
        ["Oluchi Mordi", "ACC1011111", "Accounting", "300"],
        ["Adams Aliyu", "ECO1011011", "Economics", "200"],
        ["Shania Bolade", "CSC1032882", "Computer", "200"],
        ["Adekunle Gold", "EEE1020202", "Electrical", "100"],
        ["Blanca Edemoh", "MEE1020201", "Mechanical", "100"],
    ];

    // Print to console
    println!("{}", header);
    for student in students.iter() {
        println!("{}\t{}\t{}\t{}", student[0], student[1], student[2], student[3]);
    }

    // Write to a file
    let mut file = File::create("students.csv").expect("Failed to create file");
    file.write_all(header.as_bytes()).expect("Failed to write to file");

    for student in students.iter() {
        let line = format!("{}\t{}\t{}\t{}\n", student[0], student[1], student[2], student[3]);
        file.write_all(line.as_bytes()).expect("Failed to write to file");
    }

    println!("\nStudent data has been saved to 'students.txt'.");
}
