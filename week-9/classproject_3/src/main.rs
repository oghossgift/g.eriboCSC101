use std::fs::File;
use std::io::Write;

fn main() {
    // Header for the output
    let header = "S/N\tNAME OF COMMISSIONER\tMINISTRY\t\tGEOPOLITICAL ZONE\n";

    // Separate datasets stored in arrays
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Print to console
    println!("{}", header);
    for i in 0..commissioners.len() {
        println!(
            "{}\t{}\t{}\t\t{}",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
    }

    // Write to a file
    let mut file = File::create("convicted_ministers.txt").expect("Failed to create file");
    file.write_all(header.as_bytes()).expect("Failed to write to file");

    for i in 0..commissioners.len() {
        let line = format!(
            "{}\t{}\t{}\t\t{}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
        file.write_all(line.as_bytes()).expect("Failed to write to file");
    }

    println!("\nData has been saved to 'convicted_ministers.txt'.");
}
