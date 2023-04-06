use eframe::egui;
use font_kit::source::SystemSource;

pub fn load_fonts(ctx: &egui::Context) {
    let font_config = SystemSource::new();
    for font_family in ["Microsoft YaHei UI", "Microsoft YaHei"] {
        if let Ok(handle) = font_config.select_family_by_name(font_family) {
            let font_name = font_family.to_string();
            let font_data = match &handle.fonts()[1] {
                font_kit::handle::Handle::Path { path, .. } => std::fs::read(path).unwrap(),
                font_kit::handle::Handle::Memory { bytes, .. } => bytes.to_vec(),
            };
            let mut font_defs = egui::FontDefinitions::default();
            font_defs
                .font_data
                .insert(font_name.clone(), egui::FontData::from_owned(font_data));
            font_defs
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, font_name);
            ctx.set_fonts(font_defs);
            break;
        }
    }
}
