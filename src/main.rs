#![allow(unused_variables)]
#![allow(non_snake_case)]
use std::io;
use colored::*;

fn main() {
    let mut get_input = String::new();
    println!("Welcome!");
    println!("Enter your operation that you want to perform!\n1.Add\n2.Substract\n3.Divide\n4.Multiply");
    io::stdin().read_line(&mut get_input).expect("Error!");
    match get_input.trim() {
        "Add" => {
            add()
            
        }

        "Subtract" => {
            subtract()
        }


        "Divide" => {
            divide()
        }

        "Multiply" => {
            multiply()
        }

        _ => println!("{}", "Invalid input!".red())
    }
}



fn add() {
    let mut first_num_add = String::new();
    let mut second_num_add = String::new();
    println!("Enter your first number to add:");
    io::stdin().read_line(&mut first_num_add).expect("Error!");
    let first_num_add: u32 = match first_num_add.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!"),
    };
    println!("Enter your second number to add:");
    io::stdin().read_line(&mut second_num_add).expect("Error");
    let second_num_add: u32 = match second_num_add.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!")
    };
    let result: u32 = first_num_add + second_num_add;
    println!("The sum is {}", result)
}
fn subtract() {
    let mut first_num_subtract = String::new();
    let mut second_num_subtract = String::new();
    println!("Enter your first number to add:");
    io::stdin().read_line(&mut first_num_subtract).expect("Error!");
    let first_num_subtract: u32 = match first_num_subtract.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!"),
    };
    println!("Enter your second number to subtract:");
    io::stdin().read_line(&mut second_num_subtract).expect("Error");
    let second_num_subtract: u32 = match second_num_subtract.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!")
    };
    let result: u32 = first_num_subtract - second_num_subtract;
    println!("The result is {}", result)
}
fn divide() {
    let mut first_num_divide = String::new();
    let mut second_num_divide = String::new();
    println!("Enter your first number to divide:");
    io::stdin().read_line(&mut first_num_divide).expect("Error!");
    let first_num_divide: u32 = match first_num_divide.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!"),
    };
    println!("Enter your second number to divide:");
    io::stdin().read_line(&mut second_num_divide).expect("Error");
    let second_num_divide: u32 = match second_num_divide.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!")
    };
    let result: u32 = first_num_divide / second_num_divide;
    println!("The result is {}", result)
}
fn multiply() {
    let mut first_num_multiply = String::new();
    let mut second_num_multiply = String::new();
    println!("Enter your first number to multiply:");
    io::stdin().read_line(&mut first_num_multiply).expect("Error!");
    let first_num_multiply: u32 = match first_num_multiply.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!"),
    };
    println!("Enter your second number to multiply:");
    io::stdin().read_line(&mut second_num_multiply).expect("Error");
    let second_num_multiply: u32 = match second_num_multiply.trim().parse(){
        Ok(v) => v,
        Err(error) => panic!("Invalid!")
    };
    let result: u32 = first_num_multiply * second_num_multiply;
    println!("The result is {}", result)
}