# Judge Dredd MD5 Bruteforce Demo

<p align="center">
        <img src="https://github.com/user-attachments/assets/e52ba8a0-3fec-49e6-a144-4c417e25b596" width="300">
</p>

## About

This program demonstrates how quickly MD5 password hashes can be bruteforced. It is designed for educational purposes to highlight the importance of using strong, secure hashing algorithms and passwords.

## Disclaimer

This project is for educational purposes only. Do not use this tool for malicious purposes. Always ensure you have proper authorization before testing the security of any system.

## Features

- Demonstrates MD5 hash bruteforcing.
- Highlights the importance of secure password practices.

## Requirements

- Rust
- Required libraries (see `Cargo.toml`)

## Usage

1. Clone the repository:
     ```bash
     git clone https://github.com/chief-thunder/md_dread.git
     ```
2. Build the package (compiles the application):
     ```bash
     cargo build
     ```
3. Run the program:
     ```bash
     ./target/debug/md_dread
     ```
4. Bruteforce with John the ripper.
     ```bash
     john --format=raw-md5 hash.txt
     ```

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
```  
