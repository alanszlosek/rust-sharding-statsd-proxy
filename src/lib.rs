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

            let re = Regex::new(r"[,:]").expect("Failed to compile regex");
            for line in contents.lines() {
                let parts: Vec<&str> = re.split(line).collect();
                let hash_value = hashing::hash1(parts);
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
                let parts: Vec<&str> = re.split(line).collect();
                let i = hashing::hash2(parts);
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

            let re = Regex::new(r"[,:]").expect("Failed to compile regex");
            for line in contents.lines() {
                let parts: Vec<&str> = re.split(line).collect();
                let i = hashing::hash3(parts);
            }
        });
    }

    // compare against value of djb2 in C (see helpers/djb2) on same shardable portion of metric
    #[test]
    fn test_hashing() {
        let re = Regex::new(r"[,:]").expect("Failed to compile regex");
        let contents = "dgpnfnxw.qxufgnlwp,sesdaofncycmbum=eodzjc,hggfvghceyfz=lnelpjdhpqj,gugdtstao=oxbodp:2|c";
        let parts: Vec<&str> = re.split(contents).collect();
        let hash_value = hashing::hash3(parts);
        println!("Hash value: {}", hash_value);
        assert_eq!(hash_value, 7513145795220795972);

    }
}
