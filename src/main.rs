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
    input_solar_mass: f32,
    input_age_gigayears: f32,
    star: MainSequenceStar,
}

impl WorldSmith {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        let input_solar_mass = 1.0;
        let input_age_gigayears = 100.0;
        let star =
            MainSequenceStar::calculate_parameters(input_solar_mass.into(), input_age_gigayears);

        WorldSmith {
            input_solar_mass,
            input_age_gigayears,
            star,
        }
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
            // TODO: How large should this range be?
            let main_sequence_age_range = 10.0..=1000.0;

            let color = Color32::from_rgb(
                self.star.color.r(),
                self.star.color.g(),
                self.star.color.b(),
            );

            egui::Grid::new("main_sequence_parameters")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    let previous_mass = self.input_solar_mass;
                    let previous_age = self.input_age_gigayears;

                    ui.label("Mass");
                    ui.add(
                        egui::Slider::new(&mut self.input_solar_mass, main_sequence_mass_range)
                            .logarithmic(true),
                    );
                    ui.label(SolarMass::SYMBOL).on_hover_text(SolarMass::NAME);
                    ui.end_row();

                    ui.label("Age");
                    ui.add(
                        egui::Slider::new(&mut self.input_age_gigayears, main_sequence_age_range)
                            .logarithmic(true),
                    );
                    ui.label("Gy")
                        .on_hover_text("Giga years (1.000.000.000 years)");
                    ui.end_row();

                    if self.input_solar_mass != previous_mass
                        || self.input_age_gigayears != previous_age
                    {
                        self.star = MainSequenceStar::calculate_parameters(
                            self.input_solar_mass.into(),
                            self.input_age_gigayears,
                        );
                    }

                    let star = &self.star;

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

            star_size_comparison_chart(ui, self.star.radius);

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
