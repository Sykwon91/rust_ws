use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;
use std::time::Instant;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Spring-Damper Graph",
        options,
        Box::new(|_cc| Box::new(SpringApp::new())),
    )
}

struct SpringApp {
    dt: f32,
    local_position: f32,
    local_velocity: f32,
    external_force: f32,

    time: f64,
    history: VecDeque<[f64; 2]>,
    max_points: usize,

    last_update: Instant,
}

impl SpringApp {
    fn new() -> Self {
        Self {
            dt: 0.01,
            local_position: 0.0,
            local_velocity: 0.0,
            external_force: 1.0,

            time: 0.0,
            history: VecDeque::new(),
            max_points: 5000,

            last_update: Instant::now(),
        }
    }

    fn step_simulation(&mut self) {
        let local_acceleration =
            spring_damper(self.local_position, self.local_velocity, self.external_force);

        self.local_velocity += local_acceleration * self.dt;
        self.local_position += self.local_velocity * self.dt;
        self.time += self.dt as f64;

        self.history.push_back([self.time, self.local_position as f64]);

        if self.history.len() > self.max_points {
            self.history.pop_front();
        }
    }
}

impl eframe::App for SpringApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while self.last_update.elapsed().as_secs_f32() >= self.dt {
            self.step_simulation();
            self.last_update = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Spring-Damper Simulation");

            ui.horizontal(|ui| {
                ui.label(format!("time: {:.3} s", self.time));
                ui.label(format!("position: {:.5}", self.local_position));
                ui.label(format!("velocity: {:.5}", self.local_velocity));
            });

            ui.add(egui::Slider::new(&mut self.external_force, -10.0..=10.0).text("external force"));

            if ui.button("Reset").clicked() {
                self.local_position = 0.0;
                self.local_velocity = 0.0;
                self.time = 0.0;
                self.history.clear();
            }

            let points: PlotPoints = self.history.iter().copied().collect();
            let line = Line::new(points);

            Plot::new("position_plot")
                .height(400.0)
                .allow_scroll(false)
                .allow_zoom(false)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });

        ctx.request_repaint();
    }
}

fn spring_damper(local_position: f32, local_velocity: f32, external_force: f32) -> f32 {
    let k = 1.0f32;
    let c = 0.707f32;
    let m = 1.0f32;

    (-k * local_position - c * local_velocity + external_force) / m
}