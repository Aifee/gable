use crate::common::utils;
use eframe::egui::{
    Button, Color32, CornerRadius, Frame, Label, Response, RichText, Sense, Stroke, Ui, Vec2,
};
use log::Level;

pub fn excel_tap(
    ui: &mut Ui,
    text: &str,
    selected: bool,
    padding: Vec2,
) -> (Response, Option<Response>) {
    let frame: Frame = if selected {
        Frame::NONE
            .fill(utils::get_selected_color(ui.ctx()))
            .stroke(Stroke::new(1.0, utils::get_selected_color(ui.ctx())))
            .corner_radius(CornerRadius {
                nw: 4, // 左上角
                ne: 4, // 右上角
                sw: 0, // 左下角
                se: 0, // 右下角
            })
            .inner_margin(padding)
    } else {
        // 未选中状态：透明背景，灰色边框
        Frame::NONE
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .corner_radius(CornerRadius {
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
            // 创建可交互的文本标签
            label_response = Some(ui.add(Label::new(RichText::new(text)).sense(Sense::click())));
            ui.add_space(8.0);
            // 创建关闭按钮
            let button = Button::new("❌").small().frame(false);
            close_response = Some(ui.add(button));
        })
        .response
    });

    // Frame响应（不包括内部的按钮区域）
    let frame_response = label_response.unwrap();

    (frame_response, close_response)
}

pub fn sheet_tab(ui: &mut Ui, text: &str, selected: bool, padding: Vec2) -> Response {
    // 创建 Frame 样式
    let frame = if selected {
        Frame::NONE
            .fill(utils::get_selected_color(ui.ctx()))
            .stroke(Stroke::new(1.0, utils::get_selected_color(ui.ctx())))
            .corner_radius(CornerRadius {
                nw: 0, // 左上角
                ne: 0, // 右上角
                sw: 4, // 左下角
                se: 4, // 右下角
            })
            .inner_margin(padding)
    } else {
        // 未选中状态：透明背景，灰色边框
        Frame::NONE
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .corner_radius(CornerRadius {
                nw: 0, // 左上角
                ne: 0, // 右上角
                sw: 4, // 左下角
                se: 4, // 右下角
            })
            .inner_margin(padding)
    };
    // 使用 Frame 包装按钮
    let response = frame.show(ui, |ui| ui.label(RichText::new(text))).response;

    // 添加interact方法使Frame可以响应点击事件
    let response = ui.interact(response.rect, response.id, Sense::click());
    response
}

pub fn log_text(ui: &mut Ui, text: &str, level: log::Level) {
    let color = match level {
        Level::Error => Color32::RED,
        Level::Warn => Color32::YELLOW,
        Level::Info => ui.style().visuals.text_color(),
        Level::Debug => Color32::LIGHT_BLUE,
        Level::Trace => Color32::GRAY,
    };
    ui.colored_label(color, text);
}
