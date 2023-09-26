use std::{time::Duration, io::ErrorKind};
use flume::{Receiver, Sender};
use tokio::{net::TcpStream, io::Interest};
use uuid::Uuid;
use crate::Message;

pub fn handle_client(stream: TcpStream, uuid: Uuid, server_uuid: Uuid, send: Sender::<Message>, recv: Receiver<Message>) {
    tokio::spawn(async move {
        loop {
            let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();

            if ready.is_readable() {
                let mut data = vec![0; 1024];

                match stream.try_read(&mut data) {
                    Ok(n) => {
                        println!("read {} bytes", n);
                        send.send_async(Message { author: uuid, target: server_uuid, data: data[0..n].to_vec() }).await.unwrap();
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

            if ready.is_writable() {
                let message = match recv.recv_timeout(Duration::from_millis(50)) {
                    Ok(message) => message,
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        continue
                    },
                };

                if message.target != uuid {
                    continue;
                }
                println!("yo3");
                println!("{:#?}", message);
                
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