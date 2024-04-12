extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, RpcApi};

// ssh -v -NL 8332:127.0.0.1:8332 <remote-host>

fn main() {
    let mut args = std::env::args();
    let command = args.next().expect("Must provide password");
    let url = args.next().expect("Usage: {} <url> <username> <password>", command)
    let user = args.next().expect("Must provide username")
    let password = args.next().expect("Must provide password")
    let auth = Auth::UserPass(user, password;
    let client = Client::new(
        url,
        auth
    ).expect("Couldn't create client");
    let num_blocks = client.get_block_count().expect("Couldn't fetch block count");
    for index in 0..num_blocks {
        let hash = client.get_block_hash(index).expect("Couldn't fetch block hash");
        println!("{},{}", index, hash);
    }
}
