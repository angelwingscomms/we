pub struct E {
    i: Vec<String>,
    n: String,
    r: i64, // rooms
    f: bool, // for sale
    c: bool, // has cleaners
    p: i64, // price
    rp: Option<String>,
    l: String,
    g: String
}

// pub enum Sort {
//     RealEstate,
//     Product,
//     Service
// }

// pub struct RealEstate {
//     r: i64, // rooms
//     f: bool, // for sale
//     c: bool, // has cleaners
//     p: i64, // price
// }

pub fn e(ui: &mut egui::Ui, e: E) {
    /*
        name
        
    */
    
    ui.label(e.n);
    ui.hyperlink_to(e.l.clone(), e.g.clone());
    for i in e.i {
        ui.label(i);
    }
}

pub fn e_edit(ui: &mut egui::Ui, e: &mut E) {
    ui.text_edit_singleline(&mut e.n);
    
    ui.label("price");
    ui.text_edit_singleline(&mut e.p.to_string());
    
    ui.label("location");
    ui.text_edit_singleline(&mut e.l);
    
    if let Some(mut rp) = e.rp.clone() {
        ui.label("payment period");
        ui.text_edit_singleline(&mut rp);
    }
    
    ui.label("rooms");
    ui.text_edit_singleline(&mut e.r.to_string());
    
    ui.toggle_value(&mut e.f, "For Sale");
    ui.toggle_value(&mut e.c, "Has cleaners");
    
    ui.label("map link");
    ui.text_edit_singleline(&mut e.g);
    
    ui.label("features");
    if ui.button("add feature").clicked() {
        e.i.push(String::new());
    }
    
    
    for i in &mut e.i {
        ui.text_edit_multiline(i);
    }
    
    if ui.button("save").clicked() {
        // todo
    }
}