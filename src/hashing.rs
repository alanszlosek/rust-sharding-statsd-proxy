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

    // NOTE: I'm keeping all hash*() functions around so I can use
    // `cargo bench` to show performance improvements

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

    pub fn hash4(&self, message: &str) -> u64 {
        // djb2 hash across string slices within the vector
        // also hashing commas that were taken out during RegEx split
        let mut hash: u64 = 5381;

        // Parse StatsD message, hashing the metric name as we go
        let mut at_beginning = true;
        let mut previous_is_comma = false;
        let mut tag_start_index: usize = 0;
        let mut tags: Vec<&str> = Vec::new();
        for (index, char) in message.char_indices() {
            if char == ',' {
                at_beginning = false;
                previous_is_comma = true;
                if tag_start_index > 0 {
                    // Found the end of a previous tag
                    // Push it!
                    tags.push( &message[tag_start_index..index] );
                    tag_start_index = 0;
                }
            } else if char == ':' {
                at_beginning = false;
                if tag_start_index > 0 {
                    // Found the end of a previous tag
                    // Push it!
                    tags.push( &message[tag_start_index..index] );
                }

                // All done!
                break;

            } else if at_beginning {
                // hash the metric name
                hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
            } else {
                if previous_is_comma {
                    // log tag?
                    tag_start_index = index;

                }
                previous_is_comma = false;
            }
        }

        if tags.len() > 0 {
            tags.sort();
        
            // hash tags and their preceding commas
            for part in tags {
                // hash comma delimiters that were taken out when we split StatsD message
                // into parts using RegEx
                hash = (hash << 5).wrapping_add(hash).wrapping_add(',' as u64);
                for char in part.chars() {
                    hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
                }
            }
        }
        hash
    }

    pub fn hash5(&self, message: &str) -> u64 {
        // djb2 hash across string slices within the vector
        // also hashing commas that were taken out during RegEx split
        let mut hash: u64 = 5381;

        let mut tag_start_index: usize = 0;
        let mut tags: Vec<&str> = Vec::new();
        let mut chars = message.char_indices();
        // Parse StatsD message, hashing the metric name as we go
        while let Some((index, char)) = chars.next() {
            match char {
                ':' => break,
                ',' => {
                    // We found a comma which starts the tag section
                    tag_start_index = index + 1;
                    break;
                },
                _ => {}
            }
            // hash the metric name
            hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
        }
        if tag_start_index > 0 {
            // TODO: finish here
            while let Some((index, char)) = chars.next() {
                if char == ':' {
                    // Push and done
                    tags.push( &message[tag_start_index..index] );
                    break;
                } else if char == ',' {
                    tags.push( &message[tag_start_index..index] );
                    tag_start_index = index + 1;
                }
            }

            tags.sort();
        
            // hash tags and their preceding commas
            for part in tags {
                // hash comma delimiters that we saw during char matching
                // into parts using RegEx
                hash = (hash << 5).wrapping_add(hash).wrapping_add(',' as u64);
                for char in part.chars() {
                    hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u64);
                }
            }
        }
        hash
    }

}
