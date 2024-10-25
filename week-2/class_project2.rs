
fn main() {
    let t: f64 = 450_000.0;
    let m: f64 = 1_500_000.0;
    let h: f64 = 750_000.0;
    let d: f64 = 2_850_000.0;
    let a: f64 = 250_000.0;

    let sum = t * 2.0 + m + h * 3.0 + d * 3.0 + a ;
    let average = sum/10.0;
    println!("The total sum is {}", sum);
    println!("The average total {} ", average);

}
