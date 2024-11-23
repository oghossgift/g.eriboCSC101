use std::io;
fn main() {
    println!("Restaurant Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut input = String::new();
    println!("Input your food choice(s) (P, F, A, E, W): ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let choice = input.trim();

    let mut price:i32 = 0;
    if choice == "P"{
       price = 3200;
    }else if choice == "F"{
        price = 3000;
    }else if choice == "A"{
        price = 2500;
    }else if choice == "E"{
        price = 2000;
    }else if choice == "W" {
        price = 2500;
    } else {
        println!("Invalid food type selected!");
    }

    let mut quantity_input = String::new();
    println!("Enter the quantity:");
    io::stdin().read_line(&mut quantity_input).expect("Not a valid string");
    let quantity:i32 =quantity_input.trim().parse().expect("Not a valid number");

    let mut total:i32 = price * quantity;

   
    if total > 10000 {
        let discount = total * 5/100; // Calculate 5% discount using integer math
        total -= discount;             // Apply the discount
        println!("A 5% discount of N{} has been applied!", discount);
    }
    

   println!("Total cost: N{}", total);
}
