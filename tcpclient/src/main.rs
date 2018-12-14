use std::io::stdin;
use std::io::prelude::*;
use std::thread;
use std::net::TcpStream;

fn main() {
    let res = TcpStream::connect("127.0.0.1:7878");
    if res.is_ok() {
        let mut stream = res.unwrap();
        stream.write("hell".as_bytes());
        //stream.read_to_end(&mut buffer);
        let input_stream = stream.try_clone();
        let handle = thread::spawn(|| {
            handle_stream(input_stream.unwrap());
        });

        let stdin = stdin();
        for line in stdin.lock().lines() {
            //println!("{}", line.unwrap());
            stream.write(line.unwrap().as_bytes());
        }
        println!("ok worlud");

        handle.join().unwrap();
                /*stream.read(&mut buffer).unwrap();
        println!("Reply 2: {}", String::from_utf8_lossy(&buffer[..]));*/
        println!("ok");     
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [1; 512];
    loop {
            match stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 { 
                        break; 
                    }
                    println!("Reply: {}", String::from_utf8_lossy(&buffer[..]));
                    buffer = [1; 512];
                    //stream.flush().unwrap();
                },
                Err(_error) => {
                    println!("ERROR");
                }
            }
    }

}
