use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::net::UdpSocket;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

mod settings;

fn main() {
    // TODO: catch TERM signal and use this to gracefully shutdown
    let mut running = true;
    // When this proxy receives StatsD messages, we push them on this vec/queue for processing in other threads
    let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
    // Create an atomically-reference-counted mutex around our vec/queue
    let mutex = Arc::new(Mutex::new(queue));
    // TODO: implement graceful shutdown and wait for these threads
    // We store thread handles here.
    let mut handles = vec![];

    println!("Loading settings from config.ini");
    let settings = settings::Settings::load("config.ini");
    let a = format!("{}:{}", settings.bind_interface, settings.bind_port);
    println!(
        "Listening on {}\nThreads: {}\nSharding to: {:?}",
        a, settings.threads, settings.destinations
    );

    // PROCESSING THREADS
    let num_destinations = settings.destinations.len() as u64;
    for _ in 0..settings.threads {
        let cloned_mutex = Arc::clone(&mutex);
        let destinations = settings.destinations.clone();
        let handle = thread::spawn(move || {
            // A tally of the metrics we've proxied in this thread.
            // I send this count to my downstream StatsD server to measure proxy performance
            let mut num_metrics = 0;
            // We'll wait if there's nothing in the vec/queue to process
            let ten_millis = time::Duration::from_millis(10);
            // This RegEx pattern helps us split the StatsD metric into: MEASUREMENT TAG1 TAG2 ... TYPE+AND+VALUE
            let re = Regex::new(r"[,:]").expect("Failed to compile regex");
            // We use this socket to send proxied+sharded metrics to a downstream StatsD server
            let sender = UdpSocket::bind("0.0.0.0:0").expect("Could not bind sender UDP socket");

            loop {
                // Acquire a mutex lock and unwrap the associated vec/queue
                let mut q = cloned_mutex.lock().unwrap();
                if q.len() == 0 {
                    // If no messages to process, release the mutex lock ASAP then wait
                    drop(q);
                    thread::sleep(ten_millis);
                    continue;
                }

                let message = q.pop_front().unwrap();
                let message = str::from_utf8(&message).unwrap();
                // Releasing the mutex ASAP gets us at least another 1 million messages processed
                // per 10 seconds
                drop(q);

                for line in message.lines() {
                    // Splitting with Regular Expressions is so much faster than string split()
                    // Split StatsD metric into: MEASUREMENT TAG1 TAG2 ... TYPE|VALUE
                    let mut parts: Vec<&str> = re.split(line).collect();

                    // If we don't have at least a measurement and TYPE|VALUE, go to next line
                    if parts.len() < 2 {
                        continue;
                    }

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

                    // Hash into a shard number
                    // TODO: implement djb hash function to learn bitwise ops
                    let mut s = DefaultHasher::new();
                    shardable_metric.hash(&mut s);
                    /*
                    help: you can convert a `u64` to a `usize` and panic if the converted value doesn't fit
                    let shard_number: usize = (s.finish() % num_shards).try_into().unwrap();
                    */
                    let shard_number: usize = (s.finish() % num_destinations).try_into().unwrap();

                    // Send the original line to the appropriate downstream server
                    // to avoid the extra string op of pushing the type+value onto the shardable_metric string
                    sender
                        .send_to(line.as_bytes(), &destinations[shard_number])
                        .expect("Failed to send");
                    // TODO: batch metrics up to MTU to reduce number of UDP packets we send

                    // Increment tally of how many messages this thread has processed and sent
                    num_metrics += 1;
                }

                // This sends meta metrics (for the proxy itself) to my StatsD server
                if num_metrics > 1000 {
                    sender
                        .send_to(
                            format!(
                                "sharding_proxy.metrics_sent,host=gigabyte:{}|c",
                                num_metrics
                            )
                            .as_bytes(),
                            "192.168.1.173:8125",
                        )
                        .expect("Failed to send");
                    num_metrics = 0;
                }
            }
        });
        handles.push(handle);
    }

    // RECEIVING SOCKET
    let socket: UdpSocket = UdpSocket::bind(a).expect("Could not bind");
    // TODO: make this configurable in INI .... max_udp_packet_size or something
    let mut buf = [0; 1024];
    while running {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Did not recieve data");
        {
            let mut q = mutex.lock().unwrap();
            // TODO: is there a way to enqueue the buf directly?
            q.push_back( buf[..amt].to_owned() );
        }
    }
}
