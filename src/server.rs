use std::io::Write;
use std::net::TcpListener;
use std::thread;
use raft;

#[derive(Debug)]
pub struct Server {
	bind: String,
	peers: Vec<String>,
	state: raft::State,
}

impl Server {
	pub fn serve(self) {
        let bind: &str = &self.bind;
        let listener = TcpListener::bind(bind).unwrap();
        println!("listening at {}, ready to accept", self.bind);
        for stream in listener.incoming() {
            thread::spawn(|| {
                let mut stream = stream.unwrap();
                stream.write(b"Hello World\r\n").unwrap();
            });
        }
	}
}

pub fn new(bind: String, peers: Vec<String>) -> Server {
	return Server{
		bind: bind,
		peers: peers,
		state: raft::State::Candidate,
	};
}
