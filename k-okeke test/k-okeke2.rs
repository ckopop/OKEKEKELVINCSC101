use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter Your number of papers published:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Fix variable name (no spaces) and parse the input
    let number_of_paper: f32 = input.trim().parse().expect("Not a valid number");

    // Incentive calculation based on the number of papers
    if number_of_paper >= 3.0 && number_of_paper <= 5.0 {
        println!("Incentive: 500000");
    } 
    else if number_of_paper > 5.0 && number_of_paper <= 10.0 {
        println!("Incentive: 800000");
    }
    else if number_of_paper > 10.0 {
        println!("Incentive: 1000000");
    } 
    else {
        println!("Incentive: 100000");
    }
}
