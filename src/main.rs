use std::net::TcpListener;
use std::thread;
use std::io::prelude::*;

fn main() {
    // 调用标准库 TcpListener监听 127.0.0.1:8888
    let listener = TcpListener::bind("127.0.0.1:8888");
    // 对监听127.0.0.1:8888做标准的错误处理（模式匹配）
    let listener = match listener {
        Ok(listener) => listener,
        Err(error) => {
            panic!("Problem listening 127.0.0.1:8888: {:?}", error);
        },
    };
    println!("listening started, ready to accept");
    // 循环监听每个请求的steam
    for stream in listener.incoming() {
        thread::spawn(|| {
            // 对监听stream做标准的错误处理（模式匹配）
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(error) => {
                    panic!("Problem listening incoming stream: {:?}", error);
                },
            };
            // 初始化变量buffer，用来装读取client发送的信息
            let mut buffer = [0; 999];
            // 对读取buffer做标准的错误处理（模式匹配）
            match stream.read(&mut buffer) {
                Ok(s) => s,
                Err(error) => {
                    panic!("Problem reading incoming stream: {:?}", error);
                },
            };
            // 对 tcp client发过来的消息，打印，并做 echo 返回
            println!("echo: {}", String::from_utf8_lossy(&buffer[..]));
        });
    }
}
