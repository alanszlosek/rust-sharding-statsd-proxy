use regex::Regex;

pub struct Hashing {
    re: regex::Regex,
}

impl Hashing {
    pub fn new() -> Self {
        Hashing {
            // This RegEx pattern helps us split the StatsD metric into: MEASUREMENT TAG1 TAG2 ... TYPE+AND+VALUE
            re: Regex::new(r"[,:]").expect("Failed to compile regex"),
        }
    }

    pub fn hash1(&self, message: &str) -> u64 {
        let mut parts: Vec<&str> = self.re.split(message).collect();
        // Remove the measurement ...
        let measurement: &str = parts.remove(0);
        // Remove the type and value
        let _measurement_type: &str = parts.pop().unwrap();
        // Left are tags ... sort them so we can ensure consistent sharding
        parts.sort();
        // Push measurement back onto the front
        parts.insert(0, measurement);
        // Join measurement and tags into a string we can hash to shards
        let shardable_metric = parts.join(",");

        // djb2 hash
        let mut hash: u64 = 5381;
        for char in shardable_metric.chars() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
        }
        hash
    }

    pub fn hash2(&self, message: &str) -> u64 {
        let mut parts: Vec<&str> = self.re.split(message).collect();

        let l = parts.len() - 1;
        if parts.len() > 3 {
            parts[1..l].sort();
        }
        let shardable_metric = parts[0..l].join(",");

        // djb2 hash
        let mut hash: u64 = 5381;
        for char in shardable_metric.chars() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
        }
        hash
    }

    // With this logic we can process 100k more messages per 10 seconds on my machine
    pub fn hash3(&self, message: &str) -> u64 {
        let mut parts: Vec<&str> = self.re.split(message).collect();

        let l = parts.len() - 1;
        if parts.len() > 3 {
            parts[1..l].sort();
        }

        // djb2 hash across string slices within the vector
        // also hashing commas that were taken out during RegEx split
        let mut hash: u64 = 5381;
        // hash measurement
        for char in parts[0].chars() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
        }
        // hash tags and their preceding commas
        for part in parts[1..l].iter() {
            // hash comma delimiters that were taken out when we split StatsD message
            // into parts using RegEx
            hash = (hash << 5).wrapping_add(hash).wrapping_add(',' as u64);
            for char in part.chars() {
                hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
            }
        }
        hash
    }
}
