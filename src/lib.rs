#![feature(test)]

use std::collections::VecDeque;
use std::convert::TryInto;
use std::net::UdpSocket;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

// the library crate uses modules, and we need to "export" some as public
// so they can be used by the binary crate
pub mod cli;
mod hashing;
pub mod settings;

extern crate test;


pub fn run(settings: settings::Settings) {
    // TODO: catch TERM signal and use this to gracefully shutdown
    let running = true;
    // When this proxy receives StatsD messages, we push them on this vec/queue for processing in other threads
    let queue: VecDeque<Vec<u8>> = VecDeque::new();
    // Create an atomically-reference-counted mutex around our vec/queue
    let mutex = Arc::new(Mutex::new(queue));
    // TODO: implement graceful shutdown and wait for these threads
    // We store thread handles here.
    let mut handles = vec![];

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

            // We use this socket to send proxied+sharded metrics to a downstream StatsD server
            let sender = UdpSocket::bind("0.0.0.0:0").expect("Could not bind sender UDP socket");

            let h = hashing::Hashing::new();

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
                    let hash_value = h.hash5(line);
                    let shard_number: usize = (hash_value % num_destinations).try_into().unwrap();

                    // Send the original line to the appropriate downstream server
                    // to avoid unnecessary string ops on parts vector
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
            q.push_back(buf[..amt].to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use test::Bencher;

    #[bench]
    fn hash1(b: &mut Bencher) {
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Yay"),
                    Err(_) => println!("Failed to read messages.txt"),
                },
                Err(e) => println!("Failed to open messages.txt {}", e),
            }

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let _ = h.hash1(line);
            }
        });
    }

    #[bench]
    fn hash2(b: &mut Bencher) {
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Yay"),
                    Err(_) => println!("Failed to read messages.txt"),
                },
                Err(e) => println!("Failed to open messages.txt {}", e),
            }

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let _ = h.hash2(line);
            }
        });
    }

    #[bench]
    fn hash3(b: &mut Bencher) {
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Yay"),
                    Err(_) => println!("Failed to read messages.txt"),
                },
                Err(e) => println!("Failed to open messages.txt {}", e),
            }

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let _ = h.hash3(line);
            }
        });
    }

    #[bench]
    fn hash4(b: &mut Bencher) {
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Yay"),
                    Err(_) => println!("Failed to read messages.txt"),
                },
                Err(e) => println!("Failed to open messages.txt {}", e),
            }

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let _ = h.hash4(line);
            }
        });
    }

    #[bench]
    fn hash5(b: &mut Bencher) {
        b.iter(|| {
            let mut contents = String::new();
            match File::open("messages.txt") {
                Ok(mut file) => match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Yay"),
                    Err(_) => println!("Failed to read messages.txt"),
                },
                Err(e) => println!("Failed to open messages.txt {}", e),
            }

            let h = hashing::Hashing::new();
            for line in contents.lines() {
                let _ = h.hash5(line);
            }
        });
    }

    // compare against value of djb2 in C (see helpers/djb2) on same shardable portion of metric
    #[test]
    fn test_hashing() {
        let contents = "dgpnfnxw.qxufgnlwp,sesdaofncycmbum=eodzjc,hggfvghceyfz=lnelpjdhpqj,gugdtstao=oxbodp:2|c";
        let h = hashing::Hashing::new();
        let hash_value = h.hash5(contents);
        println!("Hash value: {}", hash_value);
        assert_eq!(hash_value, 7513145795220795972);
    }
}
