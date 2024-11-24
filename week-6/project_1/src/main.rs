use std::io;

fn main() {
    println!("Menu");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    println!("Enter the type of food (P, F, A, E, W):");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type: char = food_type.trim().chars().next().unwrap().to_uppercase().next().unwrap();

    println!("Enter the quantity:");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
    let quantity: u32 = quantity_input.trim().parse().expect("Failed to convert to number");

    let price: u32;

    match food_type {
        'P' => price = 3200,
        'F' => price = 3000,
        'A' => price = 2500,
        'E' => price = 2000,
        'W' => price = 2500,
        _ => {
            println!("Invalid food type");
            return;
        }
    }

    let total_cost = price * quantity;

    let discounted_cost = if total_cost > 10000 {
        (total_cost as f64) * 0.95
    } else {
        total_cost as f64
    };

    println!("Total cost: N{}", discounted_cost);
}
