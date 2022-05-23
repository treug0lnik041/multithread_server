use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

use multithread_server::ThreadPool;

fn main() {
	let listener = TcpListener::bind("0.0.0.0:80").unwrap();
	let pool = ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	let (status_line, contents) = if buffer.starts_with("GET".as_bytes()) {
		let request = String::from_utf8_lossy(&buffer[..]);

		let (start, end) = (request.find("/").unwrap()+1, request.find("HTTP").unwrap());
		let filename = &request[start..end];

		if let Ok(contents) = fs::read_to_string(filename) {
			("HTTP/1.1 200 OK", contents)
		} else {
			("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap())
		}

	} else {
		("HTTP/1.1 405 NOT ALLOWED", String::from("Allow: GET"))
	};

	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		status_line,
		contents.len(),
		contents
	);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
