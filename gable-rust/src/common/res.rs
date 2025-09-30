use crate::gui::datas::edevelop_type::EDevelopType;
use eframe::egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions, Vec2};
use image::{DynamicImage, ImageBuffer, Rgba};

/**
 * 字体资源
*/
pub const FONT_ASSETS: &[u8] = include_bytes!(r"../../assets/fonts/SourceHanSansSC-Normal.otf");
/**
 * fallback字体
*/
pub const FONT_FALLBACK: &[u8] = include_bytes!(r"../../assets/fonts/fallback_font.ttf");

// pub const ICON_FOLDER_BLUE: &[u8] = include_bytes!("../../assets/icons/folder_blue.png");
// pub const ICON_FOLDER_NORMAL: &[u8] = include_bytes!("../../assets/icons/folder_normal.png");
// pub const ICON_EXCEL: &[u8] = include_bytes!("../../assets/icons/excel.png");
// pub const ICON_SHEET: &[u8] = include_bytes!("../../assets/icons/sheet.png");

pub const ICON_C: &[u8] = include_bytes!("../../assets/icons/c.png");
pub const ICON_CANGJIE: &[u8] = include_bytes!("../../assets/icons/cangjie.png");
pub const ICON_CSHARP: &[u8] = include_bytes!("../../assets/icons/csharp.png");
pub const ICON_GOLANG: &[u8] = include_bytes!("../../assets/icons/golang.png");
pub const ICON_JAVA: &[u8] = include_bytes!("../../assets/icons/java.png");
pub const ICON_JAVASCRIPT: &[u8] = include_bytes!("../../assets/icons/javascript.png");
pub const ICON_LUA: &[u8] = include_bytes!("../../assets/icons/lua.png");
pub const ICON_PYTHON: &[u8] = include_bytes!("../../assets/icons/python.png");
pub const ICON_TYPESCRIPT: &[u8] = include_bytes!("../../assets/icons/typescript.png");
pub const ICON_RUST: &[u8] = include_bytes!("../../assets/icons/rust.png");
// pub const ICON_YAML: &[u8] = include_bytes!("../../assets/icons/yaml.png");

/**
 * 加载图片
*/
pub fn load_texture(ctx: &Context, data: &[u8], name: &str) -> TextureHandle {
    let img: DynamicImage = image::load_from_memory(data).expect("无法从内存加载图像");
    let size: [usize; 2] = [img.width() as usize, img.height() as usize];
    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let pixels: Vec<Color32> = image_buffer
        .pixels()
        .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    let color_image: ColorImage = ColorImage {
        size,
        pixels,
        source_size: Vec2::new(size[0] as f32, size[1] as f32),
    };
    ctx.load_texture(name, color_image, TextureOptions::default())
}

/**
 * 加载开发语言图标
*/
pub fn load_develop_icon(ctx: &Context, dev: &EDevelopType) -> TextureHandle {
    let icon_texture = match dev {
        EDevelopType::Cpp => ICON_C,
        EDevelopType::Cangjie => ICON_CANGJIE,
        EDevelopType::Csharp => ICON_CSHARP,
        EDevelopType::Golang => ICON_GOLANG,
        EDevelopType::Java => ICON_JAVA,
        EDevelopType::JavaScript => ICON_JAVASCRIPT,
        EDevelopType::Lua => ICON_LUA,
        EDevelopType::Python => ICON_PYTHON,
        EDevelopType::TypeScript => ICON_TYPESCRIPT,
        EDevelopType::Rust => ICON_RUST,
    };
    load_texture(ctx, icon_texture, dev.to_string())
}
