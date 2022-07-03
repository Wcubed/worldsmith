#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

use crate::egui::{Ui, Visuals};
use eframe::egui::Context;
use eframe::{egui, Frame};
use worldsmith_lib::MainSequenceStar;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Worldsmith",
        native_options,
        Box::new(|cc| Box::new(WorldSmith::new(cc))),
    );
}

struct WorldSmith {
    solar_mass: f32,
}

impl WorldSmith {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        WorldSmith { solar_mass: 1.0 }
    }
}

impl eframe::App for WorldSmith {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.separator();
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Star calculator");

            let main_sequence_mass_range = 0.075..=100.0;

            ui.add(
                egui::Slider::new(&mut self.solar_mass, main_sequence_mass_range)
                    .logarithmic(true)
                    .text("M☉ (Solar masses)"),
            );

            // TODO (Wybe 2022-07-03): Cache this?
            let star = MainSequenceStar::calculate_parameters(self.solar_mass, 0.0);

            egui::Grid::new("main_sequence_parameters")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label(format!("{:.5}", star.solar_radius));
                    ui.label("R☉ (Solar radii)");
                    ui.end_row();

                    ui.label(if star.solar_luminosity < 1000. {
                        format!("{:.5}", star.solar_luminosity)
                    } else {
                        format!("{:.0}", star.solar_luminosity)
                    });
                    ui.label("L☉ (Solar radii)");
                    ui.end_row();
                });
        });
    }
}

fn global_dark_light_mode_switch(ui: &mut Ui) {
    let style = (*ui.ctx().style()).clone();
    let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
    if let Some(visuals) = new_visuals {
        ui.ctx().set_visuals(visuals);
    }
}
