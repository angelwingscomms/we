use std::sync::Arc;

use parking_lot::Mutex;

use crate::{auth::Auth, exams::Exams, line::Line, stuff, user, ylt::{self, YLT}};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    value: String,
    pub view: View,
    #[serde(skip)]
    pub loading: Arc<Mutex<bool>>,
    pub auth: Auth,
    pub line: Line,
    pub user: user::State,
    pub a: crate::a::A,
    pub stuff: crate::stuff::Stuff,
    
    pub ylt: YLT,

    pub f: F,

    pub e: Exams,

    pub search_tags: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
#[serde(default)]
pub struct F {
    q: String,
    a: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
#[serde(default)]
pub struct Result {
    i: i64,
    n: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub enum View {
    #[default]
    A,
    Auth,
    Exams,
    YLT,
    Line,
    UserSearch,
    Stuff,
    User,
    Edit,
}

impl App {
    /// Called once before the first frame.
    ///

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut a: Self = Default::default();
        a.a.page = 1;
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
                // let is_web = cfg!(target_arch = "wasm32");
                // if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("stuff").clicked() {
                        self.view = View::Stuff;
                    }
                    if ui.button("ylt").clicked() {
                        self.view = View::YLT;
                    }
                    if ui.button("a").clicked() {
                        self.view = View::A;
                    }
                    if ui.button("line").clicked() {
                        self.view = View::Line;
                    }
                    if self.auth.user.is_some() {
                        if ui.button("Edit profile").clicked() {
                            self.view = View::Edit;
                        }
                    }
                    if ui.button("Exams").clicked() {
                        self.view = View::Exams;
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                if let Some(user) = &self.auth.user {
                    if ui.button("logout").clicked() {
                        self.auth.users.lock().retain(|u| u.id != user.id);
                        self.auth.user = None;
                    }
                    if ui.button("switch user").clicked() {
                        self.view = View::Auth;
                    }
                } else {
                    if ui.button("login").clicked() {
                        self.view = View::Auth;
                    }
                }
                ui.add_space(16.0);
                // }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.view {
            View::A => crate::a::render(self, ui, ctx),
            View::Exams => self.exams(ui),
            View::Line => self.line(ui),
            View::Auth => self.auth(ui),
            View::YLT => ylt::ylt(self, ui),
            View::UserSearch => user::views::search(self, ui),
            View::Stuff => stuff::render(self, ui),
            View::User => user::views::user(self, ui),
            View::Edit => user::views::edit(self, ui),
        });
    }
}
