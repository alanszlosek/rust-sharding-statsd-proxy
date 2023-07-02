#![feature(test)]

extern crate test;

mod hashing;

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::io::Read;
    use std::fs::File;
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

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let hash_value = h.hash1(line);
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

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let i = h.hash2(line);
            }
        });
    }

    #[bench]
    fn hash3(b: &mut Bencher) {
        
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

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let i = h.hash3(line);
            }
        });
    }

    // compare against value of djb2 in C (see helpers/djb2) on same shardable portion of metric
    #[test]
    fn test_hashing() {
        let re = Regex::new(r"[,:]").expect("Failed to compile regex");
        let contents = "dgpnfnxw.qxufgnlwp,sesdaofncycmbum=eodzjc,hggfvghceyfz=lnelpjdhpqj,gugdtstao=oxbodp:2|c";
        let h = hashing::Hashing::new();
        let hash_value = h.hash3(contents);
        println!("Hash value: {}", hash_value);
        assert_eq!(hash_value, 7513145795220795972);
    }
}
