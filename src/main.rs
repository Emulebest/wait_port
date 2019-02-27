use std::net::TcpStream;
use std::io::ErrorKind;
use std::time::Duration;
use std::net::SocketAddr;
use std::time;
use std::io;
use std::io::Write;
use clap::{Arg, App};

fn collect_args() -> (String, Option<String>, Option<String>) {
    let matches = App::new("Wait for port")
        .about("This program waits for port to open")
        .version("0.1")
        .author("Igor Korotach")
        .arg(Arg::with_name("address")
            .short("addr")
            .long("address")
            .help("Sets address to wait to respond to")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .help("Sets timeout to wait")
            .takes_value(true))
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .help("Sets at which interval to poll address")
            .takes_value(true))
        .get_matches();
    let address = matches.value_of("address").expect("Couldn't parse address").to_owned();
    let timeout = matches.value_of("timeout");
    let timeout = match timeout {
        Some(val) => Some(val.to_owned()),
        None => None
    };
    let interval = matches.value_of("interval");
    let interval = match interval {
        Some(val) => Some(val.to_owned()),
        None => None
    };
    (address, timeout, interval)
}

fn parse_args(args: (String, Option<String>, Option<String>)) -> (String, Duration, Duration) {
    let (address, timeout, interval) = args;
    let mut timeout_duration = Duration::from_secs(30);
    let mut interval_duration = Duration::from_secs(1);
    match timeout {
        Some(t) => {
            timeout_duration = Duration::from_millis(t.parse::<u64>().expect("Couldn't interpret timeout as int"))
        }
        None => {}
    }
    match interval {
        Some(i) => {
            interval_duration = Duration::from_millis(i.parse::<u64>().expect("Couldn't interpret interval as int"))
        }
        None => {}
    }
    (address, timeout_duration, interval_duration)
}

fn connect() -> i32 {
    let started = time::Instant::now();
    let raw_args = collect_args();
    let (address, timeout, interval) = parse_args(raw_args);
    let stdout = io::stdout();
    let mut locked = stdout.lock();
    print!("Connecting");
    let address: SocketAddr = address.parse().expect("Couldn't parse string as socket address");
    while let Err(e) = TcpStream::connect_timeout(&address, Duration::from_secs(10)) {
        match e.kind() {
            ErrorKind::ConnectionRefused | ErrorKind::TimedOut | ErrorKind::ConnectionReset => {
                if (time::Instant::now() - started) > timeout {
                    println!("Timed out");
                    return 1
                }
                print!(".");
                locked.flush().expect("Unable to flush stdout");
                std::thread::sleep(interval);
                continue
            }
            _ => {
                eprintln!("Unexpected error while connecting");
                return 1
            }
        }
    }
    println!("\nConnected");
    0
}

fn main() {
    std::process::exit(connect())
}
