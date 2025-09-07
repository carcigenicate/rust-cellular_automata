use rand::Rng;
use crate::wireworld::grid::CellType;
use std::time::Instant;

mod environment;
mod grid;
mod cell_types;
mod wireworld;

fn main() {

    let width = 20;
    let height = 20;

    let mut env = wireworld::environment::Environment::new_empty(width, height);

    for x in 1..width {
        env.bulk_set_readable(vec![(x, 1, CellType::Conductor)]);
        env.bulk_set_readable(vec![(x, 2, CellType::Conductor)]);

        env.bulk_set_readable(vec![(x, height - 1, CellType::Conductor)]);
        env.bulk_set_readable(vec![(x, height - 2, CellType::Conductor)]);
    }

    for y in 1..height {
        env.bulk_set_readable(vec![(1, y, CellType::Conductor)]);

        env.bulk_set_readable(vec![(width - 1, y, CellType::Conductor)]);
    }

    env.bulk_set_readable(vec![(5, 1, CellType::ElectronHead)]);
    env.bulk_set_readable(vec![(5, 2, CellType::ElectronHead)]);
    env.bulk_set_readable(vec![(4, 1, CellType::ElectronTail)]);
    env.bulk_set_readable(vec![(4, 2, CellType::ElectronTail)]);

    env.bulk_set_readable(vec![(5, height - 1, CellType::ElectronHead)]);
    env.bulk_set_readable(vec![(5, height - 2, CellType::ElectronHead)]);
    env.bulk_set_readable(vec![(6, height - 1, CellType::ElectronTail)]);
    env.bulk_set_readable(vec![(6, height - 2, CellType::ElectronTail)]);

    let start_time = Instant::now();

    wireworld::ui::egui::start_gui(env).unwrap();

    let duration = start_time.elapsed();
    println!("Time elapsed in main_loop() is: {:?}", duration);  // 16.7 seconds for 0.5 chance
}
