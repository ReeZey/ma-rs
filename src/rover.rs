use std::sync::Arc;
use tokio::sync::Mutex;
use crate::planet::{Planet, CellType, CellTrait};

#[derive(Debug, Clone)]
pub struct Rover {
    pub username: String,
    pub password: String,
    pub x: i32,
    pub y: i32,
    pub rotation: Compass,
    pub points: u32,
    pub planet: Option<Arc<Mutex<Planet>>>,
}

impl Rover {
    pub async fn forward(&mut self) {
        let mut motion = Vector2 { x: 0, y: 0};
        match self.rotation {
            Compass::North => motion.y -= 1,
            Compass::East => motion.x += 1,
            Compass::South => motion.y += 1,
            Compass::West => motion.x -= 1,
        }

        let planet = self.planet.as_mut().unwrap().lock().await;

        let new_posotion = Vector2 {x: self.x + motion.x, y: self.y + motion.y};

        let cell_type = planet.get_cell_type(new_posotion.x, new_posotion.y);

        if cell_type != CellType::Air {
            return;
        }

        self.x = new_posotion.x;
        self.y = new_posotion.y;
    }
    pub async fn rotate(&mut self, clockwise: bool) {
        if clockwise {
            match self.rotation {
                Compass::North => self.rotation = Compass::East,
                Compass::East => self.rotation = Compass::South,
                Compass::South => self.rotation = Compass::West,
                Compass::West => self.rotation = Compass::North,
            }
        } else {
            match self.rotation {
                Compass::North => self.rotation = Compass::West,
                Compass::East => self.rotation = Compass::North,
                Compass::South => self.rotation = Compass::East,
                Compass::West => self.rotation = Compass::South,
            }
        }
    }
    pub fn position(&self) -> String {
        return format!("Position x:{} y:{} Direction:{:?}", self.x, self.y, self.rotation);
    }
    pub async fn scan(&mut self) -> String {
        let planet = self.planet.as_mut().unwrap().lock().await;

        let mut scanline = String::new();
        match self.rotation {
            Compass::North => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type(self.x + index, self.y - 2).to_string();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type(self.x + index, self.y - 1).to_string();
                }
            },
            Compass::East => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type(self.x + 2, self.y + index).to_string();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type(self.x + 1, self.y + index).to_string();
                }
            },
            Compass::South => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type(self.x + index, self.y + 2).to_string();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type(self.x + index, self.y + 1).to_string();
                }
            },
            Compass::West => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type(self.x - 2, self.y + index).to_string();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type(self.x - 1, self.y + index).to_string();
                }
            },
        }
        
        return scanline;
    }
    pub async fn dig(&mut self) {
        let mut planet = self.planet.as_mut().unwrap().lock().await;
        
        let motion = match self.rotation {
            Compass::North => Vector2 {x: 0, y: -1},
            Compass::East => Vector2 {x: 1, y: 0},
            Compass::South => Vector2 {x: 0, y: 1},
            Compass::West => Vector2 {x: -1, y: 0},
        };

        let cell_front = planet.get_cell(self.x + motion.x, self.y + motion.y);
        
        //println!("front: {:#?}", cell_front);

        if cell_front.is_none() {
            return;
        }
        let cell_front = cell_front.unwrap();

        if !cell_front.cell_type.mineable() {
            return;
        }
        
        let price = match cell_front.cell_type {
            CellType::Air => 0,
            CellType::Rock => 10,
            CellType::Stone => 100,
            CellType::Bedrock => 0,
            CellType::Rover => 0,
            CellType::Water => 0,
        };

        let cell_x = cell_front.x;
        let cell_y = cell_front.y;

        //println!("updated");
        planet.set_celltype(cell_x, cell_y, CellType::Air);
        self.points += price;
    }
}


#[derive(Debug, Clone)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Rover {
    pub fn new(username: String, password: String, x: i32, y: i32, planet: Arc<Mutex<Planet>>) -> Self {
        Self { username, password, x, y, planet: Some(planet), ..Default::default() }
    }
}

impl Default for Rover {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), points: Default::default(), username: "".into(), password: "".into(), rotation: Compass::North, planet: None }
    }
}