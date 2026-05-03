use eframe::egui;

pub fn draw(painter: &egui::Painter, rect: egui::Rect) {
    let grid = egui::Color32::from_rgb(26, 37, 48);
    for i in 0..=20u32 {
        let x = rect.min.x + i as f32 * rect.width()  / 20.0;
        let y = rect.min.y + i as f32 * rect.height() / 20.0;
        painter.line_segment([egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                              egui::Stroke::new(0.5, grid));
        painter.line_segment([egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                              egui::Stroke::new(0.5, grid));
    }
}
