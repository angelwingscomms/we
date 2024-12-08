use std::{collections::HashMap, thread, time::Duration};

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Text {
    name: String,
    text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
#[serde(default)]
pub struct Line {
    texts: HashMap<i64, Text>,
    id: i64,
    next_id: i64,
    play: bool,
    show_texts: bool,
    show_input: bool,
    #[serde(skip)]
    commonmark_cache: CommonMarkCache,
    controls: bool,
    size: f32,
    height: f32,
    width: f32,
    speed: f32,
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            texts: self.texts.clone(),
            id: self.id.clone(),
            next_id: self.next_id.clone(),
            play: self.play.clone(),
            show_input: self.show_input.clone(),
            commonmark_cache: CommonMarkCache::default(),
            controls: self.controls.clone(),
            size: self.size.clone(),
            height: self.height.clone(),
            width: self.width.clone(),
            speed: self.speed.clone(),
            show_texts: self.show_texts.clone(),
        }
    }
}

use egui::{RichText, WidgetText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::App;

impl App {
    pub fn line(&mut self, ui: &mut egui::Ui) {
        if ui
            .button({
                if self.line.show_input {
                    "hide input"
                } else {
                    "show input"
                }
            })
            .clicked
        {
            self.line.show_input = !self.line.show_input;
        }
        if ui.button("new text").clicked() {
            println!("{}", self.line.next_id);
            self.line.texts.insert(
                self.line.next_id,
                Text {
                    name: String::new(),
                    text: String::new(),
                },
            );
            self.line.id = self.line.next_id;
            self.line.next_id += 1;
        }
        if !self.line.texts.is_empty() {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(t) = self.line.texts.get_mut(&self.line.id) {
                    ui.text_edit_singleline(&mut t.name);
                }
                if self.line.show_input {
                    egui::ScrollArea::vertical()
                        .max_height(270.0)
                        .show(ui, |ui| {
                            if let Some(t) = self.line.texts.get_mut(&self.line.id) {
                                ui.text_edit_multiline(&mut t.text);
                            }
                        });
                }
                ui.add_space(9.0);
                egui::ScrollArea::vertical()
                    .id_salt("second scroll area")
                    .max_height(self.line.height)
                    .max_width(self.line.width)
                    .show(ui, |ui| {
                        if ui.label(WidgetText::RichText(
                            RichText::new(&self.line.texts[&self.line.id].text)
                                .size(self.line.size),
                        )).clicked()
                        // CommonMarkViewer::new().show(ui, &mut self.line.commonmark_cache, &self.line.texts[&self.line.id].text)
                        // .response.clicked()
                        {
                            self.line.play = !self.line.play
                        };
                        if self.line.play {
                            ui.scroll_with_delta(egui::Vec2 {
                                x: 0.0,
                                y: -self.line.speed,
                            });
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
                    ui.add(
                        egui::Slider::new(&mut self.line.size, 0.0..=1080.0)
                            .text("font size")
                            .drag_value_speed(0.0108),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.line.height, 0.0..=1080.0)
                            .text("height")
                            .drag_value_speed(0.0108),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.line.width, 0.0..=1080.0)
                            .text("width")
                            .drag_value_speed(0.0108),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.line.speed, 0.0..=1080.0)
                            .text("speed")
                            .drag_value_speed(0.0108),
                    );
                }
            });
        }
        if ui
            .button({
                if self.line.show_texts {
                    "hide texts"
                } else {
                    "show texts"
                }
            })
            .clicked
        {
            self.line.show_texts = !self.line.show_texts;
        }
        if self.line.show_texts {
            for (id, text) in self.line.texts.clone() {
                ui.horizontal(|ui| {
                    if ui.button(&text.name).clicked() {
                        self.line.id = id;
                    }
                    if ui.button("delete").clicked() {
                        println!("{}", id);
                        if self.line.texts.contains_key(&id) {
                            self.line.texts.remove(&id);
                        }
                    }
                });
            }
        }
    }
}
