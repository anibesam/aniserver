use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1946").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_request(stream);
    }
}


//Handle Request function
fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/.1" {
        ("HTTP/2 200 OK", "ukomu/pages/index.js")
    } else {
        ("HTTP/2 404 NOT FOUND", "ukomu/pages/404.js")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
