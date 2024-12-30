use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct YLT {
    pub b: i32,
    pub c: i32,
    pub v: i32,
    pub a: HashMap<i32, Book>,
    pub l: HashMap<i32, BookC>
}

impl Default for YLT {
    fn default() -> Self {
        Self {
            b: 1,
            c: 1,
            v: 1,
            a: serde_json::from_str(&std::fs::read_to_string("ylt.json").unwrap()).unwrap(),
            l: serde_json::from_str(&std::fs::read_to_string("ylt-count.json").unwrap()).unwrap()
        }
    }
}

type ChapterC = HashMap<i32, i32>;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
struct BookC {
    count: i32,
    chapters: ChapterC
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Verse {
    pub b: i8,
    pub c: i8,
    pub v: i8,
    pub t: String
}

type Book = HashMap<i32, Chapter>;
type Chapter = HashMap<i32, String>;

pub fn ylt(app: &mut crate::App, ui: &mut egui::Ui) {
    println!("{} {} {}", app.ylt.b, app.ylt.c, app.ylt.v);
    ui.add(
        egui::Slider::new(&mut app.ylt.b, 1..=66)
            .text("book")
            .drag_value_speed(0.0108),
    );
    ui.add(
        egui::Slider::new(&mut app.ylt.c, 1..=app.ylt.l[&app.ylt.b].count)
            .text("chapter")
            .drag_value_speed(0.0108),
    );
    ui.add(
        egui::Slider::new(&mut app.ylt.v, 1..=app.ylt.l[&app.ylt.b].chapters[&app.ylt.c])
            .text("verse")
            .drag_value_speed(0.0108),
    );
    
    ui.label(&app.ylt.a[&app.ylt.b][&app.ylt.c][&app.ylt.v]);
}
