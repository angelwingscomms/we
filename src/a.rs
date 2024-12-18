use std::sync::Arc;

use anyhow::{anyhow, Context};
use egui::Align2;
use parking_lot::Mutex;
use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct AA {
    pub i: i64,
    pub t: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct A {
    #[serde(skip)]
    pub aa: Arc<Mutex<Vec<AA>>>,
    pub a: String,
    pub sa: String,
    pub a_page: i64,
    #[serde(skip)]
    pub a_got: Arc<Mutex<bool>>,
}

pub fn a(app: &mut crate::App, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.a.a);
        if ui.button("🔍").clicked() {
            a_search(app, ctx);
        }
    });
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.a.sa);
        if ui.button("+").clicked() {
            a_add(app, ctx);
        }
    });

    ui.add_space(27.0);

    if *app.loading.lock() {
        ui.label("loading...");
    } else {
        if *app.a.a_got.lock() {
            if app.a.aa.lock().len() > 0 {
                for a in app.a.aa.lock().clone() {
                    ui.label(a.t);
                    ui.separator();
                }
            } else {
                if app.a.a_page > 1 {
                    ui.label("no more results");
                } else {
                    ui.label("no results");
                }
            }
            ui.add_space(27.0);
            ui.horizontal(|ui| {
                if app.a.a_page > 1 {
                    if ui.button("previous").clicked() {
                        app.a.a_page -= 1;
                        a_search(app, ctx);
                    }
                }
                ui.label(format!("page {}", app.a.a_page));
                if ui.button("next").clicked() {
                    app.a.a_page += 1;
                    a_search(app, ctx);
                }
            });
        }
    }
}

pub fn a_search(app: &mut crate::App, ctx: &egui::Context) {
    *app.loading.lock() = true;
    *app.a.aa.lock() = Vec::new();
    let aa = app.a.aa.clone();
    let loading = app.loading.clone();
    let a_got = app.a.a_got.clone();
    let c = ctx.clone();
    let mut url = format!("{}/a?q={}", "http://localhost:8000", app.a.a);
    // let mut url = format!("{}/a?q={}", dotenv_codegen::dotenv!("API"), app.a);
    if app.a.a_page > 1 {
        url.push_str(&format!("&p={}", app.a.a_page));
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

pub fn a_add(app: &mut crate::App, ctx: &egui::Context) {
    let c = ctx.clone();
    if let Ok(v) = serde_json::to_vec(&json!({"t": app.a.sa})) {
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
