use crate::wireworld::grid::CellType;

mod environment;
mod grid;
mod cell_types;
mod wireworld;

fn main() {
    let mut env = wireworld::environment::Environment::new_empty(10, 3);
    env.bulk_set_readable(vec![
        (2, 0, CellType::Conductor),
        (3, 0, CellType::Conductor),
        (4, 0, CellType::ElectronHead),
        (5, 0, CellType::ElectronTail),
        (6, 0, CellType::Conductor),
        (7, 0, CellType::Conductor),
        (8, 0, CellType::Conductor),

        (2, 1, CellType::Conductor),
        (8, 1, CellType::Conductor),

        (2, 2, CellType::Conductor),
        (3, 2, CellType::Conductor),
        (4, 2, CellType::Conductor),
        (5, 2, CellType::Conductor),
        (6, 2, CellType::Conductor),
        (7, 2, CellType::Conductor),
        (8, 2, CellType::Conductor),

    ]);

    env.main_loop(100);
}
