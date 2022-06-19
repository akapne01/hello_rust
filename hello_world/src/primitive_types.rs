/// Primitive Types in Rust
/// - Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
/// - u is for unsigned integers, i is for regular integers. 
/// - unsigned means non-negative. The negative values are not allowed. Natural Numbers.
/// Floats: f32, f64
/// Boolean (bool)
/// Characters (char)
/// Tuples
/// Arrays - fixed length (non-primitive type vector is similar to growable arreays)
/// 
/// Rust us a statically typed language. It means that it must know the types of all variables at
/// compile time. However, the compiler can usually unfer what type we want to use based on the value
/// and how we use it. For this reason, it is not required to define type for each variable. 

pub fn run() {
    println!("\nIn primitive_types.rs");

    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type 
    let z: i64 = 79795445;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    
    // Boolean
    let is_active = true;
    // {:?} is debug placeholder.
    println!("{:?}", (x, y, z, is_active));

    // Get Boolean from expression
    let is_greater: bool = 10 > 5;
    let is_smaller: bool = 10 < 5;

    // for char use single quotes
    let a1 = 'a';
    let face = '\u{1F600}';
    
    println!("{:?}", (is_greater, is_smaller, a1, face));


}