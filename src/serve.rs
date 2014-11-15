use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {

	let listener = TcpListener::bind("127.0.0.1:8080");

	// bind the listener to the specified address
	let mut acceptor = listener.listen();

	fn handle_client(mut stream: TcpStream, count: uint) {
		let st = format!("connection #{} get!", count);
		stream.write(st.into_bytes().as_slice());
		//println!("{}", st);
	}

	// accept connections and process them, spawning a new tasks for each one
	let mut count = 0u;
	for stream in acceptor.incoming() {
		count += 1u; // keep count
		match stream {
		Err(e) => { println!("connection not get!"); }
		Ok(stream) => spawn(proc() {
				// connection succeeded
				handle_client(stream, count);
			})
		}
	}

	// close the socket server
	drop(acceptor);
}
