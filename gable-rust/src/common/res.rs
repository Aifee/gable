use eframe::egui;

/// 字体资源
pub const FONT_ASSETS: &[u8] =
    include_bytes!(r"../../assets/fonts/NotoSansSC-VariableFont_wght.ttf");

pub const ICON_DEFUALT: &[u8] = include_bytes!(r"../../assets/icons/error.png");

pub const ICON_SETTING: &[u8] = include_bytes!("../../assets/icons/setting.png");
pub const ICON_SEARCH: &[u8] = include_bytes!("../../assets/icons/search.png");
pub const ICON_SYSTEM: &[u8] = include_bytes!("../../assets/icons/system.png");

pub const ICON_CLIENT: &[u8] = include_bytes!("../../assets/icons/client.png");
pub const ICON_SERVER: &[u8] = include_bytes!("../../assets/icons/server.png");

pub const ICON_FOLDER_BLUE: &[u8] = include_bytes!("../../assets/icons/folder_blue.png");
pub const ICON_FOLDER_NORMAL: &[u8] = include_bytes!("../../assets/icons/folder_normal.png");
pub const ICON_EXCEL: &[u8] = include_bytes!("../../assets/icons/excel.png");
pub const ICON_SHEET: &[u8] = include_bytes!("../../assets/icons/sheet.png");

pub const ICON_C: &[u8] = include_bytes!("../../assets/icons/c.png");
pub const ICON_CANGJIE: &[u8] = include_bytes!("../../assets/icons/cangjie.png");
pub const ICON_CSHARP: &[u8] = include_bytes!("../../assets/icons/csharp.png");
pub const ICON_GOLANG: &[u8] = include_bytes!("../../assets/icons/golang.png");
pub const ICON_JAVA: &[u8] = include_bytes!("../../assets/icons/java.png");
pub const ICON_JAVASCRIPT: &[u8] = include_bytes!("../../assets/icons/javascript.png");
pub const ICON_LUA: &[u8] = include_bytes!("../../assets/icons/lua.png");
pub const ICON_PYTHON: &[u8] = include_bytes!("../../assets/icons/python.png");
pub const ICON_TYPESCRIPT: &[u8] = include_bytes!("../../assets/icons/typescript.png");
pub const ICON_YAML: &[u8] = include_bytes!("../../assets/icons/yaml.png");

/// 加载图片
// 修改 load_texture 支持从字节数据加载
pub fn load_texture(ctx: &egui::Context, data: &[u8], name: &str) -> egui::TextureHandle {
    let img = image::load_from_memory(data).expect("无法从内存加载图像");
    let size = [img.width() as usize, img.height() as usize];
    let image_buffer = img.to_rgba8();
    let pixels: Vec<egui::Color32> = image_buffer
        .pixels()
        .map(|p| egui::Color32::from_rgb(p[0], p[1], p[2]))
        .collect();
    let color_image = egui::ColorImage {
        size,
        pixels,
        source_size: egui::Vec2::new(size[0] as f32, size[1] as f32),
    };
    ctx.load_texture(name, color_image, egui::TextureOptions::default())
}
