use std::{time::Duration, sync::Arc};

use flume::{Receiver, Sender};
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};
use uuid::Uuid;
use crate::Message;

pub fn handle_client(stream: TcpStream, uuid: Uuid, send: Sender::<Message>, recv: Receiver<Message>, server_uuid: Uuid) {
    let accessor = Arc::new(Mutex::new(stream));
    let alive = Arc::new(Mutex::new(true));

    let client_alive = alive.clone();
    let writer = accessor.clone();
    tokio::spawn(async move {
        while *client_alive.lock().await {
            let message = match recv.recv_timeout(Duration::from_millis(50)) {
                Ok(message) => message,
                Err(_) => continue,
            };

            if message.target != uuid {
                continue;
            }

            println!("received: {:?}", message.data);
            println!("received: {:?}", String::from_utf8(message.data.clone()).unwrap());

            let mut abc = writer.lock().await;
            abc.write(&message.data).await.unwrap();
            abc.flush().await.unwrap();
        }
    });

    let reader = accessor.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(50)).await;
            let mut buffer = vec![];
            match reader.lock().await.read_buf(&mut buffer).await {
                Ok(_) => {},
                Err(error) => {
                    println!("user disconnected: {}", error.kind());
                    break;
                }
            };
    
            //println!("{:?}", buffer);
    
            send.send_async(Message { author: uuid, target: server_uuid, data: buffer }).await.unwrap();
        }

        *alive.lock().await = false;
    });
    //return Client { uuid, send }
}