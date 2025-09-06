// A simple Rust program that reads a password from the user and prints an MD5 hash. The purpose is to demonstrate
// that MD5 is NOT a hashing algorithm meant for password hashing.

// Loading necessary crates.
use md5;
use rpassword::read_password;
use std::fs::File;
use std::io::Write;

// Main function.
fn main() {

    // Prompt the user for a password.
    println!("Enter your password:");

    // Read the password from the user without echoing it to the terminal.
    let password = read_password().expect("Failed to read password");

    // Compute the MD5 hash of the password.
    let hash = md5::compute(password);

    // Print the resulting hash.
    println!("MD5 Hash: {:x}", hash);

    // Write the hash to a file called hash.txt
    let mut file = File::create("hash.txt").expect("Failed to create file");
    writeln!(file, "{:x}", hash).expect("Failed to write to file");

    println!("The hash has been written to hash.txt");
}