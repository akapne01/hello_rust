/// Variables hold primitive data or references to data.
/// Variables are immutable by default.
/// Adding mut keyword allows to make variable mutable. 
/// Rust is a block-scoped langauge.
/// Const keyword allows to create a constant, type needs to be explicitly defined.
/// Convention for const is to be all upercase.

pub fn run() {
    println!("\nIn the vars.rs file");
    
    let name = "Agi";
    let mut number = 7;
    println!("My name is {} and I like number {}.", name, number);
    number = 9;
    println!("My name is {} and I like number {}.", name, number);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple Variables
    let (my_name, my_number) = ("Agi", 9);
    println!("{} likes number {}", my_name, my_number)


}