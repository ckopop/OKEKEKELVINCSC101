use std::io;

fn main() {
    println!("Enter the experience (1 for experienced, 0 for inexperienced):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience: u32 = experience_input.trim().parse().expect("Failed to convert to number");

    println!("Enter the age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Failed to convert to number");

    let incentive: u32;

    if experience == 1 {
        if age >= 40 {
            incentive = 1560000;
        } else if age >= 30 {
            incentive = 1480000;
        } else {
            incentive = 1300000;
        }
    } else {
        incentive = 100000;
    }

    println!("The annual incentive is N{} per month.", incentive);
}
