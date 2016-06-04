extern crate protobuf;
pub mod raft;
pub mod server;

pub fn hello() {
	let mut foo = raft::types::RequestVote::new();
	foo.set_vote(128);
	let foo = foo;
	println!("foo: {:?}", foo);
	println!("vote: {}", foo.get_vote());
	raft::hello();
}
