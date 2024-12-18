use crate::{app::View, App};

use super::save;

pub fn search(app: &mut App, ui: &mut egui::Ui) {
    ui.text_edit_multiline(&mut app.user.search);
}

pub fn user(app: &mut App, ui: &mut egui::Ui) {
    if let Some(u) = &app.auth.user {
        if u.id == *app.user.user.lock().i {
            if ui.button("edit profile").clicked() {
                app.view = View::Edit;
            }
        }
        if ui.button("show referral link").clicked() {
            ui.label(&u.r);
        }
    } else {
        ui.label(format!(
            "Similarity: {}%",
            app.user.user.lock().x.to_string()
        ));
    }
    ui.add_space(9.0);
    ui.label(&app.user.user.lock().u);
    ui.add_space(9.0);
    ui.label(format!("similarity: {}", app.user.user.lock().x));
    ui.horizontal(|ui| {
        for t in &app.user.user.lock().t {
            ui.label(format!("{t}"));
        }
    });
    ui.add_space(9.0);
    ui.label("description");
    ui.label(&app.user.user.lock().d);
    for contact in app.user.user.lock().c.clone() {
        ui.horizontal(|ui| ui.hyperlink_to(contact.n, contact.t));
    }
}

pub fn edit(app: &mut App, ui: &mut egui::Ui) {
    ui.label("username");
    ui.text_edit_singleline(&mut app.user.edit.u);
    for (i, tag) in app.user.edit.t.clone().iter_mut().enumerate() {
        ui.text_edit_singleline(tag);
        if ui.button("X").clicked() {
            app.user.edit.t.remove(i);
        }
    }
    for contact in &mut app.user.edit.c {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut contact.n);
            ui.text_edit_singleline(&mut contact.t);
        });
    }
    ui.label("description");
    ui.text_edit_multiline(&mut app.user.edit.d);
    if ui.button("save").clicked() {
        save(app, ui);
    }
}
