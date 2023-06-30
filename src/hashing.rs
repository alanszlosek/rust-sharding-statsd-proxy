use std::convert::TryInto;

pub fn hash1(mut parts: Vec<&str>, num_destinations: u32) -> usize {
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
    /*
    unsigned long
    hash(unsigned char *str)
    {
        unsigned long hash = 5381;
        int c;
        while (c = *str++)
            hash = ((hash << 5) + hash) + c; /* hash * 33 + c */

        return hash;
    }
    */
    let mut hash: u32 = 5381;
    for char in shardable_metric.chars() {
        hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u32);
    }
    let shard_number: usize = (hash % num_destinations).try_into().unwrap();
    shard_number
}



pub fn hash2(mut parts: Vec<&str>, num_destinations: u32) -> usize {
    let l = parts.len() - 1;
    if parts.len() > 3 {
        parts[1..l].sort();        
    }

    // djb2 hash
    /*
    unsigned long
    hash(unsigned char *str)
    {
        unsigned long hash = 5381;
        int c;
        while (c = *str++)
            hash = ((hash << 5) + hash) + c; /* hash * 33 + c */

        return hash;
    }
    */

    let mut hash: u32 = 5381;
    // hash measurement
    for char in parts[0].chars() {
        hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u32);
    }
    // hash tags and their preceding commas
    for part in parts[1..l].iter() {
        hash = (hash << 5).wrapping_add(hash).wrapping_add(',' as u32);
        for char in part.chars() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(char as u32);
        }
    }
    let shard_number: usize = (hash % num_destinations).try_into().unwrap();
    shard_number
}
