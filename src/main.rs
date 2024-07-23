use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Starting server on 7878 port");

    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let status_ok = format!("HTTP/1.1 200 OK");
    let status_not_found = format!("HTTP/1.1 404 NOT FOUND");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (status_ok, "public/hello.html"),
        "GET /sleep HTTP/1.1" => (status_ok, "public/hello.html"),
        _ => (status_not_found, "public/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("{status_line} {length}");

    stream.write_all(response.as_bytes()).unwrap();
}
