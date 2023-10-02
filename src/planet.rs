use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::io::Write as fmt;

use bracket_noise::prelude::FastNoise;
use rand::Rng;
use strum::EnumIter;

#[derive(Debug)]
pub struct Planet {
    cells: Vec<Cell>,
    size: u32
}

impl Planet {
    pub fn new(size: u32) -> Planet {
        let mut cells = vec![];

        let mut rng = rand::thread_rng();

        let scatterness = 4;

        let mut cell_types = vec![];
        cell_types.push(CellType::Air);
        cell_types.push(CellType::Rock);
        //cell_types.push(CellType::Water);
        cell_types.push(CellType::Stone);
        cell_types.push(CellType::Bedrock);

        let mut noise = FastNoise::new();
        noise.set_seed(rng.gen_range(0..1000));

        for y in 0..size {
            for x in 0..size {
                let cell_types = cell_types.iter();
                let length = cell_types.len().pow(scatterness);
                let index = length - rng.gen_range(0..length);

                let mut cell_type = CellType::Air;

                let noise_value = noise.get_noise(x as f32 / 35.0, y as f32 / 35.0);

                if noise_value > 0.5 {
                    cell_type = CellType::Water;
                } else {
                    if noise_value < 0.0 {
                        for (i, ct) in cell_types.clone().enumerate() {
                            let number = cell_types.len() - (i + 1);
        
                            if index > number.pow(scatterness) {
                                cell_type = ct.clone();
                                break;
                            }
                        }
                    }
                }

                cells.push(Cell::new(cell_type, x as i32, y as i32));
            }
        }
        return Planet {
            cells,
            size
        }
    }

    pub fn cells(&self) -> Vec<Cell> {
        return self.cells.clone();
    }

    pub fn print_ascii(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];
        for (index, cell) in self.cells.iter().enumerate() {
            if index != 0 && index % (self.size as usize) == 0 {
                buffer.write(b"\n").unwrap();
            }
            write!(&mut buffer, "{}", cell.cell_type).unwrap();
        }
        return buffer;
    }

    pub fn color_buffer(&self) -> Vec<u8> {
        let mut buffer = vec![];

        for cell in self.cells.iter() {
            let cell_color = cell.cell_type.get_color();
            buffer.push(cell_color.r);
            buffer.push(cell_color.g);
            buffer.push(cell_color.b);
        }

        return buffer;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell>  {
        if x < 0 || y < 0 {
            return None;
        }
        
        return self.cells.get(x as usize + y as usize * self.size as usize);
    }

    pub fn get_cell_type(&self, x: i32, y: i32) -> CellType  {
        return match self.get_cell(x, y) {
            Some(cell) => cell.cell_type,
            None => CellType::Bedrock,
        };
    }

    pub fn set_celltype(&mut self, x: i32, y: i32, cell_type: CellType) {
        self.cells[x as usize + y as usize * self.size as usize].cell_type = cell_type;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(cell_type: CellType, x: i32, y: i32) -> Cell {
        return Cell { cell_type, x, y };
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum CellType {
    Air,
    Rock,
    Stone,
    Bedrock,
    Water,
    Rover
}

impl Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CellType::Air => f.write_char(' '),
            CellType::Rock => f.write_char('.'),
            CellType::Stone => f.write_char('o'),
            CellType::Bedrock => f.write_char('X'),
            CellType::Rover => f.write_char('R'),
            CellType::Water => f.write_char('W'),
        }
    }
}

impl CellTrait for CellType {
    fn get_color(&self) -> CellColor {
        match self {
            CellType::Air => CellColor { r: 250, g: 165, b: 0 },
            CellType::Rock => CellColor { r: 128, g: 100, b: 64 },
            CellType::Stone => CellColor { r: 64, g: 64, b: 64 },
            CellType::Bedrock => CellColor { r: 0, g: 0, b: 0 },
            CellType::Water => CellColor { r: 0, g: 0, b: 255 },
            CellType::Rover => CellColor { r: 255, g: 0, b: 0 },
        }
    }

    fn mineable(&self) -> bool {
        match self {
            CellType::Air => false,
            CellType::Rock => true,
            CellType::Stone => true,
            CellType::Bedrock => false,
            CellType::Water => false,
            CellType::Rover => false,
        }
    }
}

pub trait CellTrait {
    fn get_color(&self) -> CellColor;
    fn mineable(&self) -> bool;
}

pub struct CellColor {
    r: u8,
    g: u8,
    b: u8,
}