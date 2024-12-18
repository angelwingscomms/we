use std::sync::Arc;

use anyhow::{anyhow, Context};
use egui::Align2;
use parking_lot::Mutex;
use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Result {
    pub i: i64,
    pub t: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Stuff {
    pub search: String,
    pub new: String,
    pub page: i64,
    #[serde(skip)]
    pub results: Arc<Mutex<Vec<Result>>>,
    #[serde(skip)]
    pub got: Arc<Mutex<bool>>,
    pub current: Option<Result>,
    pub view: View,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
pub enum View {
    #[default]
    Top,
    Result,
}

pub fn render(app: &mut crate::App, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.stuff.search);
        if ui.button("🔍").clicked() {
            search(app, ui.ctx());
        }
    });
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.stuff.new);
        if ui.button("+").clicked() {
            add(app, ui.ctx());
        }
    });

    ui.add_space(27.0);

    if *app.loading.lock() {
        ui.label("loading...");
    } else {
        if *app.stuff.got.lock() {
            if app.stuff.results.lock().len() > 0 {
                if let Some(current) = &app.stuff.current {
                    ui.label(&current.t);
                    ui.add_space(9.0);
                }
                let results = app.stuff.results.lock().clone();
                for a in results {
                    if ui.label(&a.t).clicked() {
                        search(app, ui.ctx());
                        app.stuff.current = Some(a);
                    };
                    ui.separator();
                }
            } else {
                if app.stuff.page > 1 {
                    ui.label("no more results");
                } else {
                    ui.label("no results");
                }
            }
            ui.add_space(27.0);
            ui.horizontal(|ui| {
                if app.stuff.page > 1 {
                    if ui.button("previous").clicked() {
                        app.stuff.page -= 1;
                        search(app, ui.ctx());
                    }
                }
                ui.label(format!("page {}", app.stuff.page));
                if ui.button("next").clicked() {
                    app.stuff.page += 1;
                    search(app, ui.ctx());
                }
            });
        }
    }
}

pub fn search(app: &mut crate::App, ctx: &egui::Context) {
    *app.loading.lock() = true;
    *app.stuff.results.lock() = Vec::new();
    let aa = app.stuff.results.clone();
    let loading = app.loading.clone();
    let a_got = app.stuff.got.clone();
    let c = ctx.clone();
    // let mut url = format!("{}/a?q={}", dotenv_codegen::dotenv!("API"), app.a);
    let mut url = format!("{}/a?q={}", "http://localhost:8000", app.stuff.search);
    if let Some(current) = &app.stuff.current {
        url.push_str(&format!("&i={}", current.i));
    }
    if app.stuff.page > 1 {
        url.push_str(&format!("&p={}", app.stuff.page));
    }
    ehttp::fetch(
        ehttp::Request::get(url),
        move |result: ehttp::Result<ehttp::Response>| {
            let r = |result: ehttp::Result<ehttp::Response>| -> anyhow::Result<()> {
                let res = result
                    .map_err(|e| anyhow!(e.to_string()))
                    .context("ehttp result")?;
                let res_text_result = res.text();
                println!("res_text_result: {:#?}", res_text_result);
                *aa.lock() = serde_json::from_str(
                    res_text_result
                        .ok_or(anyhow!("empty ehttp result"))
                        .context("response.text()")?,
                )
                .context("serde_json from string")?;
                if !*a_got.lock() {
                    *a_got.lock() = true
                };
                c.request_repaint();
                Ok(())
            };
            match r(result) {
                Ok(_) => {
                    println!("{:#?}", *aa.lock())
                }
                Err(e) => {
                    println!("e: {}", e.to_string());
                    let mut toasts = egui_toast::Toasts::new()
                        .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                        .direction(egui::Direction::TopDown);
                    toasts.add(egui_toast::Toast {
                        text: format!("fetch error: {}", e.to_string()).into(),
                        kind: egui_toast::ToastKind::Error,
                        options: egui_toast::ToastOptions::default()
                            .duration_in_seconds(10.0)
                            .show_progress(true)
                            .show_icon(true),
                        style: egui_toast::ToastStyle::default(),
                    });
                    c.request_repaint();
                    toasts.show(&c);
                }
            };
            *loading.lock() = false;
        },
    );
    // ctx.request_repaint();
}

pub fn add(app: &mut crate::App, ctx: &egui::Context) {
    let c = ctx.clone();
    if let Ok(v) = serde_json::to_vec(&json!({"t": app.stuff.new})) {
        let mut request = ehttp::Request::post("http://127.0.0.1:8000/a", v);
        request.headers.insert("Content-Type", "application/json");
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            let r = |result: ehttp::Result<ehttp::Response>| -> anyhow::Result<()> {
                let res = result
                    .map_err(|e| anyhow!(e.to_string()))
                    .context("ehttp result")?;
                let res_text_result = res.text();
                println!("res_text_result: {:#?}", res_text_result);
                // *aa.lock() = serde_json::from_str(
                //     res_text_result
                //         .ok_or(anyhow!("empty ehttp result"))
                //         .context("response.text()")?,
                // )
                // .context("serde_json from string")?;
                Ok(())
            };
            if let Err(e) = r(result) {
                println!("e: {}", e.to_string());
                let mut toasts = egui_toast::Toasts::new()
                    .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                    .direction(egui::Direction::TopDown);
                toasts.add(egui_toast::Toast {
                    text: format!("fetch error: {}", e.to_string()).into(),
                    kind: egui_toast::ToastKind::Error,
                    options: egui_toast::ToastOptions::default()
                        .duration_in_seconds(10.0)
                        .show_progress(true)
                        .show_icon(true),
                    style: egui_toast::ToastStyle::default(),
                });
                toasts.show(&c);
            };
        });
    } else {
        println!("to_vec failed"); //todo
    }
}
