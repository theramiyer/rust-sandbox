// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ram";
    let mut age = 31;
    println!("My name is {} and I am {}", name, age);
    age = 32;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Ram", 31);
    println!("{} is {}", my_name, my_age)
}