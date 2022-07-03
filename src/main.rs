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

            // TODO (Wybe 2022-07-03): Encode this in the MainSequenceStar struct?
            let main_sequence_mass_range = 0.075..=94.0;

            egui::Grid::new("main_sequence_parameters")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Mass");
                    ui.add(
                        egui::Slider::new(&mut self.solar_mass, main_sequence_mass_range)
                            .logarithmic(true),
                    );
                    ui.label("M☉").on_hover_text("Solar masses");
                    ui.end_row();

                    // TODO (Wybe 2022-07-03): Cache this?
                    let star = MainSequenceStar::calculate_parameters(self.solar_mass, 0.0);

                    ui.label("Stellar class");
                    ui.label(format!("{}", star.class));
                    // TODO (Wybe 2022-07-03): Add explanation of stellar class here.
                    ui.end_row();

                    // TODO (Wybe 2022-07-03): Allow copying by clicking a value (and change cursor to indicate you can click).
                    // TODO (Wybe 2022-07-03): Add a button to copy everything into the clipboard.
                    ui.label("Maximum age");
                    ui.label(format!("{:.5}", star.max_age_gigayears));
                    ui.label("Gyr")
                        .on_hover_text("Giga earth years. 1 billion years (1,000,000,000)");
                    ui.end_row();

                    ui.label("Radius");
                    ui.label(format!("{:.5}", star.solar_radius));
                    ui.label("R☉").on_hover_text("Solar radii");
                    ui.end_row();

                    ui.label("Luminosity");
                    ui.label(if star.solar_luminosity < 1000. {
                        format!("{:.5}", star.solar_luminosity)
                    } else {
                        format!("{:.0}", star.solar_luminosity)
                    });
                    ui.label("L☉").on_hover_text("Solar luminosities");
                    ui.end_row();

                    ui.label("Density");
                    ui.label(format!("{:.5}", star.solar_density));
                    ui.label("D☉").on_hover_text("Solar density");
                    ui.end_row();

                    ui.label("Temperature");
                    ui.label(format!("{:.0}", star.temperature));
                    ui.label("K").on_hover_text("Kelvin");
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
