use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::mem::swap;
use std::thread::sleep;
use std::time::Duration;
use crate::wireworld::grid::{CellType, Grid};
use crate::wireworld::grid::CellType::{Conductor, ElectronHead};

type AdvanceCellF = dyn Fn(&Grid, usize, usize) -> CellType;

pub struct Environment {
    read_grid: Grid,
    write_grid: Grid,

    active_cells: HashSet<(usize, usize)>,
}

impl Environment {
    pub fn new(width: usize, height: usize, initial_cell_producer: fn(x: usize, y: usize) -> CellType) -> Environment {
        let read_grid = Grid::new(width, height, initial_cell_producer);
        let write_grid = Grid::new(width, height, initial_cell_producer);

        Environment {
            read_grid: read_grid,
            write_grid: write_grid,

            active_cells: HashSet::new(),
        }
    }

    pub fn new_empty(width: usize, height: usize) -> Environment {
        Self::new(width, height, |x, y| CellType::Empty)
    }

    fn swap_grids(&mut self) -> () {
        swap(&mut self.read_grid, &mut self.write_grid);
    }

    fn get_cell(&self, x: usize, y: usize) -> CellType {
        self.read_grid.get_cell(x, y)
    }

    fn set_cell(&mut self, x: usize, y: usize, cell_type: CellType) {
        self.write_grid.set_cell(x, y, cell_type);
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.read_grid.get_width(), self.read_grid.get_height())
    }

    pub fn bulk_set_readable(&mut self, cells: Vec<(usize, usize, CellType)>) {
        let (width, height) = self.get_dimensions();

        for (x, y, cell_type) in cells {
            if x >= 0 && x < width && y >= 0 && y < height {
                self.read_grid.set_cell(x, y, cell_type);
            } else {
                eprintln!("Could not set cell at {}, {}. Dimensions: ({}, {})", x, y, width, height);
            }
        }
    }

    pub fn advance(&mut self) {
        for y in 0..self.read_grid.get_height() {
            for x in 0..self.read_grid.get_width() {
                // println!("\t\tChecking {}, {}: {:?}", x, y, self.get_cell(x, y));
                let next_cell = match self.get_cell(x, y) {
                    CellType::Empty => CellType::Empty,
                    CellType::ElectronHead => CellType::ElectronTail,
                    CellType::ElectronTail => CellType::Conductor,
                    CellType::Conductor => {
                        let mut found_heads = 0;
                        for neighbor in self.read_grid.get_moore_neighborhood_around(x, y) {
                            if neighbor == CellType::ElectronHead {
                                found_heads += 1;
                            }
                        }
                        // println!("\tFound heads: {}", found_heads);

                        if found_heads == 1 || found_heads == 2 { ElectronHead } else { Conductor }
                    },
                };

                self.set_cell(x, y, next_cell);
            }
        }

        self.swap_grids();
    }

    pub fn main_loop(&mut self, max_iters: usize) {
        for _ in 0..max_iters {
            println!("{self:?}");

            self.advance();

            sleep(Duration::from_millis(166));
        }
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.read_grid.get_width();
        let height  = self.read_grid.get_height();

        let mut output: String = String::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let char = match self.get_cell(x, y) {
                    CellType::Empty => ' ',
                    CellType::ElectronHead => '#',
                    CellType::ElectronTail => '~',
                    CellType::Conductor => '+',
                };

                output.push(char);

                if x == width - 1 {
                    output.push('\n');
                }
            }
        }

        write!(f, "{}", output)
    }
}