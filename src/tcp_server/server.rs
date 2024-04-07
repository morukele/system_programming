use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn run_sever(ip_address: &str, port: i8) {
    // Connects to a TCP server running on localhost:8080
    let address = format!("{}:{}", ip_address, port);
    let _stream = TcpStream::connect(&address).expect("unable to connect to address");
    let listener = TcpListener::bind(&address).expect("unable to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return;
        }

        stream
            .write_all(&buf[0..bytes_read])
            .expect("Failed to write to client");
    }
}
