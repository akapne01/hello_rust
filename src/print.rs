pub fn run() {
    // Print to console
    println!("\nIn print.rs file");
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("Number: {}", 1);
    println!("{} - {}", "Aga", "Paga");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Agi", "LV", "code");

    // Named Arguments
    println!("{name} likes to {activity}", name = "Agi", activity = "code" );

    // Test Autosave
    println!("Autosaved!");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Maths
    println!("10 + 10 = {}", 10 + 10);
}