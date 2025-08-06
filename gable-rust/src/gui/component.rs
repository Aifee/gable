use eframe::egui;
pub fn frame_button(
    ui: &mut egui::Ui,
    text: &str,
    selected: bool,
    padding: egui::Vec2,
) -> egui::Response {
    // 创建 Frame 样式
    let frame = if selected {
        // 选中状态：蓝色背景
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(0, 120, 255))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 80, 200)))
            .corner_radius(0.0)
            .inner_margin(padding)
    } else {
        // 未选中状态：透明背景，灰色边框
        egui::Frame::NONE
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .corner_radius(0.0)
            .inner_margin(padding)
    };

    // 使用 Frame 包装按钮
    let response = frame
        .show(ui, |ui| {
            // 设置文字颜色
            let text_color = if selected {
                egui::Color32::WHITE
            } else {
                ui.style().visuals.text_color()
            };

            ui.label(egui::RichText::new(text).color(text_color))
        })
        .response;

    // 添加interact方法使Frame可以响应点击事件
    let response = ui.interact(response.rect, response.id, egui::Sense::click());
    response
}
