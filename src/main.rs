use md5;
use rpassword::read_password;

fn main() {
    println!("Enter your password:");

    // Read the password from the user without echoing it to the terminal
    let password = read_password().expect("Failed to read password");

    // Compute the MD5 hash of the password
    let hash = md5::compute(password);

    // Print the resulting hash as a hexadecimal string
    println!("MD5 Hash: {:x}", hash);
}