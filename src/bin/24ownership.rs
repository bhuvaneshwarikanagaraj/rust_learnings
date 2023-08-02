fn main() {
    // Example 1: Integer Ownership
    let x = 42; // x is the owner of the integer value 42
    println!("Value of x: {}", x);

    // Example 2: String Ownership
    let mut s = String::from("Hello"); // s is the owner of the String "Hello"
    s.push_str(", world!"); // We can modify the String as it is mutable
    println!("Value of s: {}", s);

    // Example 3: Ownership Transfer
    let s1 = String::from("Rust"); // s1 is the owner of the String "Rust"
    let s2 = s1; // s2 takes ownership of the String, and s1 is no longer valid
    // println!("Value of s1: {}", s1); // Error! s1 is no longer valid
    println!("Value of s2: {}", s2);

    // Example 4: Borrowing and References
    let s3 = String::from("Borrowing");
    let s4 = &s3; // s4 is a reference to s3, and s3 is still the owner
    println!("Value of s3: {}", s3);
    println!("Value of s4: {}", s4);
}