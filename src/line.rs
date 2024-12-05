use std::{thread, time::Duration};

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Line {
    text: String,
    play: bool,
    controls: bool,
    size: f32,
    height: f32,
    width: f32,
    speed: f32
}

use egui::{RichText, WidgetText};

use crate::App;

impl App {
    pub fn line(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .max_height(270.0)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.line.text);
                });
            ui.add_space(9.0);
            egui::ScrollArea::vertical()
                .id_salt("second scroll area")
                .max_height(self.line.height)
                .max_width(self.line.width)
                .show(ui, |ui| {
                    if ui.label(WidgetText::RichText(RichText::new(&self.line.text).size(self.line.size))).clicked() {
                        self.line.play = !self.line.play
                    };
                    if self.line.play {
                        ui.scroll_with_delta(egui::Vec2 { x: 0.0, y: -self.line.speed });
                    }
                });
            if ui
                .button({
                    if self.line.play {
                        "pause"
                    } else {
                        "play"
                    }
                })
                .clicked
            {
                self.line.play = !self.line.play;
            }
            if ui
                .button({
                    if self.line.controls {
                        "hide controls"
                    } else {
                        "show controls"
                    }
                })
                .clicked
            {
                self.line.controls = !self.line.controls;
            }
            if self.line.controls {
                ui.add(egui::Slider::new(&mut self.line.size, 0.0..=1080.0).text("font size"));
                ui.add(egui::Slider::new(&mut self.line.height, 0.0..=1080.0).text("height"));
                ui.add(egui::Slider::new(&mut self.line.width, 0.0..=1080.0).text("width"));
                ui.add(egui::Slider::new(&mut self.line.speed, 0.0..=1080.0).text("speed"));
            }
        });
    }
}

