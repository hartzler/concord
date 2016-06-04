extern crate concord;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
Hello Concord.

Usage:
  hello server --port=<port> [--peer=<peer>...]
  hello (-h | --help)
  hello --version

Options:
  --port=<port>      The port to do RPC on
  [--peer=<peer>...] Connect to peer
  -h --help          Show this screen.
  --version          Show version.

Version:
  0.1.0
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: u16,
    flag_peer: Vec<String>,
    cmd_server: bool,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    concord::hello();
}
