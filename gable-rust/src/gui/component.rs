use eframe::egui;

pub fn excel_tap(
    ui: &mut egui::Ui,
    text: &str,
    selected: bool,
    padding: egui::Vec2,
) -> (egui::Response, Option<egui::Response>) {
    // 创建 Frame 样式
    let frame = if selected {
        // 选中状态：蓝色背景
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(0, 120, 255))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 80, 200)))
            .corner_radius(egui::CornerRadius {
                nw: 4, // 左上角
                ne: 4, // 右上角
                sw: 0, // 左下角
                se: 0, // 右下角
            })
            .inner_margin(padding)
    } else {
        // 未选中状态：透明背景，灰色边框
        egui::Frame::NONE
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .corner_radius(egui::CornerRadius {
                nw: 4, // 左上角
                ne: 4, // 右上角
                sw: 0, // 左下角
                se: 0, // 右下角
            })
            .inner_margin(padding)
    };

    let mut close_response = None;
    let mut label_response = None;

    // 使用 Frame 包装按钮
    let _ = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // 设置文字颜色
            let text_color = if selected {
                egui::Color32::WHITE
            } else {
                ui.style().visuals.text_color()
            };

            // 创建可交互的文本标签
            label_response = Some(
                ui.add(
                    egui::Label::new(egui::RichText::new(text).color(text_color))
                        .sense(egui::Sense::click()),
                ),
            );
            ui.add_space(8.0);
            // 创建关闭按钮
            let button = egui::Button::new("❌").small().frame(false);
            close_response = Some(ui.add(button));
        })
        .response
    });

    // Frame响应（不包括内部的按钮区域）
    let frame_response = label_response.unwrap();

    (frame_response, close_response)
}

pub fn sheet_tab(
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
            .corner_radius(egui::CornerRadius {
                nw: 0, // 左上角
                ne: 0, // 右上角
                sw: 4, // 左下角
                se: 4, // 右下角
            })
            .inner_margin(padding)
    } else {
        // 未选中状态：透明背景，灰色边框
        egui::Frame::NONE
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .corner_radius(egui::CornerRadius {
                nw: 0, // 左上角
                ne: 0, // 右上角
                sw: 4, // 左下角
                se: 4, // 右下角
            })
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
