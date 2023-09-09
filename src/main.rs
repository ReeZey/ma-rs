mod planet;
mod client;
mod rover;

use std::collections::VecDeque;
use std::fs;
use std::sync::Arc;
use flume::Sender;
use planet::Planet;
use planet::CellType;
use planet::Cell;
use rover::Rover;
use tokio::net::TcpListener;
use client::handle_client;
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mars = Planet::new(100);
    
    let cells =  mars.cells();
    let empty_spots: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Air).collect();

    println!("{:?}", empty_spots.len());

    //fs::write("map.txt", mars.print_vec()).unwrap();

    let clients = Arc::new(Mutex::new(vec![]));
    let server = TcpListener::bind("0.0.0.0:6969").await.unwrap();

    let server_uuid = Uuid::new_v4();

    let message_channel = flume::unbounded::<Message>();
    
    let client_pusher = clients.clone();
    let (sender, receiver) = message_channel.clone();
    tokio::spawn(async move {
        loop {
            let (stream, _sock_addr) = server.accept().await.unwrap();

            let uuid = Uuid::new_v4();
            client_pusher.lock().await.push(Client { uuid, send: sender.clone(), rover: None });
            handle_client(stream, uuid, sender.clone(), receiver.clone(), server_uuid);
        }
    });

    let (sender, receiver) = message_channel;
    loop {
        let message = match receiver.recv_async().await {
            Ok(message) => message,
            Err(_error) => {
                //println!("hejsan {:?}", _error);
                continue;
            },
        };
        
        //println!("message: {:#?}", message);
        if message.target != server_uuid {
            continue;
        }

        let message_string = match String::from_utf8(message.data) {
            Ok(message) => message,
            Err(_) => {
                println!("not a password");
                continue
            },
        };

        //println!("{:?}", message_string);


        let mut args: VecDeque<&str> = message_string.split(" ").collect();
        let command = args.pop_front().unwrap();

        let index = get_client_index(&clients, message.author).await.unwrap();
        let clients = clients.lock().await;
        let client = clients.get(index).unwrap();
        
        match command {
            "login" => {
                if args.len() != 2 {
                    sender.send(Message { author: server_uuid, target: client.uuid, data: "login failed.".as_bytes().to_vec() }).unwrap();
                    continue;
                }
                sender.send(Message { author: server_uuid, target: client.uuid, data: "login successful".as_bytes().to_vec() }).unwrap();
            },
            _ => {
                println!("unknown command: {:?} {:?}", command, args);
            }
        }
    }
}

async fn get_client_index(clients: &Arc<Mutex<Vec<Client>>>, uuid: Uuid) -> Option<usize> {
    let clients_aa = clients.lock().await;
    for (index, client) in clients_aa.iter().enumerate() {
        if client.uuid == uuid {
            return Some(index);
        }
    }
    return None;
}

#[derive(Debug)]
pub struct Message {
    author: Uuid,
    target: Uuid,
    data: Vec<u8>,
}

pub struct Client {
    uuid: Uuid,
    send: Sender<Message>,
    rover: Option<Rover>,
}
