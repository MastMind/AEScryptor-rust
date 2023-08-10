# AEScryptor-rust
A linux command line utility for encrypting/decrypting messages and files with AES128 CBC algorithm. Rust implementation.
This is the same utility (https://github.com/MastMind/AEScryptor) but in Rust language.

## Build:

        cargo build --release

## Examples:

For encrypting a file (some_message.txt in that case) you have to use the next command (encrypted file has name some_message.txt.crypted):

        cat some_message.txt | target/release/AEScryptor --key SecretKey1 -e > some_message.txt.crypted

For decrypting a crypted file you have to use the next command:

        cat some_message.txt.crypted | target/release/AEScryptor --key SecretKey1 -d > some_message.txt

Also you can use the utility for encrypting a raw string (result will be in stdout):

        echo -n "Very secret message" | target/release/AEScryptor --key SecretKey1 -e --hex-output

For decrypting a raw string you have to use the next command:

        echo -n "4DC0CAFB7220D7C8F7612C52978DB07D6FEDFD61FDEC19F40750606215463E12" | target/release/AEScryptor --key SecretKey1 -d --hex-input

Also you can use keys in hex form:

        echo -n "Very secret message" | target/release/AEScryptor --hex-key 0B055000 -e --hex-output

        echo -n "0368D7A4FB000A94132792909AD5992FAD3055CD99BB2AD837A411B413A6CFFF" | target/release/AEScryptor --hex-key 0B055000 -d --hex-input

For more information about options:

        target/release/AEScryptor --help
