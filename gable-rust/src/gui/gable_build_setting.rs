use crate::{
    common::{res, utils},
    gui::datas::edevelop_type::EDevelopType,
};
use eframe::egui::{
    Align, Align2, Button, CentralPanel, Color32, ComboBox, Context, FontId, Frame, Image, Label,
    Layout, Rect, Response, RichText, ScrollArea, Sense, Separator, SidePanel, TextureHandle,
    TopBottomPanel, Ui, Vec2, Window,
};

pub struct GableBuildSetting {
    pub visible: bool,
    pub add_selected: EDevelopType,
    pub dev_list: Vec<EDevelopType>,
    pub selected_index: usize,
}
impl GableBuildSetting {
    pub fn new() -> Self {
        Self {
            visible: true,
            add_selected: EDevelopType::cpp,
            dev_list: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn ongui(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        let mut visible = self.visible;

        let window = Window::new("构建设置")
            .resizable(true)
            .collapsible(false)
            .default_width(960.0)
            .default_height(600.0)
            .vscroll(false)
            .open(&mut visible);
        window.show(ctx, |ui| {
            SidePanel::left("left_panel")
                .resizable(true)
                .default_width(300.0)
                .width_range(300.0..=700.0)
                .show_inside(ui, |ui| {
                    self.ongui_left_panel(ui);
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
        self.visible = visible;
    }

    fn ongui_left_panel(&mut self, ui: &mut Ui) {
        ui.heading("开发环境");
        let available_height = ui.available_height();
        let combo_area_height = 40.0;

        ScrollArea::vertical()
            .auto_shrink(false)
            .max_height(available_height - combo_area_height)
            .show(ui, |ui| {
                for (index, v) in self.dev_list.iter().enumerate() {
                    let texture: TextureHandle = res::load_develop_icon(ui.ctx(), v);
                    let image: Image<'_> = Image::new(&texture)
                        .tint(Color32::WHITE)
                        .fit_to_exact_size(Vec2::new(24.0, 24.0));
                    let button_size: Vec2 = Vec2::new(ui.available_width(), 40.0);
                    ui.horizontal(|ui| {
                        let tab_button = Button::new("")
                            .fill(if self.selected_index == index {
                                utils::get_selected_color(ui.ctx())
                            } else {
                                Color32::TRANSPARENT
                            })
                            .min_size(Vec2::new(button_size.x - 20.0, button_size.y));

                        let response: Response = ui.add_sized(button_size, tab_button);
                        if response.clicked() {
                            self.selected_index = index;
                        }

                        let rect: Rect = response.rect;
                        ui.put(rect, |ui: &mut Ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.add(image);
                                ui.add_space(8.0);
                                ui.painter().text(
                                    ui.available_rect_before_wrap().left_top()
                                        + Vec2::new(0.0, 18.0),
                                    Align2::LEFT_CENTER,
                                    v.to_string(),
                                    FontId::default(),
                                    ui.style().visuals.text_color(),
                                );
                            });
                            ui.allocate_rect(ui.available_rect_before_wrap(), Sense::hover())
                        });
                    });
                }
            });

        ui.separator();
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ComboBox::from_id_salt("develop_type")
                    .selected_text(format!("{:?}", self.add_selected))
                    .show_ui(ui, |ui| {
                        for item in EDevelopType::iter() {
                            ui.selectable_value(&mut self.add_selected, *item, item.to_string());
                        }
                    });
                ui.add_space(5.0);
                if ui.add_sized([120.0, 26.0], Button::new("添加")).clicked() {
                    self.dev_list.push(self.add_selected.clone());
                }
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
