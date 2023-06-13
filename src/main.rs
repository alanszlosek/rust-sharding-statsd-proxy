use regex::Regex;
use std::net::UdpSocket;
use std::str;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::convert::TryInto;

fn main() {
    // TODO: make this configurable
    let destinations = ["192.168.3.74:5002", "192.168.1.120:5002"];
    let bind_address = "0.0.0.0:5001";

    let socket: UdpSocket = UdpSocket::bind(bind_address).expect("Could not bind");
    let mut running = true;
    let re = Regex::new(r"[,:]").expect("Failed to compile regex");
    let num_shards= destinations.len() as u64;

    println!("Listening on {}, sharding to: {:?}", bind_address, destinations);

    // buffer of 1024
    // TODO: choose something larger than max UDP packet size
    let mut buf = [0; 1024];
    let mut num_metrics = 0;
    while running {
        // THESE COMMENTS ARE OLD, BUT LEAVING HERE FOR WHEN I'M READY TO ADDRESS
        // TODO: how do we walk the buffer, noting end of metric+tags, looking for end of line
        // TODO: parse the tags, sort and create a unique tag set
        // TODO: shard the metric+tags to a destination statsd server
        // TODO: queue up each line into the list appropriate for the destination server
        // TODO: concurrently send to those when data has arrived, batching up as much as possible,
        // but not for longer than 10 seconds
        // TODO: skip utf8 checks for speed?
        // TODO: properly handle invalid UTF8
        // TODO: exclude bytes beyond amt .... slice of array somehow?

        let (amt, _src) = socket.recv_from(&mut buf).expect("Did not recieve data");
        let data = str::from_utf8(&buf[..amt]).unwrap();
        for line in data.lines() {
            // TODO: properly handle empty lines
            let mut parts: Vec<&str> = re.split(line).collect();
            // TODO: handle case where parts isn't a Vec with 2+

            // If we ensure parts has expected num elements, these are safe
            let measurement: &str = parts.remove(0);
            let _measurement_type: &str = parts.pop().unwrap();
            parts.sort();
            // Push measurement back onto the front
            parts.insert(0, measurement);
            let shardable_metric = parts.join(",");

            let mut s = DefaultHasher::new();
            shardable_metric.hash(&mut s);

            /*
            help: you can convert a `u64` to a `usize` and panic if the converted value doesn't fit
            let shard_number: usize = (s.finish() % num_shards).try_into().unwrap();
            */
            let shard_number: usize = (s.finish() % num_shards).try_into().unwrap();

            socket.send_to(line.as_bytes(), destinations[shard_number]).expect("Failed to send");

            num_metrics += 1;
        }

        // This sends meta metrics (for the proxy itself) to my StatsD server
        if num_metrics > 1000 {
            socket.send_to(format!("sharding_proxy.metrics_sent,host=gigabyte:{}|c", num_metrics).as_bytes(), "192.168.1.173:8125").expect("Failed to send");
            num_metrics = 0;
        }
    }

}
