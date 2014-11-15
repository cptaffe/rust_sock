use std::io::TcpStream;
use std::str;

fn client() {
	let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
	let response = socket.read_to_end();
	match response {
		Err(e) => { println!("error: {}", e) },
		Ok(buf) => {
			match str::from_utf8(buf.as_slice()) {
				None => { println!("response error!") },
				Some(s) => { println!("response get! {}", s) }
			}
		}
	}
}

fn main() {
	let max = 2000000u;
	for in range(0, max) {
		spawn(proc() {
			client();
		});
	}
}
