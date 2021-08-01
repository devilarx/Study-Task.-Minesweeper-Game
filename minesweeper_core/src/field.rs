
use std::vec::Vec;
use std::clone::Clone;
use std::fmt;
use std::error::Error;

use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum CellState{
    Opened,
    Closed,
    Flagged,
    Suspected,
    UnderPressing
}

#[derive(Debug, Copy, Clone)]
pub struct Cell{
    pub state: CellState,
    pub is_mine: bool,
    pub mines_around: u8,
    pub x_pos: u8,
    pub y_pos: u8
}

impl Cell{
    fn new() -> Cell{
        Cell{
            state: CellState::Closed,
            is_mine: false,
            mines_around: 0,
            x_pos: 0, 
            y_pos: 0
        }
    }
}

pub struct Field{
    cells: Vec<Vec<Cell>>
}

// Minesweeper errors implementation

#[derive(Debug, Clone)]
pub enum MinesweeperError{
    FieldCreateError(&'static str),
    UnexpectedError(&'static str)
}

impl fmt::Display for MinesweeperError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match *self{
            MinesweeperError::FieldCreateError(err) => write!(f, "Field Create Error: {}", err),
            MinesweeperError::UnexpectedError(err) => write!(f, "Unexpected error: {}", err)
        }
    }
}

impl Error for MinesweeperError{
    fn description(&self) -> &str {
        match *self {
            MinesweeperError::FieldCreateError(err) => err,
            MinesweeperError::UnexpectedError(err) => err,
        }
    }
}

impl Field{
    pub fn new(width: u8, height: u8, x : u8, y: u8, mines_count: u16) -> Result<Field, MinesweeperError>{

        // Create Field
        if (width as u16 * height as u16 - 2) < mines_count{
            return Err(MinesweeperError::FieldCreateError("There are more mines than cells on the field"));
        }
        if x> width || y > height {
            return Err(MinesweeperError::FieldCreateError("Start coordinates outside the field"));
        }

        let mut field = Field{
            cells: vec![vec![Cell::new(); width as usize]; height as usize]
        };
        
        // Generate mines
        let mut cells_vec: Vec<(u8,u8)> = Vec::new();

        for j in 0..field.cells.len(){
            for i in 0..field.cells[j].len(){
                field.cells[j][i].x_pos = i as u8;
                field.cells[j][i].y_pos = j as u8;
                if i as u8 == x && j as u8 == y{
                    continue;
                }
                cells_vec.push((i as u8, j as u8));
            }           
        }

        cells_vec.shuffle(&mut thread_rng());

        for i in 0..mines_count{
            let (x , y) = cells_vec[i as usize];
            field.cells[y as usize][x as usize].is_mine = true;
        }

        // Count mines around every cells
        field.count_mines_around(width, height);        
        Ok(field)
    }

    fn count_mines_around(&mut self, width : u8, height : u8){
        for j in 0..height{
            for i in 0..width{
                let mut count = 0;
                count += if j>0 && i>0 && self.cells[j as usize -1][i as usize -1].is_mine {1} else {0};
                count += if j>0 && self.cells[j as usize -1][i as usize].is_mine {1} else {0};
                count += if j>0 && i<width-1 && self.cells[j as usize -1][i as usize +1].is_mine {1} else {0};

                count += if j>0 && i>0 && self.cells[j as usize][i as usize -1].is_mine {1} else {0};
                count += if j>0 && i+1<width && self.cells[j as usize][i as usize +1].is_mine {1} else {0};

                count += if j+1<height && i>0 && self.cells[j as usize+1][i as usize-1].is_mine {1} else {0};
                count += if j+1<height && self.cells[j as usize+1][i as usize].is_mine {1} else {0};
                count += if j+1<height && i+1 < width && self.cells[j as usize+1][i as usize+1].is_mine {1} else {0};
                self.cells[j as usize][i as usize].mines_around = count;
            }
        }
    }

    pub fn is_mine(&self, x : usize, y : usize) -> bool{
        self.cells[y][x].is_mine
    }

    pub fn get_mines_around(&self, x : usize, y: usize) -> u8{
        self.cells[y][x].mines_around
    }
}