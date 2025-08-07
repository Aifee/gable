use crate::gui::component;
use eframe::egui;
// pub(crate) struct TestApp {
//     selected_button: usize,
// }

/// 测试 excel_tap 组件
// impl TestApp {
//     pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         Self { selected_button: 0 }
//     }
//     pub fn set_selected(&mut self, selected: usize) {
//         self.selected_button = selected;
//     }

//     pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Frame Button Example");

//             // 创建一些样式参数
//             let button_padding = egui::Vec2::new(8.0, 4.0);
//             let button_spacing = 0.0;

//             ui.horizontal(|ui| {
//                 ui.spacing_mut().item_spacing.x = button_spacing;

//                 // 使用 for 循环创建 5 个按钮
//                 for i in 0..5 {
//                     let button_text = format!("Button {}", i + 1);
//                     let is_selected = self.selected_button == i;

//                     // 更好的方法是直接处理点击事件，而不是通过闭包传递
//                     let (response, close_response) =
//                         component::excel_tap(ui, &button_text, is_selected, button_padding);

//                     if response.clicked() {
//                         self.set_selected(i);
//                     }
//                     // 处理关闭按钮点击事件
//                     if let Some(close_resp) = close_response {
//                         if close_resp.clicked() {
//                             println!("关闭按钮 {} 被点击", i + 1);
//                             // 在这里添加关闭按钮的处理逻辑
//                         }
//                     }
//                 }
//             });
//             ui.separator();
//             ui.label(format!("当前选中: 按钮 {}", self.selected_button + 1));
//         });
//     }
// }

/// 测试 sheet_tab 组件
// impl TestApp {
//     pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         Self { selected_button: 0 }
//     }

//     fn set_selected(&mut self, button_index: usize) {
//         self.selected_button = button_index;
//     }

//     pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Sheet Tab");

//             let button_padding = egui::Vec2::new(8.0, 4.0);
//             let button_spacing = 0.0;
//             ui.horizontal(|ui| {
//                 ui.spacing_mut().item_spacing.x = button_spacing;
//                 for i in 0..5 {
//                     let response = component::sheet_tab(
//                         ui,
//                         &format!("Sheet {}", i),
//                         self.selected_button == i,
//                         button_padding,
//                     );
//                     if response.clicked() {
//                         self.set_selected(i);
//                         println!("Clicked Sheet {}", i);
//                     }
//                 }
//             });
//         });
//     }
// }
pub(crate) struct TestApp {
    selected_button: usize,
}

impl TestApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { selected_button: 0 }
    }

    fn set_selected(&mut self, button_index: usize) {
        self.selected_button = button_index;
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sheet Tab");

            let button_padding = egui::Vec2::new(8.0, 4.0);
            let button_spacing = 0.0;
            egui::ScrollArea::horizontal()
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource::ALL)
                .wheel_scroll_multiplier(egui::Vec2::new(1.0, 1.0))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = button_spacing;
                        for i in 0..20 {
                            let response = component::sheet_tab(
                                ui,
                                &format!("Sheet {}", i),
                                self.selected_button == i,
                                button_padding,
                            );
                            if response.clicked() {
                                self.set_selected(i);
                                println!("Clicked Sheet {}", i);
                            }
                        }
                    });
                });
        });
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update(ctx, frame);
    }
}
