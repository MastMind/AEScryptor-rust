use std::process::ExitCode;
use std::io::Read;
use clap::*;
use hex;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

enum Operation {
    Encrypt,
    Decrypt
}

type Aes128Cbc = Cbc<Aes128, Pkcs7>;


fn encrypt(_key: &Vec<u8>, _iv: &[u8], _buf: &mut Vec<u8>) {
    let mut key_aes: [u8;16] = [0; 16];

    for (place, data) in key_aes.iter_mut().zip(_key.iter()) {
        *place = *data
    }

    let cipher = Aes128Cbc::new_from_slices(&key_aes, _iv).unwrap();
    let ciphertext = cipher.encrypt_vec(_buf);

    _buf.clear();
    _buf.resize(ciphertext.len(), 0);

    for (place, data) in _buf.iter_mut().zip(ciphertext.iter()) {
        *place = *data
    }
}

fn decrypt(_key: &Vec<u8>, _iv: &[u8], _buf: &mut Vec<u8>) {
    let mut key_aes: [u8;16] = [0; 16];
    let mut buffer = _buf.clone();

    for (place, data) in key_aes.iter_mut().zip(_key.iter()) {
        *place = *data
    }

    let cipher = Aes128Cbc::new_from_slices(&key_aes, _iv).unwrap();
    let decryptedtext = cipher.decrypt_vec(&mut buffer).unwrap();

    _buf.clear();
    _buf.resize(decryptedtext.len(), 0);

    for (place, data) in _buf.iter_mut().zip(decryptedtext.iter()) {
        *place = *data
    }
}

fn main() -> ExitCode {
    let matches = App::new("AESCryptor")
        .version("0.1.0")
        .author("MastMind <github.com/MastMind>")
        .about("Encryption and decryption with AES-CBC algorithm")
        .arg(Arg::with_name("key")
                .short("k")
                .long("key")
                .takes_value(true)
                .help("Key for encryption"))
        .arg(Arg::with_name("hexkey")
                .long("hex-key")
                .takes_value(true)
                .help("Key for encrypting (in hex view)"))
        .arg(Arg::with_name("operation-encrypt")
                .short("e")
                .long("encrypt")
                .takes_value(false)
                .help("Set encryption mode (default)"))
        .arg(Arg::with_name("operation-decrypt")
                .short("d")
                .long("decrypt")
                .takes_value(false)
                .help("Set decryption mode"))
        .arg(Arg::with_name("hex-output")
                .long("hex-output")
                .takes_value(false)
                .help("Set output in hex"))
        .arg(Arg::with_name("hex-input")
                .long("hex-input")
                .takes_value(false)
                .help("Set input in hex"))
        .get_matches();

    let key_opt = matches.value_of("key");
    let hex_key_opt = matches.value_of("hexkey");
    let encrypt_opt = matches.is_present("operation-encrypt");
    let decrypt_opt = matches.is_present("operation-decrypt");
    let mut key_advertised = false;
    let mut oper = Operation::Encrypt;
    let mut key:Vec<u8> = vec![];

    //set mode
    if encrypt_opt && decrypt_opt {
        println!("Please choose encrypt or decrypt mode (not both)");
        return ExitCode::from(254);
    }

    if encrypt_opt {
        oper = Operation::Encrypt;
    }

    if decrypt_opt {
        oper = Operation::Decrypt;
    }

    match key_opt {
        None => {
        },
        Some(key_opt) => {
            key_advertised = true;
            key = key_opt.into();
        }
    }

    match hex_key_opt {
        None => {
        },
        Some(hex_key_opt) => {
            key_advertised = true;
            key = hex::decode(hex_key_opt).expect("Decoding failed");
        }
    }

    if !key_advertised {
        return ExitCode::from(255);
    }

    let iv = hex::decode("00000000000000000000000000000000").expect("Decoding failed");

    //reading from stdin
    let mut buf:Vec<u8> = vec![];

    for i in std::io::stdin().bytes() {
        buf.push(i.unwrap());
    }

    if matches.is_present("hex-input") {
        buf = hex::decode(buf).expect("Input hex is wrong");
    }

    match oper {
        Operation::Encrypt => { 
            encrypt(&key, &iv, &mut buf);

            if matches.is_present("hex-output") {
                let hex_string : String = buf.iter()
                    .map(|b| format!("{:02X}", b).to_string())
                    .collect::<Vec<String>>()
                    .join("");
                print!("{}", hex_string);
            } else {
                unsafe {
                    print!("{}", std::str::from_utf8_unchecked(&buf));
                }
            }
        },
        Operation::Decrypt => {
            decrypt(&key, &iv, &mut buf);

            if matches.is_present("hex-output") {
                let hex_string : String = buf.iter()
                    .map(|b| format!("{:02X}", b).to_string())
                    .collect::<Vec<String>>()
                    .join("");
                print!("{}", hex_string);
            } else {
                unsafe {
                    print!("{}", std::str::from_utf8_unchecked(&buf));
                }
            }
        }
    }

    return ExitCode::SUCCESS;
}
