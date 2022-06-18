/// There are 2 types of Strings.
/// 1) Primitive Strings (str) and String
///     - immutable
///     - fixed-length
///     - Located somewhere in memory.
/// 2) String defined as mut
///     - mutable
///     - Growable
///     - Located in Heap memory
///     - Use when need to modify or own the str data.
/// 
pub fn run() {
    println!("\n### In strings.rs ###");

    // Primitive str
    let hello = "Hello";
    println!("\nPrimitive String (str): {}", hello);

    // String
    let mut hello_mutable  = String::from("Hello ");
    println!("Mutable string: {}", hello_mutable);

    // Get length
    println!("str Length: {}", hello.len());
    println!("String Length: {}", hello_mutable.len());

    // Append char to a mutable string
    hello_mutable.push('W');
    println!("After char 'W' was pushed String now looks: {}", hello_mutable);

    // Append string to a mutable string
    hello_mutable.push_str("orld!");
    println!("After String was pushed variable now is as follows: {}", hello_mutable);
    
    // Capacity in bytes
    println!("Capacity: {}", hello_mutable.capacity());

    // Is String Empty? 
    println!("Is Empty: {}", hello_mutable.is_empty());

    // Check if String contains a substring
    println!("Contains 'World': {}", hello_mutable.contains("World"));

    // Replace
    println!("Replace: {}", hello_mutable.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_mutable.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hello_mutable);

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    /* 
    Assertion Testing. Only prints out error when fails.
    If asserion passes then no action is taken. 
    */
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    
    println!("{}", s);

}