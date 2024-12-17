use std::io::Write;
fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all("Nigerian Brewries Plc Drink Categories\n".as_bytes()).expect("Write failed");
    
    file.write_all("\nLager:\n".as_bytes()).expect("Write failed");
    for drink in lager {
        file.write_all((drink.to_string() + "\n" ).as_bytes()).expect("Failed to write to file");
    }
    file.write_all("\nStout:\n".as_bytes()).expect("Write failed");
    for drink in stout {
        file.write_all((drink.to_string() + "\n").as_bytes()).expect("Failed to write to file");
    }
    file.write_all("\nNon-Alcoholic:\n".as_bytes()).expect("Write failed");
    for drink in non_alcoholic {
        file.write_all((drink.to_string() + "\n").as_bytes()).expect("Failed to write to file");
    }

    println!("\nDrinks data written to 'drinks.txt'.");
}


