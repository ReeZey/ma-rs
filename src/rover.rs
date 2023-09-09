use std::sync::Arc;
use tokio::sync::Mutex;
use crate::planet::{Planet, CellType};

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

        let new_posotion = Vector2 {x: self.x as i32 + motion.x, y: self.y as i32 + motion.y};

        let cell_type = planet.get_cell_type(new_posotion.x as u32, new_posotion.y as u32);

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
                    scanline += &planet.get_cell_type((self.x as i32 - index) as u32, self.y as u32 - 2).this_is_a_very_bad_fix();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type((self.x as i32 - index) as u32, self.y as u32 - 1).this_is_a_very_bad_fix();
                }
            },
            Compass::East => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type(self.x as u32 + 2, (self.y + index) as u32).this_is_a_very_bad_fix();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type(self.x as u32 + 1, (self.y + index) as u32).this_is_a_very_bad_fix();
                }
            },
            Compass::South => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type((self.x as i32 - index) as u32, self.y as u32 + 2).this_is_a_very_bad_fix();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type((self.x as i32 - index) as u32, self.y as u32 + 1).this_is_a_very_bad_fix();
                }
            },
            Compass::West => {
                for index in -2..3 {
                    scanline += &planet.get_cell_type((self.x as i32 - 2) as u32, (self.y - index) as u32).this_is_a_very_bad_fix();
                }
                for index in -1..2 {
                    scanline += &planet.get_cell_type((self.x as i32 - 1) as u32, (self.y - index) as u32).this_is_a_very_bad_fix();
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

        let cell_front = planet.get_cell((self.x + motion.x) as u32, (self.y + motion.y) as u32);
        if cell_front.is_none() {
            return;
        }
        let cell_front = cell_front.unwrap();

        if match cell_front.cell_type {
            CellType::Air => true,
            CellType::Rock => false,
            CellType::Stone => false,
            CellType::Bedrock => true,
        } {
            return;
        }
        
        let price = match cell_front.cell_type {
            CellType::Air => 0,
            CellType::Rock => 10,
            CellType::Stone => 100,
            CellType::Bedrock => 0,
        };

        let cell_x = cell_front.x;
        let cell_y = cell_front.y;

        planet.set_celltype(cell_x, cell_y, CellType::Air);
        self.points += price;
    }
}

#[derive(Debug)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Rover {
    pub fn new(username: String, x: i32, y: i32, planet: Arc<Mutex<Planet>>) -> Self {
        Self { username, x, y, planet: Some(planet), ..Default::default() }
    }
}

impl Default for Rover {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), points: Default::default(), username: "".into(), password: "".into(), rotation: Compass::North, planet: None }
    }
}