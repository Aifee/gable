use crate::{
    common::{
        setting::{self, AppSettings, BuildSetting},
        utils,
    },
    gui::datas::{
        esheet_type::ESheetType,
        etarget_type::ETargetType,
        gables,
        tree_data::{FieldInfo, TreeData},
        tree_item::TreeItem,
    },
};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
    sync::MutexGuard,
};
use tera::{Context, Tera};

pub fn from_target(setting: &BuildSetting) {
    let items: MutexGuard<'_, Vec<TreeItem>> = gables::TREE_ITEMS.lock().unwrap();
    let mut datas: HashMap<String, &TreeData> = HashMap::new();
    for item in items.iter() {
        let item_datas: HashMap<String, &TreeData> = item.get_datas();
        if item_datas.len() > 0 {
            datas.extend(item_datas);
        }
    }
    if datas.len() <= 0 {
        log::error!("未找到要导出的配置");
        return;
    }
    for (_, data) in datas.iter() {
        match setting.target_type {
            ETargetType::Json => {
                if data.gable_type != ESheetType::Enum {
                    to_json(setting, data)
                }
            }
            ETargetType::CSV => {
                if data.gable_type != ESheetType::Enum {
                    to_csv(setting, data)
                }
            }
            ETargetType::Protobuff => to_proto(setting, data),
        }
    }
}

pub fn from_items(item: &TreeItem) {
    let datas: HashMap<String, &TreeData> = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }

    let settings: MutexGuard<'_, AppSettings> = setting::APP_SETTINGS.lock().unwrap();
    for build_setting in settings.build_settings.iter() {
        for (_, data) in datas.iter() {
            match build_setting.target_type {
                ETargetType::Json => {
                    if data.gable_type != ESheetType::Enum {
                        to_json(build_setting, data)
                    }
                }
                ETargetType::CSV => {
                    if data.gable_type != ESheetType::Enum {
                        to_csv(build_setting, data)
                    }
                }
                ETargetType::Protobuff => to_proto(build_setting, data),
            }
        }
    }
}

fn to_json(build_setting: &BuildSetting, tree_data: &TreeData) {
    let target_path = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.json", tree_data.content.sheetname));
    let json_data: Vec<Map<String, Value>> = tree_data.to_json_data(&build_setting.keyword);
    let contents: String = serde_json::to_string_pretty(&json_data).expect("JSON序列化失败");
    let result: Result<(), Error> = std::fs::write(&target_path, contents);
    if result.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "导出【{}】成功:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    }
}

fn to_csv(build_setting: &BuildSetting, tree_data: &TreeData) {
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.csv", tree_data.content.sheetname));
    let csv_data: Vec<Vec<String>> = tree_data.to_csv_data(&build_setting.keyword);
    // 创建CSV文件
    let file: Result<File, Error> = File::create(&target_path);
    if file.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
        return;
    }
    let file = file.unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);
    // 写入CSV数据
    for row_data in csv_data.iter() {
        let mut line: String = String::new();
        let mut is_first: bool = true;
        for col_value in row_data.iter() {
            if !is_first {
                line.push(',');
            }
            // 转义包含逗号或引号的值
            if col_value.contains(',') || col_value.contains('"') || col_value.contains('\n') {
                line.push('"');
                line.push_str(&col_value.replace("\"", "\"\""));
                line.push('"');
            } else {
                line.push_str(col_value);
            }
            is_first = false;
        }

        line.push('\n');
        if let Err(e) = writer.write_all(line.as_bytes()) {
            log::error!("写入【{}】文件时出错:{}", build_setting.display_name, e);
            return;
        }
    }

    if let Err(e) = writer.flush() {
        log::error!("刷新【{}】文件时出错:{}", build_setting.display_name, e);
        return;
    }

    log::info!(
        "导出【{}】成功:{}",
        build_setting.display_name,
        target_path.to_str().unwrap()
    );
}

