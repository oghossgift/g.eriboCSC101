use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a : ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b : ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c : ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    //Calculate the discriminant 
    let d: f32 = b.powi(2) -( 4.0 * a * c);

    //Check if the discriminant is negative
    if d > 0.0{
        let root1 =-b + d.sqrt()/(2.0 * a);
        let root2 =-b - d.sqrt()/(2.0 * a);
        println!("The discriminant has two distint roots: root1 = {} and root2 = {}", root1, root2);

    } else if d == 0.0{ 
        let root = -b / (2.0 * a);
        println!("There is one real root: root = {} ",root);
    
    }else {
        println!("The equation has no real root");
    }
    
}