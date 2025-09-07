extern crate eframe;


use std::time::Duration;
use eframe::{egui,};
use eframe::egui::{Color32, Pos2, Rect};
use crate::wireworld::environment::Environment;
use crate::wireworld::grid::CellType;

const INITIAL_WINDOW_SIZE: [usize; 2] = [750, 750];

struct GuiState {
    env: Environment,
}

impl GuiState {
    fn new(env: Environment) -> Self {
        GuiState {
            env: env,
        }
    }
}

pub fn start_gui(env: Environment) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([INITIAL_WINDOW_SIZE[0] as f32, INITIAL_WINDOW_SIZE[1] as f32]),
        ..Default::default()
    };

    let state = GuiState {
        env: env,
    };

    eframe::run_native(
        "Wireworld",
        options,
        Box::new(|cc| {
            return Ok(Box::<GuiState>::new(state));
        }),
    )
}

impl GuiState {
    fn calculate_pixel_width(&self, ctx: &egui::Context) -> f32 {
        let (env_width, env_height) = self.env.get_dimensions();
        let window_rect = ctx.input(|i| i.viewport().inner_rect.unwrap());
        let window_width = window_rect.width();
        let window_height = window_rect.height();

        (window_width / env_width as f32).min(window_height / env_height as f32)
    }

    fn window_dimensions(&self, ctx: &egui::Context) -> (f32, f32) {
        let window_rect = ctx.input(|i| i.viewport().inner_rect.unwrap());

        (window_rect.width() as f32, window_rect.height() as f32)
    }
}

fn map_dimension(value: f32, current_min: f32, current_max: f32, new_min: f32, new_max: f32) -> f32 {
    let current_range = current_max - current_min;
    let current_perc = (value - current_min) / (current_range - current_min);

    let new_range = new_max - new_min;
    return (new_range * current_perc) + new_min;
}

fn cell_color(cell_type: CellType) -> Color32 {
    match cell_type {
        CellType::Empty => Color32::BLACK,
        CellType::Conductor => Color32::YELLOW,
        CellType::ElectronHead => Color32::RED,
        CellType::ElectronTail => Color32::BLUE,
    }
}

impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (window_width, window_height) = self.window_dimensions(ctx);
        let block_width = self.calculate_pixel_width(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let painter = ui.painter();
                let (env_width, env_height) = self.env.get_dimensions();


                for env_y in 0..env_height {
                    let window_y = map_dimension(env_y as f32, 0f32, env_height as f32, 0f32, window_height);
                    for env_x in 0..env_width {
                        let window_x = map_dimension(env_x as f32, 0f32, env_width as f32, 0f32, window_width);
                        let rect = Rect {
                            min: Pos2 { x: window_x, y: window_y },
                            max: Pos2 { x: window_x + block_width, y: window_y + block_width }
                        };
                        let color = cell_color(self.env.get_cell(env_x as usize, env_y as usize));
                        painter.rect_filled(rect, 1.0, color);
                    }
                }
            });
        });

        self.env.advance();
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}