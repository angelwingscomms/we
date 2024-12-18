use std::sync::Arc;

use parking_lot::Mutex;
// use wallet_adapter::WalletAdapter;

use crate::App;

// modal
//  if users
     // list of users, onclick switch to user, close modal
// username input
// password input
//  login button
//  register button

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct User {
    pub id: String,
    pub token: String,
    pub username: String,
    // referral link
    pub r: String
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Input {
    username: String,
    password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Auth {
    pub user: Option<User>,
    #[serde(skip)]
    pub users: Arc<Mutex<Vec<User>>>,
    pub open: bool,
    pub input: Input,
}

impl App {
    pub fn auth(&mut self, ui: &mut egui::Ui) {
        // match WalletAdapter::init() {
        //     Ok(adapter) => {
        //         for wallet in adapter.wallets() {
        //             ui.label(wallet.name());
        //         }
        //         // adapter.get_wallet("Solana");
        //     }
        //     Err(e) => toast(ui.ctx(), &e.to_string()),
        // }
        if !self.auth.users.lock().is_empty() {
            for user in &*self.auth.users.lock() {
                if ui.label(&user.username).clicked() {
                    self.auth.user = Some(user.clone());
                }
            }
        }
        // ui.label("username");
        // ui.text_edit_singleline(&mut self.auth.input.username);
        // ui.label("password");
        // ui.text_edit_singleline(&mut self.auth.input.password);
        // ui.horizontal(|ui| {
        //     let ctx = ui.ctx().clone();
        //     let v = &self.auth.input;
        //     let users = self.auth.users.clone();
        //     if ui.button("login").clicked() {
        //         http::post(
        //             &util::api("login"),
        //             &v,
        //             move |r| match serde_json::from_str(r) {
        //                 Ok(u) => {
        //                     users.lock().push(u);
        //                 }
        //                 Err(e) => {
        //                     toast(&ctx, &e.to_string());
        //                 }
        //             },
        //             |e| println!("{}", e),
        //         );
        //     }
        //     // if ui.button("register").clicked() {}
        // });
    }
}
