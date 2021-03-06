/*
Authors: Awildo G., Sosina A., Siqi F., Sam A.
Project: Advent of Code 2019 Day 1
Date: 05/24/2020
Desc: Calculate the required amount of fuel for the given set of modules and their mass.
*/

// we were unable to import math
use std::fs; // to read file
use std::io; // to read file name from input

// Helper function that returns the amount of fuel required for a module of mass mod_mass.
// Algorithm: Take its mass (a u32), divide by three, round down, and subtract 2
fn fuel_alg(mod_mass: i32) -> i32 {
    // Turns the parameter to f64, divides by 3 and rounds down.
    // Then, we subtract 2 and return that value

    // Don't take negative inputs
    if mod_mass < 0 {
        return 0;
    }

    let first_step = (mod_mass / 3) as f64;
    let second_step = first_step.floor();

    (second_step as i32) - 2
}

// Calculates the fuel required 
fn fuel_required(mod_mass: i32) -> i32 {
    let fuel_req: i32 = fuel_alg(mod_mass);
    // negative fuel
    if fuel_req < 0 {
        return 0;
    }
    // positive fuel
    else {
        return fuel_req + fuel_required(fuel_req);
    }
}

// The Main method
fn main() {
    // read the file name
    println!("Enter file name:");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Invalid input");

    // trim function on string doesn't return strings for some reason
    file_name = file_name.trim().to_string();
    // println!("{}", file_name); // for debugging

    // put the whole file into a string (contents)
    let contents = fs::read_to_string(file_name).expect("Oops didn't open");
    // println!("{}", contents); // debugging

    // split contents into a vector containing each line from the input file
    // collect() takes the pieces from split() and puts it into the vector
    let contents: Vec<&str> = contents.split("\n").collect();

    // printing contents for debugging
    // for i in 0..contents.len() {
    //     println!("{}", contents[i]);
    // }

    // our result
    let mut total_fuel = 0;

    // for each line, calculate the fuel required for each module and add it to the total
    for line in contents.iter() {
        // parse() returns a Result type, expect will output an error message if it evaluates to err
        let module_mass: i32 = line.trim().parse().expect("Problems!");

        total_fuel += fuel_required(module_mass);
    }

    println!("Total Fuel Required: {}", total_fuel)
}
