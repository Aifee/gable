use crate::{
    common::{
        res,
        setting::{self, BuildSetting},
        utils,
    },
    gui::{
        datas::{edevelop_type::EDevelopType, etarget_type::ETargetType},
        gable_app::GableApp,
    },
};
use eframe::egui::{
    Align, Align2, Button, CentralPanel, Checkbox, Color32, ComboBox, Context, FontId, Image,
    Label, Layout, Rect, Response, ScrollArea, Sense, SidePanel, TextEdit, TextureHandle,
    TopBottomPanel, Ui, Vec2, Window,
};
use std::path::PathBuf;

pub struct GableBuildSetting {
    pub visible: bool,
    pub add_selected: EDevelopType,
    pub selected_index: usize,
}
impl GableBuildSetting {
    pub fn new() -> Self {
        Self {
            visible: false,
            add_selected: EDevelopType::Cpp,
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
                    self.ongui_bottom_panel(ui);
                });

            CentralPanel::default().show_inside(ui, |ui| {
                ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    self.ongui_settings(ui);
                });
            });
        });
        self.visible = visible;
    }

    fn ongui_left_panel(&mut self, ui: &mut Ui) {
        ui.heading("开发环境");
        let build_settings: Vec<BuildSetting> = setting::clone_build_settings();
        let available_height = ui.available_height();
        let combo_area_height = 40.0;
        ScrollArea::vertical()
            .auto_shrink(false)
            .max_height(available_height - combo_area_height)
            .show(ui, |ui| {
                for (index, v) in build_settings.iter().enumerate() {
                    let texture: TextureHandle = res::load_develop_icon(ui.ctx(), &v.dev);
                    let image: Image<'_> = Image::new(&texture)
                        .tint(Color32::WHITE)
                        .fit_to_exact_size(Vec2::new(36.0, 36.0));
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
                                    &v.display_name,
                                    FontId::proportional(16.0),
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
                    if let Some(index) = setting::add_build_setting(self.add_selected) {
                        self.selected_index = index;
                    }
                }
            });
        });
    }

    fn ongui_bottom_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                if ui.add_sized([165.0, 26.0], Button::new("删除")).clicked() {
                    self.selected_index =
                        if let Some(index) = setting::remove_build_setting(self.selected_index) {
                            index
                        } else {
                            0
                        }
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                if ui
                    .add_sized([165.0, 26.0], Button::new("全部导出"))
                    .clicked()
                {
                    if let Some(build_settings) = setting::get_build_setting(self.selected_index) {
                        GableApp::convert_target_command(build_settings.display_name);
                    }
                }
                if ui.add_sized([165.0, 26.0], Button::new("编译")).clicked() {}
            });
        });
    }
    fn ongui_settings(&mut self, ui: &mut Ui) {
        let available_height: f32 = 22.0;
        let available_width: f32 = ui.available_width() - 15.0;
        let item_size: Vec2 = Vec2::new(available_width, available_height);
        let title_size: Vec2 = Vec2::new(150.0, available_height);
        let third_size: Vec2 = Vec2::new(120.0, available_height);
        let content_width: f32 = if available_width - title_size.x > 10.0 {
            available_width - title_size.x - 8.0
        } else {
            10.0
        };
        let second_width: f32 = if content_width - third_size.x > 10.0 {
            content_width - third_size.x
        } else {
            10.0
        };
        let content_size: Vec2 = Vec2::new(content_width, available_height);
        let second_size: Vec2 = Vec2::new(second_width, available_height);
        let mut build_settings: BuildSetting =
            if let Some(build_settings) = setting::get_build_setting(self.selected_index) {
                build_settings
            } else {
                return;
            };
        let before_settings = build_settings.clone();
        // development
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(item_size);
                ui.add_sized(title_size, Label::new("开发环境:").truncate());
                ui.allocate_ui_with_layout(content_size, Layout::left_to_right(Align::Min), |ui| {
                    ui.label(build_settings.dev.to_string());
                });
            });
        });
        // alias
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(item_size);
                ui.add_sized(title_size, Label::new("标识:").truncate());
                ui.add_sized(
                    content_size,
                    TextEdit::singleline(&mut build_settings.display_name),
                );
            });
        });
        // keyword
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(item_size);
                ui.add_sized(title_size, Label::new("关键字:").truncate());
                ui.add_sized(
                    content_size,
                    TextEdit::singleline(&mut build_settings.keyword),
                );
            });
        });
        // target_type
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(item_size);

                ui.add_sized(title_size, Label::new("导出类型:").truncate());
                ComboBox::from_id_salt("build_settings.target_type")
                    .selected_text(format!("{:?}", build_settings.target_type))
                    .show_ui(ui, |ui| {
                        for item in ETargetType::iter() {
                            ui.selectable_value(
                                &mut build_settings.target_type,
                                *item,
                                item.to_string(),
                            );
                        }
                    });
            });
        });
        if build_settings.target_type == ETargetType::Protobuff {
            // is_proto_2
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(item_size);
                    ui.add_sized(title_size, Label::new("Proto版本:").truncate());
                    ui.allocate_ui_with_layout(
                        content_size,
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            for (text, value) in [("Proto 2", true), ("Proto 3", false)] {
                                ui.radio_value(&mut build_settings.is_proto_2, value, text);
                            }
                        },
                    );
                });
            });
        }

        // target_path
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(item_size);
                ui.add_sized(title_size, Label::new("导出路径:").truncate());
                ui.allocate_ui_with_layout(second_size, Layout::left_to_right(Align::Min), |ui| {
                    let absolute_path: PathBuf =
                        utils::get_absolute_path(&build_settings.target_path);
                    ui.add(Label::new(absolute_path.to_string_lossy().to_string()).truncate());
                });
                if ui.add_sized(third_size, Button::new("浏览")).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("选择目标路径")
                        .pick_folder()
                    {
                        let re_path: PathBuf = utils::get_env_relative_path(&path);
                        build_settings.target_path = re_path;
                    }
                }
            });
        });
        // generate_script
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(item_size);
                ui.add_sized(title_size, Label::new("生成脚本:").truncate());
                ui.allocate_ui_with_layout(content_size, Layout::left_to_right(Align::Min), |ui| {
                    ui.add(Checkbox::new(&mut build_settings.generate_script, ""))
                });
            });
        });
        if build_settings.generate_script {
            if build_settings.target_type == ETargetType::Protobuff {
                // is_proto_custom
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(item_size);
                        ui.add_sized(title_size, Label::new("自定义模板:").truncate());
                        ui.allocate_ui_with_layout(
                            content_size,
                            Layout::left_to_right(Align::Min),
                            |ui| ui.add(Checkbox::new(&mut build_settings.is_proto_custom, "")),
                        );
                    });
                });
                if build_settings.is_proto_custom {
                    // proto_custom_template
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.set_min_size(item_size);
                            ui.add_sized(title_size, Label::new("模板文件:").truncate());
                            ui.allocate_ui_with_layout(
                                second_size,
                                Layout::left_to_right(Align::Min),
                                |ui| {
                                    let absolute_path: PathBuf = utils::get_absolute_path(
                                        &build_settings.proto_custom_template,
                                    );
                                    ui.add(
                                        Label::new(absolute_path.to_string_lossy().to_string())
                                            .truncate(),
                                    );
                                },
                            );
                            if ui.add_sized(third_size, Button::new("Browse")).clicked() {
                                if let Some(path) =
                                    rfd::FileDialog::new().set_title("选择文件").pick_file()
                                {
                                    let re_path: PathBuf = utils::get_env_relative_path(&path);
                                    build_settings.proto_custom_template = re_path;
                                }
                            }
                        });
                    });
                }
                // proto_target_path
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(item_size);
                        ui.add_sized(title_size, Label::new("proto文件路径:").truncate());
                        ui.allocate_ui_with_layout(
                            second_size,
                            Layout::left_to_right(Align::Min),
                            |ui| {
                                let absolute_path: PathBuf =
                                    utils::get_absolute_path(&build_settings.proto_target_path);
                                ui.add(
                                    Label::new(absolute_path.to_string_lossy().to_string())
                                        .truncate(),
                                );
                            },
                        );
                        if ui.add_sized(third_size, Button::new("浏览")).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("选择目标路径")
                                .pick_folder()
                            {
                                let re_path: PathBuf = utils::get_env_relative_path(&path);
                                build_settings.proto_target_path = re_path;
                            }
                        }
                    });
                });

                // postprocessing
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(item_size);
                        ui.add_sized(title_size, Label::new("proto后处理:").truncate());
                        ui.add_sized(
                            content_size,
                            TextEdit::multiline(&mut build_settings.postprocessing),
                        )
                    });
                });
            } else {
                // script_path
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(item_size);
                        ui.add_sized(title_size, Label::new("脚本路径:").truncate());
                        ui.allocate_ui_with_layout(
                            second_size,
                            Layout::left_to_right(Align::Min),
                            |ui| {
                                let absolute_path: PathBuf =
                                    utils::get_absolute_path(&build_settings.script_path);
                                ui.add(
                                    Label::new(absolute_path.to_string_lossy().to_string())
                                        .truncate(),
                                );
                            },
                        );
                        if ui.add_sized(third_size, Button::new("浏览")).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("选择目标路径")
                                .pick_folder()
                            {
                                let re_path: PathBuf = utils::get_env_relative_path(&path);
                                build_settings.script_path = re_path;
                            }
                        }
                    });
                });
            }
        }
        if build_settings != before_settings {
            if let Err(e) = setting::update_build_setting(self.selected_index, build_settings) {
                log::error!("Failed to update build setting: {}", e);
            }
        }
    }
}
