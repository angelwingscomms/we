use dotenv_codegen::dotenv;

pub fn toast(ctx: &egui::Context, text: &str) {
    let mut toasts = egui_toast::Toasts::new()
        .anchor(egui::Align2::LEFT_TOP, (10.0, 10.0))
        .direction(egui::Direction::TopDown);
    toasts.add(egui_toast::Toast {
        text: text.into(),
        kind: egui_toast::ToastKind::Error,
        options: egui_toast::ToastOptions::default()
            .duration_in_seconds(10.0)
            .show_progress(true)
            .show_icon(true),
        style: egui_toast::ToastStyle::default(),
    });
    toasts.show(ctx);
}

pub fn api(path: &str) -> String {
    dotenv!("API").to_string() + "/" + path
}