use std::io;

struct Device{
    name:String,
    price:f32,
    quantity:u32,
}

fn main(){
    let mut inventory = vec![
        Device{
            name:"HP laptop".to_string(),
            price:650_000.0,
            quantity:10,
        },
        Device{
            name:"IBM laptop".to_string(),
            price:755_000.0,
            quantity:6,
        },
        Device{
            name:"Toshiba laptop".to_string(),
            price:550_000.0,
            quantity:10,
        },
        Device{
            name:"Dell laptop".to_string(),
            price:850_000.0,
            quantity:4,
        },
    ];

    let mut cart:Vec<Device> = Vec::new();
    println!("Welcome to our store!");
    loop{
        let choice:u32 = read_line("
 Please enter an option (1-4):
    1. List all devices
    2. Add a device to your cart
    3. Remove a device from your cart
    4. Checkout
",).parse().expect("Invalid input");

        match choice{
            1 => list_devices(&inventory),
            2 => add_to_cart(&mut inventory, &mut cart),
            3 => remove_from_cart(&mut inventory, &mut cart),
            4 =>{
                checkout(&mut cart);
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn checkout(cart:&mut Vec<Device>){
    let mut total = 0.0;
    for device in cart.iter(){
        total += device.price * device.quantity as f32;
    }
    list_devices(cart);
    println!("Total: ${:.2}", total);
}

fn remove_from_cart(inventory:&mut Vec<Device>, cart:&mut Vec<Device>){
    println!("Your cart:");
    list_devices(cart);
    let choice:u32 = read_line("Enter the device number to remove:").parse().unwrap();
    let idx = choice as usize - 1;
    if idx >= cart.len(){
        println!("Invalid choice");
        return;
    }
    let device = &mut cart[idx];
    let quantity:u32 = read_line(
        format!("Please select quantity of {} to remove from your cart. Available units:{}", device.name, device.quantity).as_str(),
        ).parse().unwrap();

    if quantity > device.quantity{
        println!("Not enough units in your cart");
        return;
    }

    device.quantity -= quantity;
    for inv_device in inventory{
        if device.name == inv_device.name{
            inv_device.quantity += quantity;
            break;
        }
    }
    println!("{} units of {} removed from your cart", quantity, device.name);
}

fn add_to_cart(inventory:&mut Vec<Device>, cart:&mut Vec<Device>){
    println!("Please select a device to add to your cart:");
    list_devices(inventory);
    let choice:u32 = read_line("Choose 1-4").parse().unwrap();
    let idx = choice as usize - 1;
    if idx >= inventory.len() {
        println!("Invalid choice");
        return;
    }
    let device = &mut inventory[idx];
    let quantity:u32 = read_line(
        format!("Please select quantity of {} to add to your cart. Available units: {}", device.name, device.quantity).as_str(),
        ).parse().unwrap();

    if quantity > device.quantity{
        println!("Not enough units available");
        return;
    }
    device.quantity -= quantity;
    cart.push(Device{
        name:device.name.clone(),
        price:device.price,
        quantity,
    });
    println!("{} units of {} added to your cart", quantity, device.name);
}

fn list_devices(inventory:&Vec<Device>){
    let mut i = 1;
    for device in inventory{
        println!("{}. {} - ${:.2} - {}", i, device.name, device.price, device.quantity);
        i += 1;
    }
}

fn read_line(prompt:&str) -> String{
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}