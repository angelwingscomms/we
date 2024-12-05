use crate::App;

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
pub enum Section {
    #[default]
    N,
    P,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Class {
    r: bool, //received
    p: bool, //prepared
    d: bool, //done
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Primary {
    g1: Class,
    g2: Class,
    g4: Class,
    g5: Class,
}

impl Render for Primary {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("g1");
            self.g1.render(ui);
        });
        ui.horizontal(|ui| {
            ui.label("g2");
            self.g2.render(ui);
        });
        ui.horizontal(|ui| {
            ui.label("g4");
            self.g4.render(ui);
        });
        ui.horizontal(|ui| {
            ui.label("g5");
            self.g5.render(ui);
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Nursery {
    pn: Class,
    n1: Class,
    n2: Class,
}

impl Render for Nursery {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("pn");
            self.pn.render(ui);
        });
        ui.horizontal(|ui| {
            ui.label("n1");
            self.n1.render(ui);
        });
        ui.horizontal(|ui| {
            ui.label("n2");
            self.n2.render(ui);
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum ClassGroup {
    N(Nursery),
    P(Primary),
}

impl Default for ClassGroup {
    fn default() -> Self {
        Self::N(Default::default())
    }
}

impl Render for ClassGroup {
    fn render(&mut self, ui: &mut egui::Ui) {
        match self {
            ClassGroup::P(p) => {
                p.render(ui);
            }
            ClassGroup::N(n) => {
                n.render(ui);
            }
        }
    }
}

impl Class {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.r, "");
            ui.checkbox(&mut self.p, "");
            ui.checkbox(&mut self.d, "");
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Subject {
    n: String,
    classes: ClassGroup,
}

impl Subject {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.label(&self.n);
        self.classes.render(ui);
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Day {
    m: Subject,         //morning
    a: Option<Subject>, //afternoon
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Exams {
    m: M,
    t: T,
    w: W,
}

#[derive(serde::Deserialize, Default, serde::Serialize, Clone)]
#[serde(default)]
pub struct M {
    n_ead: Subject,
    n_psrn: Subject,
    p_nv: Subject,
    p_french: Subject,
}

impl Render for M {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.n_ead.render(ui);
        self.n_psrn.render(ui);
        self.p_nv.render(ui);
        self.p_french.render(ui);
    }
}

#[derive(serde::Deserialize, Default, serde::Serialize, Clone)]
#[serde(default)]
pub struct T {
    n_ict: Subject,
    p_pvs: Subject,
    p_history: Subject,
}

impl Render for T {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.n_ict.render(ui);
        self.p_pvs.render(ui);
        self.p_history.render(ui);
    }
}

#[derive(serde::Deserialize, Default, serde::Serialize, Clone)]
#[serde(default)]
pub struct W {
    n_rhyme: Subject,
    p_phe: Subject,
    p_crs: Subject,
}

impl Render for W {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.n_rhyme.render(ui);
        self.p_phe.render(ui);
        self.p_crs.render(ui);
    }
}

pub trait Render {
    fn render(&mut self, ui: &mut egui::Ui);
}

impl Default for Exams {
    fn default() -> Self {
        Self {
            m: M {
                n_ead: Subject {
                    n: "EAD".into(),
                    classes: ClassGroup::N(Nursery::default()),
                },
                n_psrn: Subject {
                    n: "PSRN".into(),
                    classes: ClassGroup::N(Nursery::default()),
                },
                p_nv: Subject {
                    n: "NV".into(),
                    classes: ClassGroup::P(Primary::default()),
                },
                p_french: Subject {
                    n: "French".into(),
                    classes: ClassGroup::P(Primary::default()),
                },
            },
            t: T {
                n_ict: Subject {
                    n: "ICT".into(),
                    classes: ClassGroup::N(Nursery::default()),
                },
                p_pvs: Subject {
                    n: "PVS".into(),
                    classes: ClassGroup::P(Primary::default()),
                },
                p_history: Subject {
                    n: "History".into(),
                    classes: ClassGroup::P(Primary::default()),
                },
            },
            w: W {
                n_rhyme: Subject {
                    n: "Rhyme".into(),
                    classes: ClassGroup::N(Nursery::default()),
                },
                p_phe: Subject {
                    n: "PHE".into(),
                    classes: ClassGroup::P(Primary::default()),
                },
                p_crs: Subject {
                    n: "CRS".into(),
                    classes: ClassGroup::P(Primary::default()),
                },
            },
        }
    }
}

impl App {
    pub fn exams(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // ui.label("Monday");
            // self.e.m.render(ui);
            ui.add_space(9.0);
            ui.label("Tuesday");
            self.e.t.render(ui);
            ui.add_space(9.0);
            ui.label("Wednesday");
            self.e.w.render(ui);
        });
    }
}
