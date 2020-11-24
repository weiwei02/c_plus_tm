

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
/**
 * 
 * 简单实现 http 监听
 */
pub fn listener(){
    let listen = TcpListener::bind("0.0.0.0:12701").unwrap();
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


/// 处理接收到的连接
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let html = std::fs::read_to_string("hello.html").unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";


    stream.write(response.as_bytes()).unwrap();
    stream.write(html.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[test]
fn run_sigle_server(){
    listener();
}