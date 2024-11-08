# File Encryptor
A simple file encryption tool built in Rust, featuring a user-friendly GUI powered by Iced. This application allows you to securely encrypt and decrypt local files, using various algorithms for encryption. Additionally, it supports viewing encrypted text files without modifying them.

# Features
**Encryption Algorithms**: Choose between several encryption algorithms, including:
- AES-GCM
- AES-GCM-SIV
- ChaCha20-Poly1305
**Key Derivation**: Derives encryption keys from user-provided passphrases for secure, consistent encryption.
**Metadata Storage**: Automatically saves necessary decryption data (algorithm, salt, IV) with the encrypted file.
**Encrypted File Viewer**: View the contents of an encrypted text file directly in the application without altering the file.

# Getting Started
Prerequisites
Rust: Ensure that Rust is installed. You can download it from [rust-lang.org].
Iced: This project relies on the Iced GUI library, which will be automatically installed when you build the project.
# Installation
1. **Clone the repository**:

```bash
Copy code
git clone https://github.com/yourusername/file-encryptor.git
cd file-encryptor
```
2. **Build the project**:

```bash
Copy code
cargo build --release
```
3. **Run the application**:

```bash
Copy code
cargo run --release
```
# Usage
1. **Choose a File**: Use the GUI to select a file you want to encrypt or decrypt.
2. **Enter a Passphrase**: Input a secure passphrase. This passphrase will be used to derive the encryption key.
3. **Select an Algorithm**: Choose your preferred encryption algorithm from the available options.
4. **Encrypt/Decrypt**: Click the Encrypt or Decrypt button as needed.
**Viewing Encrypted Files**
To view an encrypted text file without modifying it:

1. Select the encrypted file.
2. Click the "View" button.
3. The file's contents will be decrypted and displayed within the GUI.
# Supported Encryption Algorithms
The application offers three encryption algorithms:

- AES-GCM: Known for speed and security.
- AES-GCM-SIV: Provides additional security for certain scenarios.
- ChaCha20-Poly1305: A secure and efficient alternative to AES, particularly on mobile and low-power devices.
Each algorithm has different performance and security properties, making it easy to tailor the encryption to your specific requirements.

# Contributers:
- [Ramida Laphasphokin](https://github.com/RaphOwO) 67011287

# Acknowledgements
Iced GUI Library
RustCrypto for the cryptographic algorithms and utilities
