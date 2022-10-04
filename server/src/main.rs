use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::Read;
use std::io::Write;
use std::fs;


fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut count = 0;

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        count += 1; 
        println!("Connection established! {}", count);
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        
        stream.read(&mut buffer).unwrap();

        
        stream.write(response.as_bytes()).unwrap();
    }else{
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        
        stream.read(&mut buffer).unwrap();

        stream.write(response.as_bytes()).unwrap();
    }

    
}
