#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct YLT {
    pub b: i8,
    pub c: i8,
    pub v: i8,
}

fn _ylt(app: &mut crate::App, ui: &mut egui::Ui) {
    ui.add(
        egui::Slider::new(&mut app.ylt.b, 0..=66)
            .text("book")
            .drag_value_speed(0.0108),
    );
    ui.add(
        egui::Slider::new(&mut app.ylt.b, 0..=126)
            .text("chapter")
            .drag_value_speed(0.0108),
    );
    ui.add(
        egui::Slider::new(&mut app.ylt.b, 0..=126)
            .text("verse")
            .drag_value_speed(0.0108),
    );
}