/// Function that finds the Greates Common Divisor using
/// Euler's algorithm. 

pub fn run() {
    println!("\n### In greatest_common_divisor.rs ###");
    let n1 = 12;
    let n2 = 24;
    let result = gcd(n1, n2);
    println!("\nGreatest Common Divisor of {} and {} is {}", n1, n2, result);
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