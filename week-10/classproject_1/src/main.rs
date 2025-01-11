struct Laptop{
    brand:String,
    price:u32
}

impl Laptop{
    fn total_cost(&self, quantity: u32) ->u32{
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop{
        brand:String::from("HP"),
        price: 650_000
    };

    let ibm = Laptop{
        brand:String::from("IBM"),
        price: 755_000
    };

    let toshiba = Laptop{
        brand:String::from("Toshiba"),
        price: 550_000
    };

    let dell = Laptop{
        brand: String::from("Dell"),
        price: 850_000
    };

    //cost for e4 laptops from each brand
    let hp_total = hp.total_cost(3);
    let ibm_total = ibm.total_cost(3);
    let toshiba_total = toshiba.total_cost(3);
    let dell_total = dell.total_cost(3);

    //Calculating final cost
    let final_total = hp_total + ibm_total + toshiba_total + dell_total;

    println!("Total cost for 3 HP laptops: ₦{}", hp_total);
    println!("Total cost for 3 IBM laptops: ₦{}", ibm_total);
    println!("Total cost for 3 Toshiba laptops: ₦{}", toshiba_total);
    println!("Total cost for 3 Dell laptops: ₦{}", dell_total);
    println!("Final total cost: ₦{}", final_total);

}
