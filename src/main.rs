use rand::Rng;
use crate::wireworld::grid::CellType;
use std::time::Instant;

mod environment;
mod grid;
mod cell_types;
mod wireworld;

fn main() {

    let width = 25;
    let height = 25;

    let mut env = wireworld::environment::Environment::new_empty(width, height);
    // env.bulk_set_readable(vec![
    //     // (2, 0, CellType::Conductor),
    //     // (3, 0, CellType::Conductor),
    //     // (4, 0, CellType::ElectronHead),
    //     // (5, 0, CellType::ElectronTail),
    //     // (6, 0, CellType::Conductor),
    //     // (7, 0, CellType::Conductor),
    //     // (8, 0, CellType::Conductor),
    //     //
    //     // (2, 1, CellType::Conductor),
    //     // (8, 1, CellType::Conductor),
    //     //
    //     // (2, 2, CellType::Conductor),
    //     // (3, 2, CellType::Conductor),
    //     // (4, 2, CellType::Conductor),
    //     // (5, 2, CellType::Conductor),
    //     // (6, 2, CellType::Conductor),
    //     // (7, 2, CellType::Conductor),
    //     // (8, 2, CellType::Conductor),
    //
    //
    // ]);

    let mut rand_gen = rand::rng();

    for y in 0..height {
        for x in 0..width {
            let cell_type = if rand_gen.random_bool(0.5) { CellType::Conductor } else { CellType::Empty };
            env.bulk_set_readable(vec![(x, y, cell_type)]);
        }
    }

    env.bulk_set_readable(vec![(0, 0, CellType::ElectronHead)]);

    let start_time = Instant::now();
    env.main_loop(100);

    let duration = start_time.elapsed();
    println!("Time elapsed in main_loop() is: {:?}", duration);  // 16.7 seconds for 0.5 chance
}
