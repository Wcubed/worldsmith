#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

use crate::egui::{Ui, Visuals};
use eframe::egui::Context;
use eframe::{egui, Frame};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Worldsmith",
        native_options,
        Box::new(|cc| Box::new(WorldSmith::new(cc))),
    );
}

struct WorldSmith;

impl WorldSmith {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        WorldSmith
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
            ui.heading("Hello World!");
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
