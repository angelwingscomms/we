use anyhow::{anyhow, Context};
use egui::Align2;
use serde_json::json;

use crate::App;

impl App {
    pub fn a(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.a);
            if ui.button("🔍").clicked() {
                self.a_search(ctx);
            }
        });
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.sa);
            if ui.button("+").clicked() {
                self.a_add(ui, ctx);
            }
        });

        ui.add_space(27.0);

        if *self.loading.lock() {
            ui.label("loading...");
        } else {
            if *self.a_got.lock() {
                if self.aa.lock().len() > 0 {
                    for a in self.aa.lock().clone() {
                        ui.label(a.t);
                        ui.separator();
                    }
                } else {
                    if self.a_page > 1 {
                        ui.label("no more results");
                    } else {
                        ui.label("no results");
                    }
                }
                ui.add_space(27.0);
                ui.horizontal(|ui| {
                    if self.a_page > 1 {
                        if ui.button("previous").clicked() {
                            self.a_page -= 1;
                            self.a_search(ctx);
                        }
                    }
                    ui.label(format!("page {}", self.a_page));
                    if ui.button("next").clicked() {
                        self.a_page += 1;
                        self.a_search(ctx);
                    }
                });
            }
        }
    }

    pub fn a_search(&mut self, ctx: &egui::Context) {
        *self.loading.lock() = true;
        *self.aa.lock() = Vec::new();
        let aa = self.aa.clone();
        let loading = self.loading.clone();
        let a_got = self.a_got.clone();
        let c = ctx.clone();
        let mut url = format!("{}/a?q={}", "http://localhost:8000", self.a);
        // let mut url = format!("{}/a?q={}", dotenv_codegen::dotenv!("API"), self.a);
        if self.a_page > 1 {
            url.push_str(&format!("&p={}", self.a_page));
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

    pub fn a_add(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let c = ctx.clone();
        if let Ok(v) = serde_json::to_vec(&json!({"t": self.sa})) {
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
}
