
//Question 1
use std::io;

fn main() {
    // Variables to hold user inputs
    let mut position_input = String::new();
    let mut class_input = String::new();
    let mut cgpa_input = String::new();

    // Getting the user's position
    println!("Enter your position (e.g., 'class rep'):");
    io::stdin().read_line(&mut position_input).expect("Failed to read line");
    
    // Getting the user's class level (e.g., 100, 200, etc.)
    println!("Enter your class level (e.g., 100, 200, etc.):");
    io::stdin().read_line(&mut class_input).expect("Failed to read line");

    // Getting the user's CGPA
    println!("Enter your CGPA:");
    io::stdin().read_line(&mut cgpa_input).expect("Failed to read line");

    // Trim the inputs and parse them
    let position = position_input.trim();
    let class: i32 = class_input.trim().parse().expect("Invalid class input");
    let cgpa: f32 = cgpa_input.trim().parse().expect("Invalid CGPA input");

    // Check if the user is eligible to vote based on position, class, and CGPA
    if position == "class rep" {
        // If position is "class rep", the user is eligible to vote
        println!("You can vote as class rep.");
        println!("Name:_____________ ");
        println!("Email:_____________ _____________  ");
        println!("Department:_____________  ");
        println!("State of origin:_____________ ");
    } else if class >= 200 {
        // If the user is in class level 200 or higher, they can vote
        println!("You can vote based on your class level.");
        println!("Name: _____________ ");
        println!("Email:_____________ ");
        println!("Department:_____________  ");
        println!("State of origin:_____________  ");
    } else if cgpa >= 4.0 {
        // If the user has a CGPA of 4.0 or higher, they can vote
        println!("You can vote based on your CGPA.");
        println!("Name:_____________  ");
        println!("Email: _____________ ");
        println!("Department:_____________  ");
        println!("State of origin: _____________ ");
    } else {
        // If none of the conditions are met, the user is not eligible to vote
        println!("You are not eligible to vote");
    }
}


//Question 2
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
