use crate::gui::datas::edevelop_type::EDevelopType;
use eframe::egui::{
    Align, Button, CentralPanel, ComboBox, Context, Frame, Layout, RichText, ScrollArea, Separator,
    SidePanel, TopBottomPanel, Ui, Vec2, Window,
};
use serde::de::value;

pub struct GableBuildSetting {
    pub visible: bool,
    pub add_selected: EDevelopType,
    pub dev_list: Vec<EDevelopType>,
}
impl GableBuildSetting {
    pub fn new() -> Self {
        Self {
            visible: true,
            add_selected: EDevelopType::cpp,
            dev_list: Vec::new(),
        }
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn ongui(&mut self, ctx: &Context) {
        let Self {
            visible,
            add_selected,
            dev_list,
        } = self;

        if !*visible {
            return;
        }

        let window = Window::new("构建设置")
            .resizable(true)
            .collapsible(false)
            .default_width(960.0)
            .default_height(600.0)
            .vscroll(false)
            .open(&mut self.visible);
        window.show(ctx, |ui| {
            SidePanel::left("left_panel")
                .resizable(true)
                .default_width(300.0)
                .width_range(300.0..=700.0)
                .show_inside(ui, |ui| {
                    ui.heading("开发环境");

                    // 使用垂直布局将界面分为两部分
                    ui.vertical(|ui| {
                        let available_height = ui.available_height();
                        let combo_area_height = 60.0;

                        ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(available_height - combo_area_height)
                            .show(ui, |ui| {
                                for v in dev_list.iter() {
                                    ui.label(v.to_string());
                                    ui.end_row();
                                }
                            });

                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.allocate_ui_with_layout(
                                Vec2::new(ui.available_width(), combo_area_height - 10.0),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ComboBox::from_id_salt("develop_type")
                                        .selected_text(format!("{add_selected:?}"))
                                        .show_ui(ui, |ui| {
                                            for item in EDevelopType::iter() {
                                                ui.selectable_value(
                                                    add_selected,
                                                    *item,
                                                    item.to_string(),
                                                );
                                            }
                                        });
                                    if ui.add_sized([120.0, 26.0], Button::new("添加")).clicked()
                                    {
                                        dev_list.push(add_selected.clone());
                                    }
                                },
                            );
                        });
                    });
                });

            TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(0.0)
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            if ui.add_sized([165.0, 26.0], Button::new("构建")).clicked() {}
                            if ui
                                .add_sized([165.0, 26.0], Button::new("全部构建"))
                                .clicked()
                            {}
                        });
                    });
                });

            CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Central Panel");
                });
                ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    Self::lorem_ipsum(ui);
                });
            });
        });
    }

    fn lorem_ipsum(ui: &mut Ui) {
        ui.with_layout(
            Layout::top_down(Align::LEFT).with_cross_justify(true),
            |ui| {
                ui.label(RichText::new("Lorem ipsum dolor sit amet").small().weak());
                ui.add(Separator::default().grow(8.0));
                ui.label(RichText::new("Lorem ipsum dolor sit amet").small().weak());
            },
        );
    }
}
