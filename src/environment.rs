/*use std::mem::swap;
use crate::grid::{CellType, Grid};

type AdvanceCellF = dyn Fn(&Grid, usize, usize) -> CellType;

struct Environment {
    read_grid: Grid,
    write_grid: Grid,
    advance_cell_f: Box<AdvanceCellF>
}

impl Environment {
    fn new(width: usize, height: usize, initial_cell_producer: fn(x: usize, y: usize) -> CellType, advance_cell_f: Box<AdvanceCellF>) -> Environment {
        let read_grid = Grid::new(width, height, initial_cell_producer);
        let write_grid = Grid::new(width, height, initial_cell_producer);

        Environment {
            read_grid: read_grid,
            write_grid: write_grid,
            advance_cell_f: advance_cell_f,
        }
    }

    fn swap_grids(&mut self) -> () {
        swap(&mut self.read_grid, &mut self.write_grid);
    }

    pub fn advance(&mut self) {
        for y in 0..self.read_grid.get_height() {
            for x in 0..self.read_grid.get_width() {
                let new_cell_type = *self.advance_cell_f(&self.read_grid, x, y);
            }
        }
    }
}*/