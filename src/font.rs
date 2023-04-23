use eframe::egui;
use font_kit::source::SystemSource;

pub fn load_fonts(ctx: &egui::Context) {
    let sys = SystemSource::new();
    let font_name = format!("Microsoft YaHei UI");
    let font = sys.select_family_by_name(&font_name)
        .unwrap()
        .fonts()[1].load()
        .unwrap()
        .copy_font_data()
        .unwrap().to_vec();
    let mut font_defs = egui::FontDefinitions::default();
    font_defs
        .font_data
        .insert(font_name.to_string(), egui::FontData::from_owned(font));
    font_defs
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, font_name);
    ctx.set_fonts(font_defs);
}
