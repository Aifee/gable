use crate::gui::datas::log::LogTrace;
use eframe::egui;

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
            .default_height(300.0)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    egui_extras::TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .column(egui_extras::Column::exact(self.time_col_width)) // 时间列
                        .column(egui_extras::Column::exact(self.level_col_width)) // 级别列
                        .column(egui_extras::Column::exact(self.target_col_width)) // 目标列
                        .column(egui_extras::Column::remainder()) // 消息列
                        .header(self.row_height, |mut header| {
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("时间");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("级别");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("目标");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("消息");
                                });
                            });
                        })
                        .body(|mut body| {
                            if let Some(log_records) = LogTrace::get_log_records() {
                                if let Ok(records) = log_records.lock() {
                                    for record in records.iter().rev() {
                                        body.row(self.row_height, |mut row| {
                                            row.col(|ui| {
                                                ui.label(&record.timestamp);
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{:?}", record.level));
                                            });
                                            row.col(|ui| {
                                                ui.label(&record.target);
                                            });
                                            row.col(|ui| {
                                                ui.label(&record.args);
                                            });
                                        });
                                    }
                                }
                            } else {
                                body.row(self.row_height, |mut row| {
                                    row.col(|ui| {
                                        ui.label("日志系统未初始化");
                                    });
                                });
                            }
                        });
                });
            });
    }
}
