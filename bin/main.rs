use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
extern crate server_pool;
use server_pool::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.excute(||{
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    let contents = fs::read_to_string("hello.html").unwrap();
    stream.read(&mut buffer).unwrap();

    let status_line = "HTTP 1.1 200 OK\r\n\r\n";
    let response = format!("{}{}",status_line,contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

