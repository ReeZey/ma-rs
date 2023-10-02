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
                            println!("read {} bytes", n);
                            send.send(Message { author: uuid, target: server_uuid, data: data[0..n].to_vec(), response: Some(client_send.clone()) }).unwrap();
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
                let message = match client_recv.recv_timeout(Duration::from_millis(50)) {
                    Ok(message) => message,
                    Err(_) => {
                        continue
                    },
                };

                if message.target != uuid {
                    continue;
                }
                
                match stream.try_write(&message.data) {
                    Ok(n) => {
                        println!("write {} bytes", n);
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        continue
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