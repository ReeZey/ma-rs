use std::io::Write;

use rand::Rng;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Planet {
    cells: Vec<Cell>,
    size: u32
}

impl Planet {
    pub fn new(size: u32) -> Planet {
        let mut cells = vec![];

        let mut rng = rand::thread_rng();

        for x in 0..size {
            for y in 0..size {
                let cell_types = CellType::iter();
                let length = cell_types.len().pow(2);
                let index = length - rng.gen_range(0..length);

                let mut cell_type = CellType::Air;
                for (i, c) in cell_types.enumerate() {
                    if index < i {
                        cell_type = c;
                        break;
                    }
                }
                cells.push(Cell::new(cell_type, x, y));
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

    pub fn print_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];
        for (index, cell) in self.cells.iter().enumerate() {
            if index != 0 && index % (self.size as usize) == 0 {
                write!(&mut buffer, "\n").unwrap();
            }
            let char = match cell.cell_type {
                CellType::Air => {
                    ' '
                },
                CellType::Rock => {
                    '.'
                }
                CellType::Stone => {
                    'o'
                }
                CellType::Bedrock => {
                    'O'
                }
            };
            write!(&mut buffer, "{}", char).unwrap();
        }
        return buffer;
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell>  {
        return self.cells.get(x as usize + y as usize * self.size as usize);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    x: u32,
    y: u32,
}

impl Cell {
    pub fn new(cell_type: CellType, x: u32, y: u32) -> Cell {
        return Cell { cell_type, x, y };
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum CellType {
    Air,
    Rock,
    Stone,
    Bedrock
}