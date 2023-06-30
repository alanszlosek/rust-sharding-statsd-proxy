#![feature(test)]

extern crate test;

use regex::Regex;
use std::fs::File;
use std::{io::Error, io::Read};

mod hashing;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn hash1(b: &mut Bencher) {
        
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => {
                    match file.read_to_string(&mut contents) {
                        Ok(_) => println!("Yay"),
                        Err(_) => println!("Failed to read messages.txt")
                    }
                },
                Err(e) => println!("Failed to open messages.txt {}", e)
            }

            let re = Regex::new(r"[,:]").expect("Failed to compile regex");

            for line in contents.lines() {
                let mut parts: Vec<&str> = re.split(line).collect();
                let i = hashing::hash1(parts, 3);
                assert!(i >= 0 && i < 3);
            }
        });
    }

    #[bench]
    fn hash2(b: &mut Bencher) {
        
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => {
                    match file.read_to_string(&mut contents) {
                        Ok(_) => println!("Yay"),
                        Err(_) => println!("Failed to read messages.txt")
                    }
                },
                Err(e) => println!("Failed to open messages.txt {}", e)
            }

            let re = Regex::new(r"[,:]").expect("Failed to compile regex");

            for line in contents.lines() {
                let mut parts: Vec<&str> = re.split(line).collect();
                let i = hashing::hash2(parts, 3);
                assert!(i >= 0 && i < 3);
            }
        });
    }

    #[test]
    fn test2() {
        let re = Regex::new(r"[,:]").expect("Failed to compile regex");

        let contents = "hey.there:1|c";
        for line in contents.lines() {
            let mut parts: Vec<&str> = re.split(line).collect();
            let i = hashing::hash2(parts, 3);
            assert!(i > 0 && i < 3);
        }
    }
}
