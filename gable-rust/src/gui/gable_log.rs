use crate::gui::{component, datas::log::LogTrace};
use eframe::egui::{Context, Frame, ScrollArea, TopBottomPanel};
use egui_extras::{Column, TableBuilder};

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
            target_col_width: 300.0,
        }
    }
    pub fn ongui(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("my_log_panel")
            .resizable(true)
            .default_height(300.0)
            .frame(Frame::side_top_panel(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .column(Column::initial(self.time_col_width).resizable(true)) // 时间列
                        .column(Column::initial(self.level_col_width).resizable(true)) // 级别列
                        .column(Column::initial(self.target_col_width).resizable(true)) // 目标列
                        .column(Column::remainder().resizable(true)) // 消息列
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
                                    let row_count = records.len();
                                    if row_count > 0 {
                                        body.rows(self.row_height, row_count, |mut row| {
                                            let row_index = row_count - row.index() - 1;
                                            if let Some(record) = records.get(row_index) {
                                                row.col(|ui| {
                                                    component::log_text(
                                                        ui,
                                                        &record.timestamp,
                                                        record.level,
                                                    );
                                                });
                                                row.col(|ui| {
                                                    component::log_text(
                                                        ui,
                                                        &format!("{:?}", record.level),
                                                        record.level,
                                                    );
                                                });
                                                row.col(|ui| {
                                                    component::log_text(
                                                        ui,
                                                        &record.target,
                                                        record.level,
                                                    );
                                                });
                                                row.col(|ui| {
                                                    component::log_text(
                                                        ui,
                                                        &record.args,
                                                        record.level,
                                                    );
                                                });
                                            }
                                        });
                                    } else {
                                        body.row(self.row_height, |mut row| {
                                            row.col(|ui| {
                                                ui.label("暂无日志");
                                            });
                                            row.col(|ui| {
                                                ui.label("");
                                            });
                                            row.col(|ui| {
                                                ui.label("");
                                            });
                                            row.col(|ui| {
                                                ui.label("");
                                            });
                                        });
                                    }
                                }
                            } else {
                                body.row(self.row_height, |mut row| {
                                    row.col(|ui| {
                                        ui.label("日志系统未初始化");
                                    });
                                    row.col(|ui| {
                                        ui.label("");
                                    });
                                    row.col(|ui| {
                                        ui.label("");
                                    });
                                    row.col(|ui| {
                                        ui.label("");
                                    });
                                });
                            }
                        });
                });
            });
    }
}
