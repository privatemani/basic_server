use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use std::net::Shutdown;
use std::io::{BufRead, BufReader};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Waiting for connect");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {

                println!("Connected");
                writeln!(stream, "Hello, world!").expect("failed to send message");

                let mut stream = BufReader::new(stream);
                let mut buffer = String::new();
                stream.read_line(&mut buffer).expect("failed to read line");
                println!("{}", buffer);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
