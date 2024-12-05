use std::sync::Arc;

use egui::mutex::Mutex;

use crate::{exams::Exams, line::Line};

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Auth {
    token: String,
    user: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    value: String,
    score: f32,
    res: Res,
    view: View,
    #[serde(skip)]
    pub loading: Arc<Mutex<bool>>,
    auth: Auth,
    pub line: Line,

    // a
    pub a: String,

    #[serde(skip)]
    pub aa: Arc<Mutex<Vec<AA>>>,
    pub sa: String,
    pub a_page: i64,
    #[serde(skip)]
    pub a_got: Arc<Mutex<bool>>,

    pub f: F,

    pub e: Exams,

    pub n: String,
    pub similarity: String,
    pub contact: Vec<Contact>,
    pub tags: Vec<String>,
    pub results: Vec<Result>,
    pub search_tags: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct F {
    q: String,
    a: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct AA {
    pub i: i64,
    pub t: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
struct Result {
    i: i64,
    n: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
struct Res {
    n: String,
    c: Vec<Contact>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Contact {
    pub n: String,
    pub l: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
enum View {
    #[default]
    A,
    Res,
    Search,
    Exams,
    Line,
    Edit,
}

impl App {
    /// Called once before the first frame.
    ///

    pub fn search(&mut self) {
        //todo
    }

    pub fn similarity(&mut self) {}

    pub fn get_res(&mut self, i: i64) {}

    pub fn save(&mut self) {}

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut a: Self = Default::default();
        a.a_page = 1;
        a
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                // if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("a").clicked() {
                        self.view = View::A;
                    }
                    if ui.button("line").clicked() {
                        self.view = View::Line;
                    }
                    if ui.button("Edit profile").clicked() {
                        self.view = View::Edit;
                    }
                    if ui.button("Exams").clicked() {
                        self.view = View::Exams;
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                // }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.view {
            View::A => {
                self.a(ui, ctx);
            }
            View::Exams => {
                self.exams(ui);
            }
            View::Line => {
                self.line(ui);
            }
            View::Edit => self.edit(ui),
            View::Search => {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.value);
                    if ui.button("Search").clicked() {
                        self.search();
                    }
                });

                for res in &mut self.results {
                    if ui.label(res.n.clone()).clicked() {
                        // self.get_res(res.i);
                        self.view = View::Res;
                    }
                }
            }

            View::Res => {
                ui.label(&self.res.n);
                ui.label("Similarity: ");
                ui.label(&self.similarity);
                ui.label(format!("Similarity score: {}", self.score));
                for contact in self.res.c.clone() {
                    ui.horizontal(|ui| ui.hyperlink_to(contact.n, contact.l));
                }
            }
        });
    }
}
