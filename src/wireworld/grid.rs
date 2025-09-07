
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CellType {
    Empty = 0,
    ElectronHead = 1,
    ElectronTail = 2,
    Conductor = 3,
}

pub struct Grid {
    cells: Vec<Vec<CellType>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, initial_cell_producer: fn(x: usize, y: usize) -> CellType) -> Grid {
        let mut vec_grid: Vec<Vec<CellType>> = Vec::with_capacity(height);
        for y in 0..height {
            let mut row: Vec<CellType> = Vec::with_capacity(width);
            for x in 0..width {
                row.push(initial_cell_producer(x, y));
            }
            vec_grid.push(row);
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

    pub fn get_moore_neighborhood_around(&self, center_x: usize, center_y: usize) -> impl Iterator<Item = CellType> {
        get_moore_neighborhood_iterator(center_x, center_y, self.get_width(), self.get_height()).map(move |(x, y)| {
            // println!("Moore neighborhood around at ({}, {}): ({}, {})", center_x, center_y, x, y);
            self.get_cell(x, y)
        })
    }
}

pub struct TwoDimensionRangeIterator {
    current_x: usize,
    current_y: usize,

    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,

    done: bool,
}

impl TwoDimensionRangeIterator {
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> TwoDimensionRangeIterator {
        Self {
            current_x: min_x,
            current_y: min_y,

            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,

            done: false,
        }
    }
}

impl Iterator for TwoDimensionRangeIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = (self.current_x, self.current_y);

        self.current_x += 1;
        if self.current_x > self.max_x {
            self.current_x = self.min_x;
            self.current_y += 1;
        }

        if (self.current_y > self.max_y) {
            self.done = true;
        }

        return Some(current);
    }
}

fn get_moore_neighborhood_iterator(x: usize, y: usize, width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    let min_x = x.saturating_sub(1).clamp(0, width - 1);
    let max_x = x.saturating_add(1).clamp(0, width - 1);
    let min_y = y.saturating_sub(1).clamp(0, height - 1);
    let max_y = y.saturating_add(1).clamp(0, height - 1);

    // println!("Bounds: x ({min_x} {max_x}) y ({min_y} {max_y}) - Around: ({x} {y}) - Dimensions: ({width}, {height})");

    TwoDimensionRangeIterator::new(min_x, max_x, min_y, max_y).filter_map(move |current| {
        if current.0 == x && current.1 == y {
            None
        } else {
            Some(current)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_2d_range_iterator() {
        let range_iterator = TwoDimensionRangeIterator::new(2, 4, 2, 4);
        let range_coords: Vec<(usize, usize)> = range_iterator.collect();

        assert_eq!(range_coords.len(), 9);

        assert_eq!(range_coords[0], (2, 2));
        assert_eq!(range_coords[1], (3, 2));
        assert_eq!(range_coords[2], (4, 2));
        assert_eq!(range_coords[3], (2, 3));
        assert_eq!(range_coords[4], (3, 3));
        assert_eq!(range_coords[5], (4, 3));
        assert_eq!(range_coords[6], (2, 4));
        assert_eq!(range_coords[7], (3, 4));
        assert_eq!(range_coords[8], (4, 4));
    }

    #[test]
    fn test_get_2d_range_iterator_infinite_retro() {
        let range_iterator = TwoDimensionRangeIterator::new(1, 3, 0, 1);
        let range_coords: Vec<(usize, usize)> = range_iterator.collect();

        assert_eq!(range_coords, vec![(1, 0), (2, 0), (3, 0), (1, 1), (2, 1), (3, 1)]);

    }

    #[test]
    fn test_get_moore_neighborhood_iterator_full() {
        let neighborhood_iterator = get_moore_neighborhood_iterator(1, 1, 3, 3);
        let neighbor_coords: Vec<(usize, usize)> = neighborhood_iterator.collect();

        assert_eq!(neighbor_coords, vec![(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2)]);
    }

    #[test]
    fn test_get_moore_neighborhood_iterator_corner() {
        let neighborhood_iterator = get_moore_neighborhood_iterator(0, 0, 3, 3);
        let neighbor_coords: Vec<(usize, usize)> = neighborhood_iterator.collect();

        assert_eq!(neighbor_coords, vec![(1, 0), (0, 1), (1, 1)]);
    }

    #[test]
    fn test_get_moore_neighborhood_iterator_edge() {
        let neighborhood_iterator = get_moore_neighborhood_iterator(0, 1, 3, 3);
        let neighbor_coords: Vec<(usize, usize)> = neighborhood_iterator.collect();

        assert_eq!(neighbor_coords, vec![(0, 0), (1, 0), (1, 1), (0, 2), (1, 2)]);
    }

    #[test]
    fn test_get_moore_neighborhood_iterator_retro_infinite() {
        let neighborhood_iterator = get_moore_neighborhood_iterator(2, 0, 10, 3);

        let neighbor_coords: Vec<(usize, usize)> = neighborhood_iterator.collect();

        assert_eq!(neighbor_coords, [(1, 0), (3, 0), (1, 1), (2, 1), (3, 1)]);
    }
}
