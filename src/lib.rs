use anyhow::Result;
use std::io::{Write, Read};
use std::fs::File;
use num_bigint::BigUint;

use crate::error::Errors;

pub mod cli;
pub mod error;

fn gcd(x: u128, y: u128) -> u128 {
    match x {
        0 => y,
        _ => gcd(y % x, x)
    }
}

pub fn is_prime(n: u128) -> bool {
    if n == 0 || n == 1 {
        return false
    }
    for i in 2..(n/2)+1 {
        if n % i == 0 {
            return false
        }
    }
    true
}

pub fn gen_keys(p: u128, q: u128) -> Vec<(u128, u128)> {
    assert!(is_prime(p) && is_prime(q));
    let n = p * q;
    let es_n = (p - 1) * (q - 1);
    let mut e = 3;
    let mut d = 1;
    for i in e.. {
        if gcd(i, es_n) == 1 {
            e = i;  
            break
        }
    }
    for i in d.. {
        if (e * i) % es_n == 1 {
            d = i; 
            break
        }
    }

    let mut stop = false;
    while e == d {
        for i in e+2.. {
            if gcd(i, es_n) == 1 {
                e = i;
                stop = true;
                for i in d+2.. {
                    if (e * i) % es_n == 1 {
                        d = i;
                        break
                    }
                }
                break
            }
        }
        if stop {
            break
        }
    }
    let keys = [(e, n), (d, n)].to_vec();
    keys
}

pub fn write_keys(keys: Vec<(u128, u128)>) -> Result<()> {
    let mut pub_key_file = File::create("rsa.pub")?; 
    let mut priv_key_file = File::create("rsa")?;
    for (i, key) in keys.iter().enumerate() {
        if i == 0 {
            let pub_key = format!("{}, {}", key.0, key.1);
            pub_key_file.write_all(pub_key.as_bytes())?;
        } else if i == 1 {
            let priv_key = format!("{}, {}", key.0, key.1);
            priv_key_file.write_all(priv_key.as_bytes())?;
        }
    }
    Ok(())
}

fn read_key(filename: String) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn encrypt(msg: u128) -> Result<()> {
    if let Ok(pub_key) = read_key("rsa.pub".to_string()) {
        let pub_key_parts = pub_key
            .split(",")
            .into_iter()
            .map(|x| x
                .trim()
                .parse::<u128>()
                .expect("Could not parse number in key")
            )
            .collect::<Vec<u128>>();

        assert!(pub_key_parts.len() == 2);

        match pub_key_parts.as_slice() {
            [e, n] => {
                let msg = BigUint::from(msg);
                let e = BigUint::from(*e);
                let n = BigUint::from(*n);
                let ep = msg.modpow(&e, &n);
                println!("Encrypted message: {}", ep);
            },
            _ => return Err(Errors::InvalidKeyFormat("Invalid key format in file rsa.pub".to_string()).into()),
        };
    }
    Ok(())
}

pub fn decrypt(msg: u128) -> Result<()> {
    if let Ok(priv_key) = read_key("rsa".to_string()) {
        let priv_key_parts = priv_key 
            .split(",")
            .into_iter()
            .map(|x| x
                .trim()
                .parse::<u128>()
                .expect("Could not parse number in key")
            )
            .collect::<Vec<u128>>();

        assert!(priv_key_parts.len() == 2);

        match priv_key_parts.as_slice() {
            [d, n] => {
                let msg = BigUint::from(msg);
                let d = BigUint::from(*d);
                let n = BigUint::from(*n);
                let dp = msg.modpow(&d, &n);
                println!("Decrypted message: {}", dp)
            },
            _ => return Err(Errors::InvalidKeyFormat("Invalid key format in file rsa".to_string()).into()),
        };
    }
    Ok(())
}
