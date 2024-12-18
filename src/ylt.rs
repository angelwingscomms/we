#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct YLT {
    pub b: i8,
    pub c: i8,
    pub v: i8,
}

fn ylt(app: &mut crate::App, ui: &egui::Ui) {
    // ui.add(
    //     egui::Slider::new(&mut app.ylt.b, 0...=1080.)
    //         .text("book")
    //         .drag_value_speed(0.0108),
    // );
    // ui.add(
    //     egui::Slider::new(&mut app.ylt.b, 0.0..=1080.0)
    //         .text("chapter")
    //         .drag_value_speed(0.0108),
    // );
    // ui.add(
    //     egui::Slider::new(&mut app.ylt.b, 0.0..=1080.0)
    //         .text("verse")
    //         .drag_value_speed(0.0108),
    // );
}