use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::{
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use serde_json::{Map, Value};
use std::{io::Error, path::PathBuf};
use tera::{Context, Tera};

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    let (import_data, proto_data) = tree_data.to_proto_data(&build_setting.keyword);
    let tera_result: Result<Tera, tera::Error> = Tera::new("assets/templates/proto2/*");
    if tera_result.is_err() {
        log::error!("创建Tera模板失败: {}", tera_result.unwrap_err());
        return;
    }
    let tera: Tera = tera_result.unwrap();
    let mut context: Context = Context::new();
    context.insert("CLASS_NAME", &tree_data.content.sheetname);
    context.insert("fields", &proto_data);
    context.insert("imports", &import_data);
    let rendered_result: Result<String, tera::Error> = match tree_data.gable_type {
        ESheetType::Normal => tera.render("template.proto", &context),
        ESheetType::KV => tera.render("template.proto", &context),
        ESheetType::Enum => tera.render("enums.proto", &context),
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
    to_binary(build_setting, tree_data, &proto_data);
}

fn to_binary(build_setting: &BuildSetting, tree_data: &TreeData, field_infos: &Vec<FieldInfo>) {
    match tree_data.gable_type {
        ESheetType::Normal => {
            let json_data: Vec<Map<String, Value>> = tree_data.to_json_data(&build_setting.keyword);
            let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
                .join(format!("{}.bin", tree_data.content.sheetname));

            // 为Normal类型创建一个包含所有数据的数组消息
            if let Ok(encoded) = encode_normal_data_array(&json_data, field_infos) {
                log::debug!(
                    "Normal表 {} 生成的二进制数据长度: {}",
                    tree_data.content.sheetname,
                    encoded.len()
                );
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

            // KV数据也创建一个包装消息，使其与Normal表结构一致
            if !json_data.is_empty() {
                if let Ok(encoded) = encode_kv_data_wrapper(&json_data[0], field_infos) {
                    log::debug!(
                        "KV表 {} 生成的二进制数据长度: {}",
                        tree_data.content.sheetname,
                        encoded.len()
                    );
                    // 为了调试，您可以将二进制数据以十六进制形式记录下来
                    log::debug!(
                        "KV表 {} 二进制数据前几个字节: {:?}",
                        tree_data.content.sheetname,
                        &encoded[..std::cmp::min(20, encoded.len())]
                    );

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
        let item_data: Vec<u8> = encode_normal_data_to_proto(item, field_infos)?;

        // 作为长度分隔的嵌套消息写入到repeated字段中
        let key: u32 = (array_field_number << 3) | 2; // wire type 2 for length-delimited
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

fn encode_kv_data_wrapper(
    item: &Map<String, Value>,
    field_infos: &Vec<FieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // KV表也使用字段号为1的repeated字段包装，但只包含一个元素
    let wrapper_field_number: u32 = 1u32;

    // 编码KV数据
    let item_data: Vec<u8> = encode_kv_data_to_proto(item, field_infos)?;

    // 作为长度分隔的嵌套消息写入到repeated字段中
    let key: u32 = (wrapper_field_number << 3) | 2; // wire type 2 for length-delimited
    encode_varint(key as u64, &mut buffer);
    encode_varint(item_data.len() as u64, &mut buffer);
    buffer.extend_from_slice(&item_data);

    Ok(buffer)
}

fn encode_kv_data_to_proto(
    item: &Map<String, Value>,
    field_infos: &Vec<FieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // 根据字段信息编码数据，按字段索引排序以确保正确顺序
    let mut sorted_fields: Vec<&FieldInfo> = field_infos.iter().collect();
    sorted_fields.sort_by_key(|field| field.field_index);

    for field_info in sorted_fields {
        let field_number: u32 = field_info.field_index as u32;

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
        let inner_type: &str = &field_type[9..]; // 跳过 "repeated " 前缀
        match inner_type {
            "int32" => {
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(n) = val.as_i64() {
                            let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                            encode_varint(key as u64, buffer);
                            encode_varint(n as u64, buffer);
                        }
                    }
                }
            }
            "int64" => {
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(n) = val.as_i64() {
                            let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
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
                            let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
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
                            let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
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
                            let key: u32 = (field_number << 3) | 5; // wire type 5 for 32-bit
                            encode_varint(key as u64, buffer);
                            let float_bytes: [u8; 4] = (n as f32).to_le_bytes();
                            buffer.extend_from_slice(&float_bytes);
                        }
                    }
                }
            }
            _ => {
                // 对于其他 repeated 类型，作为字符串处理
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(s) = val.as_str() {
                            let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                            encode_varint(key as u64, buffer);
                            encode_varint(s.len() as u64, buffer);
                            buffer.extend_from_slice(s.as_bytes());
                        } else {
                            let s: String = val.to_string();
                            let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                            encode_varint(key as u64, buffer);
                            encode_varint(s.len() as u64, buffer);
                            buffer.extend_from_slice(s.as_bytes());
                        }
                    }
                }
            }
        }
    } else {
        // 处理非 repeated 字段类型
        match field_type {
            "int32" | "int64" | "enum" | "time" => {
                if let Some(n) = value.as_i64() {
                    // varint编码
                    let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(n as u64, buffer);
                } else if field_type == "enum" {
                    // 对于枚举类型，如果没有值则写入默认值0
                    let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(0u64, buffer);
                }
            }
            "string" | "vector2" | "vector3" | "vector4" => {
                if let Some(s) = value.as_str() {
                    let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(s.len() as u64, buffer);
                    buffer.extend_from_slice(s.as_bytes());
                } else {
                    // 对于字符串类型，如果没有值则写入空字符串
                    let key = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(0u64, buffer);
                }
            }
            "bool" => {
                if let Some(b) = value.as_bool() {
                    let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(if b { 1 } else { 0 }, buffer);
                } else {
                    // 对于布尔类型，如果没有值则写入false
                    let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(0u64, buffer);
                }
            }
            "float" | "percentage" | "permillage" | "permian" => {
                if let Some(n) = value.as_f64() {
                    let key: u32 = (field_number << 3) | 5; // wire type 5 for 32-bit
                    encode_varint(key as u64, buffer);
                    let float_bytes: [u8; 4] = (n as f32).to_le_bytes();
                    buffer.extend_from_slice(&float_bytes);
                } else {
                    // 对于浮点类型，如果没有值则写入0.0
                    let key: u32 = (field_number << 3) | 5; // wire type 5 for 32-bit
                    encode_varint(key as u64, buffer);
                    let float_bytes: [u8; 4] = 0f32.to_le_bytes();
                    buffer.extend_from_slice(&float_bytes);
                }
            }
            _ => {
                // 默认处理为字符串
                if let Some(s) = value.as_str() {
                    let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(s.len() as u64, buffer);
                    buffer.extend_from_slice(s.as_bytes());
                } else {
                    // 对于未知类型，写入空字符串
                    let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(0u64, buffer);
                }
            }
        }
    }

    Ok(())
}

// Protocol Buffers 中的 varint 编码算法
fn encode_varint(mut value: u64, buffer: &mut Vec<u8>) {
    loop {
        let mut byte: u8 = (value & 0x7F) as u8;
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
