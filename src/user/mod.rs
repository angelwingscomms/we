use parking_lot::Mutex;
use std::sync::Arc;

use crate::{
    app::View,
    http,
    util::{api, toast},
    App,
};

pub mod views;

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct State {
    search: String,
    #[serde(skip)]
    pub users: Arc<Mutex<Vec<UserSearchRes>>>,
    #[serde(skip)]
    pub user: Arc<Mutex<Current>>,
    pub edit: UserEdit,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct UserSearchRes {
    /// username
    u: String,
    /// similarity
    s: usize,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct User {
    /// tags
    t: Vec<String>,
    /// username
    u: String,
    c: Vec<Contact>,
    /// similarity or score, depending on context
    x: usize,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Current {
    /// i
    i: String,  
    /// tags
    t: Vec<String>,
    /// username
    u: String,
    /// contact
    c: Vec<Contact>,
    /// description
    d: String,
    /// similarity
    x: usize
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct UserEdit {
    /// tags
    t: Vec<String>,
    /// username
    u: String,
    /// contact
    c: Vec<Contact>,
    /// description
    d: String
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Contact {
    pub n: String,
    pub t: String,
}

pub fn search(app: &mut App, ui: &mut egui::Ui) {
    let users = app.user.users.clone();
    let c = ui.ctx().clone();
    let c1 = ui.ctx().clone();
    http::get(
        &api("user/search"),
        move |r| match serde_json::from_str::<Vec<UserSearchRes>>(r) {
            Ok(us) => *users.lock() = us,
            Err(e) => toast(&c, &e.to_string()),
        },
        move |e| toast(&c1, &e.to_string()),
    );
}

pub fn save(app: &mut App, ui: &mut egui::Ui) {
    let l = app.loading.clone();
    let ll = l.clone();
    let ctx = ui.ctx().clone();
    http::post(
        &api("user"),
        app.user.edit.clone(),
        move |_| {
            *l.lock() = true;
        },
        move |e| {
            toast(&ctx, &e.to_string());
        },
    );
    if *ll.lock() {
        app.view = View::User;
    }
}
