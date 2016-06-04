use std::net::TcpListener;
use std::thread;
use rand;
use rand::Rng;
use protobuf::{Message,MessageStatic};
use std::io::BufWriter;

use raft;

#[derive(Debug)]
pub struct Server {
	bind: String,
	peers: Vec<String>,
	state: raft::State,
}

impl Server {
	pub fn serve(&self) {
        let bind: &str = &self.bind;
        let listener = TcpListener::bind(bind).unwrap();
        println!("listening at {}, ready to accept", self.bind);
        for stream in listener.incoming() {
            thread::spawn(|| {
                let mut stream = BufWriter::new(stream.unwrap());
                let vote = rand::thread_rng().gen_range(0, 32767);
                let mut rqv = raft::types::RequestVote::new();
                rqv.set_vote(vote);
                let mut msg = raft::types::RaftRPC::new();
                msg.set_request_vote(rqv);
                msg.write_to_writer(&mut stream).unwrap();
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
