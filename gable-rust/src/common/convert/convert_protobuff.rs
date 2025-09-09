use crate::{
    common::{
        generate::generate_protobuff::{self, ProtoFieldInfo},
        setting::BuildSetting,
        utils,
    },
    gui::datas::{
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use serde_json::{Map, Value};
use std::{error::Error, path::PathBuf};

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        return;
    }
    let value_data: Vec<Map<String, Value>> = tree_data.to_values(&build_setting.keyword);
    if value_data.is_empty() || value_data.len() <= 0 {
        log::warn!("数据为空");
        return;
    }
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let (_, proto_fields) = generate_protobuff::transition_fields(&fields);
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.bin", tree_data.content.sheetname));
    match tree_data.gable_type {
        ESheetType::Normal => {
            if let Ok(encoded) = encode_normal_data(&value_data, &proto_fields) {
                if let Err(e) = std::fs::write(&target_path, &encoded) {
                    log::error!(
                        "Normal表{}，写入二进制文件失败: {}",
                        tree_data.content.sheetname,
                        e
                    );
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
            if let Ok(encoded) = encode_kv_data(&value_data[0], &proto_fields) {
                if let Err(e) = std::fs::write(&target_path, &encoded) {
                    log::error!(
                        "KV表【{}】写入二进制文件失败: {}",
                        tree_data.content.sheetname,
                        e
                    );
                } else {
                    log::info!(
                        "导出【{}】Protobuf二进制数据成功:{}",
                        build_setting.display_name,
                        target_path.to_str().unwrap()
                    );
                }
            }
        }
        _ => {}
    }
}

fn encode_normal_data(
    items: &Vec<Map<String, Value>>,
    fields: &Vec<ProtoFieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // 创建一个字段号为1的repeated字段，包含所有数据项
    // 这对应于我们模板中的 {{CLASS_NAME}}Array 消息
    let array_field_number = 1u32;

    for item in items.iter() {
        let mut item_data: Vec<u8> = Vec::new();
        for (index, field_info) in fields.iter().enumerate() {
            let field_number: u32 = (index + 1) as u32;
            if let Some(value) = item.get(&field_info.field_name) {
                encode_field_value(field_number, value, &field_info.field_type, &mut item_data)?;
            }
        }
        // 作为长度分隔的嵌套消息写入到repeated字段中
        let key: u32 = (array_field_number << 3) | 2; // wire type 2 for length-delimited
        encode_varint(key as u64, &mut buffer);
        encode_varint(item_data.len() as u64, &mut buffer);
        buffer.extend_from_slice(&item_data);
    }
    Ok(buffer)
}

fn encode_kv_data(
    item: &Map<String, Value>,
    field_infos: &Vec<ProtoFieldInfo>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = Vec::new();

    // KV表使用字段索引作为字段号，而不是基于位置的索引
    let mut item_data: Vec<u8> = Vec::new();
    for field_info in field_infos {
        let field_number: u32 = field_info.field_index as u32;
        if let Some(value) = item.get(&field_info.field_name) {
            encode_field_value(field_number, value, &field_info.field_type, &mut item_data)?;
        }
    }

    // KV数据直接写入，而不是包装在repeated字段中
    buffer.extend_from_slice(&item_data);
    Ok(buffer)
}

fn encode_field_value(
    field_number: u32,
    value: &Value,
    field_type: &str,
    buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
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
        match field_type {
            "int32" | "int64" | "time" => {
                if let Some(n) = value.as_i64() {
                    // varint编码
                    let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                    encode_varint(key as u64, buffer);
                    encode_varint(n as u64, buffer);
                }
            }
            "enum" => {
                let key: u32 = (field_number << 3) | 0; // wire type 0 for varint
                encode_varint(key as u64, buffer);
                encode_varint(0u64, buffer);
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
