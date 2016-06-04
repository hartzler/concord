pub mod types;

#[derive(Debug)]
pub enum State {
    Candidate,
    Follower,
    Leader,
}

pub fn hello() {
    println!("Hello from raft!");
}
