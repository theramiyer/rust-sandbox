pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("Number: {}", 1);

    // basic formatting 2
    println!("{} is from {}", "Ram", "Chennai");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Ram", "Chennai", "code");

    // named arguments
    println!("{name} likes to {activity}", name = "Ram", activity = "write");

    // placeholder traits
    println!("Binary: {:b} Hexadecimal: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "Hello!"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}