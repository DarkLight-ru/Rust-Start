pub mod client;

use std::net::TcpListener;

fn main() {
loop {
    func_listener();
};

}

fn func_listener() {
    let  _listen = TcpListener::bind("0.0.0.0:25565");
}