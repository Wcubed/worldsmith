#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

mod widgets;

use crate::egui::Color32;
use crate::widgets::{color_click_to_copy, label_click_to_copy, star_size_comparison_chart};
use eframe::egui::{Context, Ui, Visuals};
use eframe::{egui, Frame};
use worldsmith_lib::units::{Kelvin, SolarDensity, SolarLuminosity, SolarMass, SolarRadius, Unit};
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

            // TODO (Wybe 2022-07-03): Cache this?
            let star = MainSequenceStar::calculate_parameters(SolarMass::new(self.solar_mass), 0.0);
            let color = Color32::from_rgb(star.color.r(), star.color.g(), star.color.b());

            egui::Grid::new("main_sequence_parameters")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Mass");
                    ui.add(
                        egui::Slider::new(&mut self.solar_mass, main_sequence_mass_range)
                            .logarithmic(true),
                    );
                    ui.label(SolarMass::SYMBOL).on_hover_text(SolarMass::NAME);
                    ui.end_row();

                    ui.label("Stellar class");
                    label_click_to_copy(ui, format!("{}", star.class));
                    // TODO (Wybe 2022-07-03): Add explanation of stellar class here.
                    ui.end_row();

                    // TODO (Wybe 2022-07-03): Allow copying by clicking a value (and change cursor to indicate you can click).
                    // TODO (Wybe 2022-07-03): Add a button to copy everything into the clipboard.
                    ui.label("Maximum age");
                    label_click_to_copy(ui, format!("{:.5}", star.max_age_gigayears));
                    ui.label("Gyr")
                        .on_hover_text("Giga earth years. 1 billion years (1,000,000,000)");
                    ui.end_row();

                    ui.label("Radius");
                    label_click_to_copy(ui, format!("{:.5}", star.radius));
                    ui.label(SolarRadius::SYMBOL)
                        .on_hover_text(SolarRadius::NAME);
                    ui.end_row();

                    ui.label("Luminosity");
                    label_click_to_copy(
                        ui,
                        &if star.luminosity < SolarLuminosity::new(1000.) {
                            format!("{:.5}", star.luminosity)
                        } else {
                            format!("{:.0}", star.luminosity)
                        },
                    );
                    ui.label(SolarLuminosity::SYMBOL)
                        .on_hover_text(SolarLuminosity::NAME);
                    ui.end_row();

                    ui.label("Density");
                    label_click_to_copy(ui, format!("{:.5}", star.density));
                    ui.label(SolarDensity::SYMBOL)
                        .on_hover_text(SolarDensity::NAME);
                    ui.end_row();

                    ui.label("Temperature");
                    label_click_to_copy(ui, format!("{:.0}", star.temperature));
                    ui.label(Kelvin::SYMBOL).on_hover_text(Kelvin::NAME);
                    ui.end_row();

                    // TODO WYBE: Add info on exactly what this color means.
                    ui.label("Color");
                    color_click_to_copy(ui, color);
                });

            star_size_comparison_chart(ui, star.radius);

            // todo: add a habitable zone comparison chart,
            //       showing where the orbits of the planets in the solar system are relative to
            //       the "habitable" zone of this star. Including how long the year lengths would be.
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
