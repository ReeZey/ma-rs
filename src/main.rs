mod planet;
mod client;
mod rover;

use std::collections::VecDeque;
use std::fs;
use std::sync::Arc;
use image::ImageBuffer;
use image::RgbImage;
use planet::Planet;
use planet::CellType;
use planet::Cell;
use rand::Rng;
use rover::Rover;
use tokio::net::TcpListener;
use client::handle_client;
use tokio::sync::Mutex;
use uuid::Uuid;

static PLANET_SIZE: u32 = 20;

#[tokio::main]
async fn main() {
    let mars = Planet::new(PLANET_SIZE);
    
    let cells =  mars.cells();
    let air_cells: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Air).collect();
    let rock_cells: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Rock).collect();
    let stone_cells: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Stone).collect();
    let water_cells: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Water).collect();
    let bedrock_cells: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Bedrock).collect();

    println!("--- planet stats ---");
    println!("air: {} rock: {} stone: {} water: {} bedrock: {}", air_cells.len(), rock_cells.len(), stone_cells.len(), water_cells.len(),  bedrock_cells.len());

    fs::write("map.txt", mars.print_ascii()).unwrap();

    let clients = Arc::new(Mutex::new(vec![]));
    let server = TcpListener::bind("0.0.0.0:6969").await.unwrap();

    let server_uuid = Uuid::new_v4();

    let mut img: RgbImage = ImageBuffer::new(PLANET_SIZE, PLANET_SIZE);
    img.copy_from_slice(&mars.color_buffer());
    img.save("world.png").unwrap();

    let message_channel = flume::unbounded::<Message>();
    
    let client_pusher = clients.clone();
    let (sender, receiver) = message_channel.clone();
    tokio::spawn(async move {
        loop {
            let (stream, _sock_addr) = server.accept().await.unwrap();

            let uuid = Uuid::new_v4();
            client_pusher.lock().await.push(Client { uuid, rover: None });
            handle_client(stream, uuid, sender.clone(), receiver.clone(), server_uuid);
        }
    });

    let mut offline_rovers: Vec<Rover> = vec![];

    let mars: Arc<Mutex<Planet>> = Arc::new(Mutex::new(mars));
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
                println!("this is not utf8");
                continue
            },
        };

        //println!("{:?}", message_string);


        let mut args: VecDeque<&str> = message_string.split(" ").collect();
        let command = args.pop_front().unwrap();

        let index = get_client_index(&clients, message.author).await.unwrap();
        let mut clients_mutex = clients.lock().await;
        let client = clients_mutex.get_mut(index).unwrap();
        
        if client.rover.is_none() {
            if command != "login" {
                sender.send(Message { author: server_uuid, target: client.uuid, data: "not signed in".as_bytes().to_vec() }).unwrap();
                continue;
            }

            println!("login: {:?}", args);

            if args.len() != 2 {
                sender.send(Message { author: server_uuid, target: client.uuid, data: "login failed".as_bytes().to_vec() }).unwrap();
                continue;
            }

            let mut rover_index = None;
            for (index, r) in offline_rovers.iter().enumerate() {
                if args[0] == r.username {
                    rover_index = Some(index);
                    break;
                }
            }

            println!("{:?}", rover_index);

            if rover_index.is_some() {
                let rover_index = rover_index.unwrap();
                if offline_rovers[rover_index].password != args[1] {
                    sender.send(Message { author: server_uuid, target: client.uuid, data: "login failed".as_bytes().to_vec() }).unwrap();
                    continue;
                }
                let rover = offline_rovers.remove(rover_index);

                client.rover = Some(rover);
            } else {
                let cells =  mars.lock().await.cells();
                let empty_spots: Vec<&Cell> = cells.iter().filter(|a| a.cell_type == CellType::Air).collect();
    
                let mut rng = rand::thread_rng();
                let spawnpoint = empty_spots.get(rng.gen_range(0..empty_spots.len())).unwrap();
    
                client.rover = Some(Rover::new(args[0].to_owned(), args[1].to_owned(), spawnpoint.x, spawnpoint.y, mars.clone()));
            }

            println!("{:?} just logged on", args[0]);
            sender.send(Message { author: server_uuid, target: client.uuid, data: "login successful".as_bytes().to_vec() }).unwrap();
            continue;
        }
        let rover = client.rover.as_mut().unwrap();

        println!("{}: {} {:?}", rover.username, command, args);

        let mut planet = mars.lock().await;
        planet.set_celltype(rover.x, rover.y, CellType::Air);
        drop(planet);

        match command {
            "position" => {
                let message = rover.position();
                sender.send(Message { author: server_uuid, target: client.uuid, data: message.as_bytes().to_vec() }).unwrap();
            },
            "forward" => rover.forward().await,
            "turnleft" => rover.rotate(false).await,
            "turnright" => rover.rotate(true).await,
            "scan" => {
                let scan: String = rover.scan().await;
                sender.send(Message { author: server_uuid, target: client.uuid, data: scan.as_bytes().to_vec() }).unwrap();
            }
            "dig" => rover.dig().await,
            "disconnected" => {
                println!("{:?} just logged off", rover.username);
                offline_rovers.push(rover.clone());
            }
            _ => {
                println!("unknown command: {:?} {:?}", command, args);
            }
        }

        let mut planet = mars.lock().await;
        planet.set_celltype(rover.x, rover.y, CellType::Rover);

        /*
        let mut rovers = vec![];

        let clients = clients.lock().await;
        rovers.extend(offline_rovers.clone());
        for client in clients.iter() {
            if client.rover.is_some() {
                rovers.push(client.rover.clone().unwrap());
            }
        }
        
        planet.update_rovers(rovers);
        */

        let mut img: RgbImage = ImageBuffer::new(PLANET_SIZE, PLANET_SIZE);
        img.copy_from_slice(&planet.color_buffer());
        img.save("world.png").unwrap();
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

#[derive(Debug)]
pub struct Client {
    uuid: Uuid,
    rover: Option<Rover>,
}
