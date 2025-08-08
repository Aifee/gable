use std::fmt::format;

use crate::gui::component;
use eframe::egui;
use serde_json::value::Index;
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
    items: Vec<String>,
}

impl TestApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_button: 0,
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
        }
    }

    fn set_selected(&mut self, index: usize) {
        self.selected_button = index;
    }

    fn remove_index(&mut self, index: usize) {
        if index <= self.items.len() - 1 {
            self.items.remove(index);
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Left Panel");
                });
            });

        egui::TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            .min_height(100.0)
            .max_height(200.0)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show_animated(ctx, true, |ui| {
                ui.heading("LeftPanelLeftPanelLeftPanelLeftPanelLeftPanel");
                if ui.button("按钮1").clicked() {}
                if ui.button("按钮2").clicked() {}
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Sheet Tab");
                let button_padding = egui::Vec2::new(8.0, 4.0);
                let button_spacing = 0.0;

                let button_height = 30.0;
                let spacing = 5.0;
                // let available_height = ui.available_height();
                // let scroll_area_height = available_height - button_height - spacing;
                ui.push_id("excel_tab_scroll", |ui| {
                    egui::ScrollArea::horizontal()
                        .auto_shrink(false)
                        .scroll_source(egui::scroll_area::ScrollSource::ALL)
                        .wheel_scroll_multiplier(egui::Vec2::new(1.0, 1.0))
                        // .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .max_height(50.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = button_spacing;
                                let mut clicked_index = None;
                                let mut remove_index = None;
                                for (i, info) in self.items.iter().enumerate() {
                                    let selected = self.selected_button == i;
                                    let (response, close_response) =
                                        component::excel_tap(ui, info, selected, button_padding);
                                    if response.clicked() {
                                        clicked_index = Some(i);
                                    }
                                    if let Some(close_resp) = close_response {
                                        if close_resp.clicked() {
                                            remove_index = Some(i)
                                        }
                                    }
                                }
                                if let Some(index) = clicked_index {
                                    self.set_selected(index);
                                    println!("Clicked Sheet {}", index);
                                }
                                if let Some(index) = remove_index {
                                    self.remove_index(index);
                                }
                            });
                        });
                });
                ui.push_id("sheet_tab_scroll", |ui| {
                    egui::ScrollArea::horizontal()
                        .auto_shrink(false)
                        .scroll_source(egui::scroll_area::ScrollSource::ALL)
                        .wheel_scroll_multiplier(egui::Vec2::new(1.0, 1.0))
                        // .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .max_height(50.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = button_spacing;
                                let mut clicked_index = None;
                                for (i, info) in self.items.iter().enumerate() {
                                    let selected = self.selected_button == i;
                                    let response =
                                        component::sheet_tab(ui, info, selected, button_padding);
                                    if response.clicked() {
                                        clicked_index = Some(i);
                                    }
                                }
                                if let Some(index) = clicked_index {
                                    // self.set_selected(index);
                                    println!("Clicked Sheet {}", index);
                                }
                            });
                        });
                });
                ui.add_space(spacing);
                if ui
                    .add_sized(
                        [ui.available_width(), button_height],
                        egui::Button::new("++++++"),
                    )
                    .clicked()
                {
                    let info = format!("item index {}", &self.items.len());
                    self.items.push(info);
                }
            });
        });
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update(ctx, frame);
    }
}