fn to_proto(build_setting: &BuildSetting, tree_data: &TreeData) {
    let (import_data, proto_data) = tree_data.to_proto_data(&build_setting.keyword);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/proto2/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context = Context::new();
    context.insert("CLASS_NAME", &tree_data.content.sheetname);
    context.insert("fields", &proto_data);
    context.insert("imports", &import_data);
    let rendered_result: Result<String, tera::Error> = if tree_data.gable_type == ESheetType::Enum {
        tera.render("enums.proto", &context)
    } else {
        tera.render("template.proto", &context)
    };
    if rendered_result.is_err() {
        log::error!("渲染模板错误: {}", rendered_result.unwrap_err());
        return;
    }
    let rendered: String = rendered_result.unwrap();

    // 写入文件
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.proto_target_path)
        .join(format!("{}.proto", tree_data.content.sheetname));

    let result: Result<(), Error> = std::fs::write(&target_path, rendered);
    if result.is_err() {
        log::error!(
            "导出【{}】失败:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    } else {
        log::info!(
            "导出【{}】成功:{}",
            build_setting.display_name,
            target_path.to_str().unwrap()
        );
    }
    to_proto_binary(build_setting, tree_data, &proto_data);
}

fn to_proto_binary(
    build_setting: &BuildSetting,
    tree_data: &TreeData,
    field_infos: &Vec<FieldInfo>,
) {
    match tree_data.gable_type {
        ESheetType::Normal => {
            let json_data: Vec<Map<String, Value>> = tree_data.to_json_data(&build_setting.keyword);
            let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
                .join(format!("{}.bin", tree_data.content.sheetname));

            // 为Normal类型创建一个包含所有数据的数组消息
            if let Ok(encoded) = encode_normal_data_array(&json_data, field_infos) {
                if let Err(e) = std::fs::write(&target_path, &encoded) {
                    log::error!("写入二进制文件失败: {}", e);
                } else {
                    log::info!(
                        "导出【{}】Protobuf二进制数据成功:{}",
                        build_setting.display_name,
                        target_path.to_str().unwrap()
                    );
                }
            }
        }
        ESheetType::KV => {
            let json_data: Vec<Map<String, Value>> = tree_data.to_json_data(&build_setting.keyword);
            let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
                .join(format!("{}.bin", tree_data.content.sheetname));

            // KV数据通常是单个对象
            if !json_data.is_empty() {
                if let Ok(encoded) = encode_kv_data_to_proto(&json_data[0], field_infos) {
                    if let Err(e) = std::fs::write(&target_path, &encoded) {
                        log::error!("写入二进制文件失败: {}", e);
                    } else {
                        log::info!(
                            "导出【{}】Protobuf二进制数据成功:{}",
                            build_setting.display_name,
                            target_path.to_str().unwrap()
                        );
                    }
                }
            }
        }
        ESheetType::Enum => {}
    }
}

fn encode_normal_data_array(
    items: &Vec<Map<String, Value>>,
    field_infos: &Vec<FieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // 创建一个字段号为1的repeated字段，包含所有数据项
    // 这对应于我们模板中的 {{CLASS_NAME}}Array 消息
    let array_field_number = 1u32;

    for item in items.iter() {
        // 编码单个数据条目
        let item_data = encode_normal_data_to_proto(item, field_infos)?;

        // 作为长度分隔的嵌套消息写入到repeated字段中
        let key = (array_field_number << 3) | 2; // wire type 2 for length-delimited
        encode_varint(key as u64, &mut buffer);
        encode_varint(item_data.len() as u64, &mut buffer);
        buffer.extend_from_slice(&item_data);
    }

    Ok(buffer)
}

fn encode_normal_data_to_proto(
    item: &Map<String, Value>,
    field_infos: &Vec<FieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // 根据字段信息编码数据
    for (index, field_info) in field_infos.iter().enumerate() {
        let field_number: u32 = (index + 1) as u32;

        // 在JSON数据中查找对应的字段值
        if let Some(value) = item.get(&field_info.field_name) {
            encode_field_value(field_number, value, &field_info.field_type, &mut buffer)?;
        }
    }

    Ok(buffer)
}

