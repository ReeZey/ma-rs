use std::{time::Duration, io::ErrorKind};
use flume::Sender;
use tokio::{net::TcpStream, io::Interest};
use uuid::Uuid;
use crate::Message;

pub fn handle_client(stream: TcpStream, uuid: Uuid, server_uuid: Uuid, send: Sender::<Message>) {    
    tokio::spawn(async move {
        let (client_send, client_recv) = flume::unbounded::<Message>();

        loop {
            let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();

            if ready.is_readable() {
                let mut data = vec![0; 1024];

                match stream.try_read(&mut data) {
                    Ok(n) => {
                        if n != 0 {
                            let data = data[0..n].to_vec();

                            let line = match String::from_utf8(data.clone()) {
                                Ok(line) => line,
                                Err(_) => "not utf8".to_owned(),
                            };

                            println!("read {} bytes, {:X?}, {:?}", data.len(), data, line);
                            
                            send.send(Message { author: uuid, target: server_uuid, data, response: Some(client_send.clone()) }).unwrap();
                        }
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        //println!("block");
                    }
                    Err(e) => {
                        println!("read error: {}", e.kind());
                        break;
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(10)).await;

            if ready.is_writable() {
                let message = match client_recv.recv_timeout(Duration::from_millis(10)) {
                    Ok(message) => message,
                    Err(_) => {
                        continue;
                    },
                };

                if message.target != uuid {
                    continue;
                }
                
                let line = String::from_utf8(message.data.clone()).unwrap();
                println!("test: {:?}", line);
                
                match stream.try_write(&message.data) {
                    Ok(n) => {
                        println!("write {} bytes", n);
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        println!("write error: {}", e.kind());
                        break;
                    }
                }
            }
        }
    });
}