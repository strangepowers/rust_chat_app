use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::sync::{Arc,Mutex};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let socket_vec: Arc<RwLock<Vec<TcpStream>>> = Arc::new(RwLock::new(Vec::new()));
    let socket_vec = Arc::new(Mutex::new(Vec::new()));
 
    for stream in listener.incoming() {
        println!("check here");
        let new_stream = stream.unwrap();
        let cloned_socket_vec = socket_vec.clone();
        cloned_socket_vec.lock().unwrap().push(new_stream.try_clone().unwrap());
        let new_cloned_vec = socket_vec.clone();
        let read_stream = new_stream.try_clone().unwrap();
        //handle_connection(new_stream);
        //thread::spawn(|| {
        //    handle_connection(new_stream);
        //});
        thread::spawn(move || {
            handle_reads(read_stream, new_cloned_vec);
        });
        
        /*thread::spawn(move|| {
            let socket_vec = socket_vec.clone();
            handle_reads(read_stream, socket_vec);
        });*/

        println!("Connection established!");
    }
}

fn handle_reads(mut stream: TcpStream, ot_socket: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    continue; 
                }
                let msg = String::from_utf8_lossy(&buffer[..]);
                println!("MSG: {}", msg); 
                for mut socket in ot_socket.lock().unwrap().iter() {
                    socket.write(msg.as_bytes()).unwrap();
                }
            },
            Err(error) => {
                println!("{}", error);
                println!("error");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    //thread::sleep(Duration::from_secs(5));
    stream.write("test".as_bytes()).unwrap();
    let response = "grizz hello world";
    loop {
        thread::sleep(Duration::from_secs(3));
        stream.write(response.as_bytes()).unwrap();
    }
    //stream.flush().unwrap();
}
