File Encryptor
A simple file encryption tool built in Rust, featuring a user-friendly GUI powered by Iced. This application allows you to securely encrypt and decrypt local files, using various algorithms for encryption. Additionally, it supports viewing encrypted text files without modifying them.

Features
Encryption Algorithms: Choose between several encryption algorithms, including:
AES-GCM
AES-GCM-SIV
ChaCha20-Poly1305
Key Derivation: Derives encryption keys from user-provided passphrases for secure, consistent encryption.
Metadata Storage: Automatically saves necessary decryption data (algorithm, salt, IV) with the encrypted file.
Encrypted File Viewer: View the contents of an encrypted text file directly in the application without altering the file.
Getting Started
Prerequisites
Rust: Ensure that Rust is installed. You can download it from rust-lang.org.
Iced: This project relies on the Iced GUI library, which will be automatically installed when you build the project.
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/yourusername/file-encryptor.git
cd file-encryptor
Build the project:

bash
Copy code
cargo build --release
Run the application:

bash
Copy code
cargo run --release
Usage
Choose a File: Use the GUI to select a file you want to encrypt or decrypt.
Enter a Passphrase: Input a secure passphrase. This passphrase will be used to derive the encryption key.
Select an Algorithm: Choose your preferred encryption algorithm from the available options.
Encrypt/Decrypt: Click the Encrypt or Decrypt button as needed.
Viewing Encrypted Files
To view an encrypted text file without modifying it:

Select the encrypted file.
Click the "View" button.
The file's contents will be decrypted and displayed within the GUI.
Supported Encryption Algorithms
The application offers three encryption algorithms:

AES-GCM: Known for speed and security.
AES-GCM-SIV: Provides additional security for certain scenarios.
ChaCha20-Poly1305: A secure and efficient alternative to AES, particularly on mobile and low-power devices.
Each algorithm has different performance and security properties, making it easy to tailor the encryption to your specific requirements.

License
This project is licensed under the MIT License - see the LICENSE file for details.

Acknowledgements
Iced GUI Library
RustCrypto for the cryptographic algorithms and utilities
