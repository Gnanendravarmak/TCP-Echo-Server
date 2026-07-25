use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener},
    thread,
};

fn main() {
    let addr = [
        SocketAddr::from(([127, 0, 0, 1], 8050)),
        SocketAddr::from(([127, 0, 0, 1], 8060)),
        SocketAddr::from(([127, 0, 0, 1], 8070)),
    ];

    let listner = TcpListener::bind(&addr[..]).expect("Failed to connect !!");

    println!("TCP Connection Started!!");
    println!(
        "Successfully listening on: {}",
        listner.local_addr().unwrap()
    );
    println!("Waiting for a client to connect...");

    for stream in listner.incoming() {
        match stream {
            Ok(mut res) => {
                println!("Client connected from: {:?}", res.peer_addr());
                let mut stream = res.try_clone().unwrap();
                let mut buffer = [0; 1024];

                thread::spawn(move || {
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(0) => {
                                // 0 bytes means the client disconnected
                                println!("\n[System]: Client disconnected.");
                                break;
                            }
                            Ok(result) => {
                                let data = String::from_utf8_lossy(&buffer[..result])
                                    .trim()
                                    .to_string();
                                println!("[Client]: {}", data);
                                // println!("{:?}: {:?}", stream.peer_addr(), data);
                                // res.write(&buffer[..result])
                                //     .expect("Failed to return the response");
                            }
                            Err(e) => {
                                println!("Failed to read the input: {:?}", e);
                                break;
                            }
                        }
                    }
                });

                let mut input_buffer = String::new();

                loop {
                    input_buffer.clear();
                    match io::stdin().read_line(&mut input_buffer) {
                        Ok(_) => {
                            println!("{:?}: {:?}", res.local_addr().unwrap(), input_buffer.trim());
                            if let Err(e) = res.write_all(input_buffer.as_bytes()) {
                                println!("[System]: Failed to send message: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                    io::stdout().flush().unwrap();
                }
            }
            Err(_) => {
                println!("Failed to read the input!!");
            }
        }
    }
}
