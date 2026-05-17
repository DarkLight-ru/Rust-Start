use std::{io::Read, net::TcpStream};

fn main() {
    let tcpConn = TcpStream::connect("0.0.0.0:25565");
    tcpConn;
    test();
    }

fn test(stream: &mut TcpStream) {
    let mut buff = [0u8; 1028];
    match stream.read(&mut buff)  {
        Ok(n) => println!("readed bytes: {}", n),
        Err(e) => println!("error read: {}", e),
    }

}