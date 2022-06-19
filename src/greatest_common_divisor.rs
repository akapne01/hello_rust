/// Function that finds the Greates Common Divisor using
/// Euler's algorithm. 
/// - Modify gcd to accept command line argument
/// cargo run 12 24
/// Trait: FromStr. Is a collection of methods that types can implement
/// - traits are similar to interfaces in Java. 
use std::str::FromStr;

/* provide useful fn and types for interacting with execution environment.
* including args which holds cmd line arguments.
*/ 
use std::env; 
pub fn run() {
    println!("\n### In greatest_common_divisor.rs ###");
    // Vector is a growable data type, similar to the List in Python.
    let mut numbers: Vec<u64> = Vec::new();

    /* Process CLI arguments 
    * args() returns iterator. 
    * The first iterator element is always name of the program being run. 
    */
    for arg in env::args().skip(1) {
        /* u64 type implements  FromStr trait, for this reason we can call
         * from_str method on u64 data type. Trait must be in scope in order
         * to use the methods.
         * - from_str returns a Result value
         * value: Ok(v)
         *  - parse succeeded and v is the value produced.
         * Err(e)
         * - parse failed and e is an error value explaining why. 
         * 
         * There are no Exeptions in rust. All  errors are handled using
         * either Result or panic. 
         * 
         * Except method checks the Result of the parse. If results in errors,
         * prints out error code and exits the program immediately. If result 
         * is OK, simply returns the value v itself. 
         * 
         */
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    // To calculate gcd we need 2 numbers. If numbers are not provided, simply exit. 
    if numbers.len() == 0 {
        // Writes out error msg to the std error output stream.
        eprintln!("Usage: cargo run NUMBER1 NUMBER2");
        std::process::exit(1);
    }

    /*
     * uses d as it's running value. It holds gcd of all the numbers
     * we have processed so far.
     */
    let mut d = numbers[0];
    /* Ownership of the vector should remain with numbers. We are merely
     * borrowing its elements for the loop. Achieved by & operator. 
     * &numbers[1..] borrows a reference to the vectors elements from the
     * second element onward. 
     * The for loop iterates over the referenced elements, letting m borrow
     * each element in succession. 
     * The *m operator dereferences m, yielding the value it refers to.
     * - numbers variable owns the Vector, rust frees it when it goes out of 
     * scope at the end of the blcok where it is defined. 
     * TLDR: &x borrows reference to x. *r us the value that the reference r
     * refers to. 
     */
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    /* Prints out to the std output stream.
     * Macro that takes template string, substitutes formated versions
     * of the remaining arguments for the (...} forms as they appear in
     * template string, and writes result to std output stream.
     */
    println!("\nGreatest Common Divisor of {:?} is {}", numbers, d);
}

/// For function parameters and return values, the type needs to be defined.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    //! compiler infers variable types within the fn bodies
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // If the fn body ends with expression that is not followed by 
    // semicolon, that's the fn return value
    n
}

#[test]
fn test_gcd() {
    //! when cargo run, the test is skipped
    //! cargo test runs all the unit tests
    //! #[test] is example of an attribute. 
    //! - Attribute is an open ended system for marking fn and 
    //! other declarations with extra info.
    //! Works similar to annotations in Java.
    //! Attributes are used to control compiler warnings and code
    //! style checks, include code conditionally, tell rust how to
    //! interact with code written in other languages and so on.
    println!("\nRunning unit tests for gcd function!");
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(12 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);

}