use crate::gui::datas::edevelop_type::EDevelopType;
use eframe::egui::{
    Align, Button, CentralPanel, ComboBox, Context, Frame, Layout, RichText, ScrollArea, Separator,
    SidePanel, TopBottomPanel, Ui, Vec2, Window,
};

pub struct GableBuildSetting {
    pub visible: bool,
    pub add_selected: EDevelopType,
}
impl GableBuildSetting {
    pub fn new() -> Self {
        Self {
            visible: false,
            add_selected: EDevelopType::c,
        }
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn ongui(&mut self, ctx: &Context) {
        let Self {
            visible,
            add_selected,
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
                .width_range(150.0..=700.0)
                .show_inside(ui, |ui| {
                    ui.heading("开发环境");
                    ScrollArea::vertical().show(ui, |ui| {
                        Self::lorem_ipsum(ui);
                    });
                    ComboBox::from_label("Take your pick")
                        .selected_text(format!("{add_selected:?}"))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(add_selected, EDevelopType::c, "C");
                            ui.selectable_value(add_selected, EDevelopType::csharp, "Csharp");
                            ui.selectable_value(add_selected, EDevelopType::cangjie, "Cangjie");
                            ui.selectable_value(add_selected, EDevelopType::go, "Golang");
                            ui.selectable_value(add_selected, EDevelopType::java, "Java");
                            ui.selectable_value(
                                add_selected,
                                EDevelopType::javascript,
                                "Javascript",
                            );
                            ui.selectable_value(add_selected, EDevelopType::lua, "Lua");
                            ui.selectable_value(add_selected, EDevelopType::python, "Python");
                            ui.selectable_value(
                                add_selected,
                                EDevelopType::typescript,
                                "Typescript",
                            );
                        });
                });

            TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(0.0)
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            if ui.add_sized([165.0, 30.0], Button::new("构建")).clicked() {}
                            if ui
                                .add_sized([165.0, 30.0], Button::new("全部构建"))
                                .clicked()
                            {}
                        });
                    });
                });

            CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Central Panel");
                });
                ScrollArea::vertical().show(ui, |ui| {
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
