use crate::gui::datas::log::LogRecord;
use crate::gui::datas::log::LogTrace;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub(crate) struct GableLog {
    row_height: f32,
    time_col_width: f32,
    level_col_width: f32,
    target_col_width: f32,
}

impl GableLog {
    pub fn new() -> Self {
        Self {
            row_height: 20.0,
            time_col_width: 200.0,
            level_col_width: 100.0,
            target_col_width: 100.0,
        }
    }
    pub fn ongui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            // .min_height(100.0)
            // .max_height(200.0)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink(true)
                    .wheel_scroll_multiplier(egui::Vec2::new(1.0, 1.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            egui::Grid::new("m_log_grid").striped(true).show(ui, |ui| {
                                ui.allocate_ui_with_layout(
                                    egui::Vec2::new(self.time_col_width, self.row_height),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.label("时间");
                                    },
                                );
                                ui.allocate_ui_with_layout(
                                    egui::Vec2::new(self.level_col_width, self.row_height),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.label("级别");
                                    },
                                );
                                ui.allocate_ui_with_layout(
                                    egui::Vec2::new(self.target_col_width, self.row_height),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.label("目标");
                                    },
                                );
                                ui.allocate_ui_with_layout(
                                    egui::Vec2::new(ui.available_width(), self.row_height),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.label("消息");
                                    },
                                );
                                ui.end_row();

                                // 从LogTrace获取log_records并显示
                                if let Some(log_records) = LogTrace::get_log_records() {
                                    if let Ok(records) = log_records.lock() {
                                        for record in records.iter() {
                                            ui.allocate_ui_with_layout(
                                                egui::Vec2::new(
                                                    self.time_col_width,
                                                    self.row_height,
                                                ),
                                                egui::Layout::left_to_right(egui::Align::LEFT),
                                                |ui| {
                                                    ui.label(&record.timestamp);
                                                },
                                            );
                                            ui.allocate_ui_with_layout(
                                                egui::Vec2::new(
                                                    self.level_col_width,
                                                    self.row_height,
                                                ),
                                                egui::Layout::left_to_right(egui::Align::LEFT),
                                                |ui| {
                                                    ui.label(format!("{:?}", record.level));
                                                },
                                            );
                                            ui.allocate_ui_with_layout(
                                                egui::Vec2::new(
                                                    self.target_col_width,
                                                    self.row_height,
                                                ),
                                                egui::Layout::left_to_right(egui::Align::LEFT),
                                                |ui| {
                                                    ui.label(&record.target);
                                                },
                                            );
                                            ui.allocate_ui_with_layout(
                                                egui::Vec2::new(
                                                    ui.available_width(),
                                                    self.row_height,
                                                ),
                                                egui::Layout::left_to_right(egui::Align::LEFT),
                                                |ui| {
                                                    ui.label(&record.args);
                                                },
                                            );
                                            ui.end_row();
                                        }
                                    }
                                } else {
                                    ui.label("日志系统未初始化");
                                    ui.end_row();
                                }
                            });
                        })
                    });
            });
    }
}
