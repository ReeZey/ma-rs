use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::io::Write as fmt;

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

        let scatterness = 4;

        for x in 0..size {
            for y in 0..size {
                let cell_types = CellType::iter();
                let length = cell_types.len().pow(scatterness);
                let index = length - rng.gen_range(0..length);

                let mut cell_type = CellType::Air;
                for (i, ct) in cell_types.clone().enumerate() {
                    let number = cell_types.len() - (i + 1);

                    if index > number.pow(scatterness) {
                        cell_type = ct;
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
                buffer.write(b"\n").unwrap();
            }
            write!(&mut buffer, "{}", cell.cell_type).unwrap();
        }
        return buffer;
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell>  {
        return self.cells.get(x as usize + y as usize * self.size as usize);
    }

    pub fn get_cell_type(&self, x: u32, y: u32) -> CellType  {
        return match self.cells.get(x as usize + y as usize * self.size as usize) {
            Some(cell) => cell.cell_type,
            None => CellType::Bedrock,
        };
    }

    pub fn set_celltype(&mut self, x: u32, y: u32, cell_type: CellType) {
        self.cells[x as usize + y as usize * self.size as usize].cell_type = cell_type;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub x: u32,
    pub y: u32,
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

impl Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CellType::Air => f.write_char(' '),
            CellType::Rock => f.write_char('.'),
            CellType::Stone => f.write_char('o'),
            CellType::Bedrock => f.write_char('X'),
        }
    }
}

//TODO: better fix sometime
impl CellType {
    pub fn this_is_a_very_bad_fix(&self) -> String {
        return format!("{}", self);
    }
}