fn encode_kv_data_to_proto(
    item: &Map<String, Value>,
    field_infos: &Vec<FieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // 根据字段信息编码数据
    for (index, field_info) in field_infos.iter().enumerate() {
        let field_number = (index + 1) as u32;

        // 在JSON数据中查找对应的字段值
        if let Some(value) = item.get(&field_info.field_name) {
            encode_field_value(field_number, value, &field_info.field_type, &mut buffer)?;
        }
    }

    Ok(buffer)
}

fn encode_field_value(
    field_number: u32,
    value: &Value,
    field_type: &str,
    buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 处理 repeated 字段类型
    if field_type.starts_with("repeated ") {
        let inner_type = &field_type[9..]; // 跳过 "repeated " 前缀
        match inner_type {
            "int32" => {
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(n) = val.as_i64() {
                            let key = (field_number << 3) | 0; // wire type 0 for varint
                            encode_varint(key as u64, buffer);
                            encode_varint(n as u64, buffer);
                        }
                    }
                }
            }
            "string" => {
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(s) = val.as_str() {
                            let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                            encode_varint(key as u64, buffer);
                            encode_varint(s.len() as u64, buffer);
                            buffer.extend_from_slice(s.as_bytes());
                        }
                    }
                }
            }
            "bool" => {
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(b) = val.as_bool() {
                            let key = (field_number << 3) | 0; // wire type 0 for varint
                            encode_varint(key as u64, buffer);
                            encode_varint(if b { 1 } else { 0 }, buffer);
                        }
                    }
                }
            }
            "float" => {
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(n) = val.as_f64() {
                            let key = (field_number << 3) | 5; // wire type 5 for 32-bit
                            encode_varint(key as u64, buffer);
                            let float_bytes = (n as f32).to_le_bytes();
                            buffer.extend_from_slice(&float_bytes);
                        }
                    }
                }
            }
            _ => {
                // 对于其他 repeated 类型，作为字符串处理
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        let s = val.to_string();
                        let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                        encode_varint(key as u64, buffer);
                        encode_varint(s.len() as u64, buffer);
                        buffer.extend_from_slice(s.as_bytes());
                    }
                }
            }
        }
    } else {
        // 处理非 repeated 字段类型
        match field_type {
            "int32" | "int64" | "enum" => {
                if let Some(n) = value.as_i64() {
                    // varint编码
                    let key = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(n as u64, buffer);
                }
            }
            "string" => {
                if let Some(s) = value.as_str() {
                    let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(s.len() as u64, buffer);
                    buffer.extend_from_slice(s.as_bytes());
                } else {
                    // 尝试转换为字符串
                    let s = value.to_string();
                    let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(s.len() as u64, buffer);
                    buffer.extend_from_slice(s.as_bytes());
                }
            }
            "bool" => {
                if let Some(b) = value.as_bool() {
                    let key = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(if b { 1 } else { 0 }, buffer);
                }
            }
            "float" => {
                if let Some(n) = value.as_f64() {
                    let key = (field_number << 3) | 5; // wire type 5 for 32-bit
                    encode_varint(key as u64, buffer);
                    let float_bytes = (n as f32).to_le_bytes();
                    buffer.extend_from_slice(&float_bytes);
                }
            }
            _ => {
                // 默认处理为字符串
                if let Some(s) = value.as_str() {
                    let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(s.len() as u64, buffer);
                    buffer.extend_from_slice(s.as_bytes());
                } else {
                    // 尝试转换为字符串
                    let s = value.to_string();
                    let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(s.len() as u64, buffer);
                    buffer.extend_from_slice(s.as_bytes());
                }
            }
        }
    }

    Ok(())
}

fn encode_varint(mut value: u64, buffer: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buffer.push(byte);
        if value == 0 {
            break;
        }
    }
}
