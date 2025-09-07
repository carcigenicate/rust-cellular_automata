/*pub type CellType = i8;

pub struct Grid {
    cells: Vec<Vec<CellType>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, initial_cell_producer: fn(x: usize, y: usize) -> CellType) -> Grid {
        let mut vec_grid: Vec<Vec<T>> = Vec::with_capacity(height);
        for y in 0..height {
            let mut row: Vec<T> = Vec::with_capacity(width);
            for x in 0..width {
                row.push(initial_cell_producer(x, y));
            }
        }

        Grid {
            cells: vec_grid,
        }
    }

    pub fn get_width(&self) -> usize {
        self.cells[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellType {
        self.cells[y][x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: CellType) {
        self.cells[y][x] = value;
    }

    // TODO: Figure out how to make lazy. Either unstable generators, or make a 2D range iterator somehow
    pub fn get_moore_neighborhood_around(&self, center_x: usize, center_y: usize) -> Vec<CellType> {
        let mut neighbors: Vec<CellType> = vec![];

        let width = self.get_width();
        let height = self.get_height();

        let min_x = center_x.saturating_sub(1).clamp(0, width);
        let max_x = center_x.saturating_add(1).clamp(0, width);
        let min_y = center_y.saturating_sub(1).clamp(0, height);
        let max_y = center_y.saturating_add(1).clamp(0, height);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                neighbors.push(self.get_cell(x, y));
            }
        }

        neighbors
    }
}

*/