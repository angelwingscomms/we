use crate::App;



impl App {
    pub fn edit(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.n);
        let mut tags = self.tags.clone();
        for (i, tag) in tags.iter_mut().enumerate() {
            ui.text_edit_singleline(tag);
            if ui.button("X").clicked() {
                self.tags.remove(i);
            }
        }
        for mut contact in self.contact.clone() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut contact.n);
                ui.text_edit_singleline(&mut contact.l);
            });
        }
        if ui.button("save").clicked() {
            self.save();
        }
    }
}