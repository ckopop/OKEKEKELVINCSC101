use std::{f32::consts::PI, io};

fn main() {
    let calc_type = read_line("Choose a type of calculator (Enter Code):
    T: Area of Trapezium
    R: Area of Rhombus
    P: Area of Parallelogram
    C: Area of Cube
    V: Volume of Cylinder");

    match calc_type.as_str() {
        "T" => area_trapezium(),
        "R" => area_rhombus(),
        "P" => area_parallelogram(),
        "C" => area_cube(),
        "V" => volume_cylinder(),
        _ => panic!("Invalid calculator type"),
    }
}

fn area_trapezium() {
    println!("Calculating Area of Trapezium: (height/2)*(base1+base2)");
    let height: f32 = read_line("Enter height:").parse().expect("height should be a number");
    let base1: f32 = read_line("Enter base1:").parse().expect("base1 should be a number");
    let base2: f32 = read_line("Enter base2:").parse().expect("base2 should be a number");
    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

fn area_rhombus() {
    println!("Calculating Area of Rhombus: (1/2)*(diagonal1*diagonal2)");
    let diagonal1: f32 = read_line("Enter diagonal1:").parse().expect("diagonal1 should be a number");
    let diagonal2: f32 = read_line("Enter diagonal2:").parse().expect("diagonal2 should be a number");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {}", area);
}

fn area_parallelogram() {
    println!("Calculating Area of Parallelogram: base * altitude");
    let base: f32 = read_line("Enter base:").parse().expect("base should be a number");
    let altitude: f32 = read_line("Enter altitude:").parse().expect("altitude should be a number");
    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

fn area_cube() {
    println!("Calculating Area of Cube: 6 * (length of side)^2");
    let length: f32 = read_line("Enter length of side:").parse().expect("length of side should be a number");
    let area = 6.0 * length * length;
    println!("Area of Cube = {}", area);
}

fn volume_cylinder() {
    println!("Volume of Cylinder: pi * (radius^2) * height");
    let radius: f32 = read_line("Enter radius:").parse().expect("radius should be a number");
    let height: f32 = read_line("Enter height:").parse().expect("height should be a number");
    let volume = PI * radius * radius * height;
    println!("Volume of Cylinder = {}", volume);
}

fn read_line(hint: &str) -> String {
    println!("{}", hint);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}