// net 模块示例：TCP/UDP通信的网络原语
use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr};
use std::io::{Read, Write};
use std::thread;

// 1. TCP服务器
fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("TCP server listening on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            let message = String::from_utf8_lossy(&buffer[..n]);
                            println!("Received: {}", message);
                            stream.write_all(b"Hello from server!").unwrap();
                        }
                        Err(e) => println!("Error reading: {}", e),
                    }
                });
            }
            Err(e) => println!("Error accepting connection: {}", e),
        }
    }
}

// 2. TCP客户端
fn tcp_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    
    stream.write_all(b"Hello from client!").expect("Failed to write");
    
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).expect("Failed to read");
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);
}

// 3. UDP示例
fn udp_example() {
    // 创建UDP套接字
    let socket = UdpSocket::bind("127.0.0.1:9000").expect("Failed to bind");
    
    // 发送消息
    let message = b"Hello UDP!";
    let addr: SocketAddr = "127.0.0.1:9001".parse().unwrap();
    socket.send_to(message, addr).expect("Failed to send");
    println!("Sent UDP message: {}", String::from_utf8_lossy(message));
    
    // 接收消息
    let mut buffer = [0; 1024];
    match socket.recv_from(&mut buffer) {
        Ok((n, addr)) => {
            let received = String::from_utf8_lossy(&buffer[..n]);
            println!("Received from {}: {}", addr, received);
        }
        Err(e) => println!("Error receiving: {}", e),
    }
}

fn main() {
    println!("Network examples");
    
    // 注意：实际运行时，需要分别运行服务器和客户端
    // 这里只是展示API的使用
    
    // 启动TCP服务器（在单独的线程中）
    // thread::spawn(|| {
    //     tcp_server();
    // });
    
    // 等待服务器启动
    // thread::sleep(std::time::Duration::from_secs(1));
    
    // 运行TCP客户端
    // tcp_client();
    
    // 运行UDP示例
    // udp_example();
    
    println!("Network examples - uncomment code to run");
}