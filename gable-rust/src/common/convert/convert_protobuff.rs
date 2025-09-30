use crate::{
    common::{generate::proto_field_info::ProtoFieldInfo, setting::BuildSetting, utils},
    gui::datas::{
        esheet_type::ESheetType,
        tree_data::{FieldInfo, TreeData},
    },
};
use serde_json::{Map, Value};
use std::{error::Error, path::PathBuf};

/**
 * 将数据转换为protobuff
 * @param build_setting 构建设置
 * @param tree_data 树数据
*/
pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if tree_data.gable_type == ESheetType::Enum {
        return;
    }
    let value_data: Vec<Map<String, Value>> = tree_data.to_values(&build_setting.keyword);
    if value_data.is_empty() || value_data.len() <= 0 {
        log::warn!("Data is empty");
        return;
    }
    let fields: Vec<FieldInfo> = tree_data.to_fields(&build_setting.keyword);
    let (_, proto_fields, _) = ProtoFieldInfo::transition_fields(&fields, true);
    let target_path: PathBuf = utils::get_absolute_path(&build_setting.target_path)
        .join(format!("{}.bin", tree_data.file_name));
    match tree_data.gable_type {
        ESheetType::Normal | ESheetType::Localize => {
            if let Ok(encoded) = encode_normal_data(&value_data, &proto_fields) {
                if let Err(e) = std::fs::write(&target_path, &encoded) {
                    log::error!("Normal表{}，写入二进制文件失败: {}", tree_data.file_name, e);
                } else {
                    log::info!(
                        "Export [{}] Protobuf binary data successful: {}",
                        build_setting.display_name,
                        target_path.to_str().unwrap()
                    );
                }
            }
        }
        ESheetType::KV => {
            if let Ok(encoded) = encode_kv_data(&value_data[0], &proto_fields) {
                if let Err(e) = std::fs::write(&target_path, &encoded) {
                    log::error!("KV表【{}】写入二进制文件失败: {}", tree_data.file_name, e);
                } else {
                    log::info!(
                        "Export [{}] Protobuf binary data successful: {}",
                        build_setting.display_name,
                        target_path.to_str().unwrap()
                    );
                }
            }
        }
        _ => {}
    }
}

/**
 * 将普通数据表转换为Protobuf二进制数据
 * @param items 数据
 * @param fields 字段数据
 * @return 二进制数据
 */
fn encode_normal_data(
    items: &Vec<Map<String, Value>>,
    fields: &Vec<ProtoFieldInfo>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut table_buffer = Vec::new();
    let items_field_number = 1u32; // UnitTable中items字段的编号 (repeated Unit items = 1)

    // 为每个数据项编码并作为repeated字段的元素添加
    for item in items.iter() {
        let mut item_buffer = Vec::new();

        // 编码Unit消息的所有字段
        for (index, field_info) in fields.iter().enumerate() {
            let field_number: u32 = (index + 1) as u32;
            if let Some(value) = item.get(&field_info.field_name) {
                encode_field_value(field_number, value, &field_info.data_type, &mut item_buffer)?;
            }
        }

        // 将编码后的Unit消息作为repeated字段的一个元素添加到UnitTable中
        // 字段key = (field_number << 3) | wire_type
        let field_key: u32 = (items_field_number << 3) | 2; // wire type 2 = length-delimited
        encode_varint(field_key as u64, &mut table_buffer);
        encode_varint(item_buffer.len() as u64, &mut table_buffer);
        table_buffer.extend_from_slice(&item_buffer);
    }

    Ok(table_buffer)
}

/**
 * 将KV表转换成二进制数据
 * @param item KV表
 * @param field_infos 字段信息
 * @return 二进制数据
 */
fn encode_kv_data(
    item: &Map<String, Value>,
    field_infos: &Vec<ProtoFieldInfo>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut item_buffer = Vec::new();

    // 直接编码KV数据项为单个消息，不使用repeated字段包装
    for field_info in field_infos {
        let field_number: u32 = field_info.field_index as u32;
        if let Some(value) = item.get(&field_info.field_name) {
            encode_field_value(field_number, value, &field_info.data_type, &mut item_buffer)?;
        }
    }

    Ok(item_buffer)
}

/**
 * 将字段转换成二进制数据
 * @param field_number 字段编号
 * @param value 字段值
 * @param data_type 字段类型
 * @param item_buffer 二进制数据缓冲区
 * @return 二进制数据缓冲区
 */
fn encode_field_value(
    field_number: u32,
    value: &Value,
    data_type: &str,
    buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    if data_type.starts_with("repeated ") {
        let inner_type: &str = &data_type[9..]; // 跳过 "repeated " 前缀
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
            "Vector2" | "Vector3" | "Vector4" => {
                // Vector类型作为嵌套消息处理
                if let Some(arr) = value.as_array() {
                    for val in arr {
                        if let Some(s) = val.as_str() {
                            let vector_data: Vec<u8> = encode_vector_data(s, inner_type)?;
                            let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                            encode_varint(key as u64, buffer);
                            encode_varint(vector_data.len() as u64, buffer);
                            buffer.extend_from_slice(&vector_data);
                        } else {
                            let s: String = val.to_string();
                            let vector_data: Vec<u8> = encode_vector_data(&s, inner_type)?;
                            let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                            encode_varint(key as u64, buffer);
                            encode_varint(vector_data.len() as u64, buffer);
                            buffer.extend_from_slice(&vector_data);
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
        match data_type {
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
                if let Some(n) = value.as_i64() {
                    encode_varint(n as u64, buffer);
                } else if let Some(s) = value.as_str() {
                    // 尝试解析字符串为整数
                    if let Ok(num) = s.parse::<i64>() {
                        encode_varint(num as u64, buffer);
                    } else {
                        // 如果无法解析，则默认为0
                        encode_varint(0u64, buffer);
                    }
                } else {
                    // 默认值为0
                    encode_varint(0u64, buffer);
                }
            }
            "string" => {
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
            "Vector2" | "Vector3" | "Vector4" => {
                // Vector类型需要特殊处理，它们是嵌套消息而不是字符串
                if let Some(s) = value.as_str() {
                    // 解析Vector字符串格式 "{x,y}" 或 "{x,y,z}" 或 "{x,y,z,w}"
                    let vector_data: Vec<u8> = encode_vector_data(s, data_type)?;
                    let key: u32 = (field_number << 3) | 2; // wire type 2 for length-delimited
                    encode_varint(key as u64, buffer);
                    encode_varint(vector_data.len() as u64, buffer);
                    buffer.extend_from_slice(&vector_data);
                } else {
                    // 对于Vector类型，如果没有值则写入空消息
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

/**
 * 将vector数据转换成二进制数据
 * @param vector_str vector字符串信息
 * @param vector_type vector类型
 * @return 二进制数据
 */
fn encode_vector_data(vector_str: &str, vector_type: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = Vec::new();

    // 移除大括号并分割值
    let cleaned = vector_str.trim_matches(|c| c == '{' || c == '}');
    let values: Vec<&str> = cleaned.split(',').collect();

    match vector_type {
        "Vector2" => {
            if values.len() >= 2 {
                // x字段编号为1，类型为float (wire type 5 = 32-bit)
                let x_key: u32 = (1 << 3) | 5;
                encode_varint(x_key as u64, &mut buffer);
                let x_val: f32 = values[0].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&x_val.to_le_bytes());

                // y字段编号为2，类型为float (wire type 5 = 32-bit)
                let y_key: u32 = (2 << 3) | 5;
                encode_varint(y_key as u64, &mut buffer);
                let y_val: f32 = values[1].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&y_val.to_le_bytes());
            }
        }
        "Vector3" => {
            if values.len() >= 3 {
                // x字段编号为1，类型为float (wire type 5 = 32-bit)
                let x_key: u32 = (1 << 3) | 5;
                encode_varint(x_key as u64, &mut buffer);
                let x_val: f32 = values[0].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&x_val.to_le_bytes());

                // y字段编号为2，类型为float (wire type 5 = 32-bit)
                let y_key: u32 = (2 << 3) | 5;
                encode_varint(y_key as u64, &mut buffer);
                let y_val: f32 = values[1].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&y_val.to_le_bytes());

                // z字段编号为3，类型为float (wire type 5 = 32-bit)
                let z_key: u32 = (3 << 3) | 5;
                encode_varint(z_key as u64, &mut buffer);
                let z_val: f32 = values[2].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&z_val.to_le_bytes());
            }
        }
        "Vector4" => {
            if values.len() >= 4 {
                // x字段编号为1，类型为float (wire type 5 = 32-bit)
                let x_key: u32 = (1 << 3) | 5;
                encode_varint(x_key as u64, &mut buffer);
                let x_val: f32 = values[0].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&x_val.to_le_bytes());

                // y字段编号为2，类型为float (wire type 5 = 32-bit)
                let y_key: u32 = (2 << 3) | 5;
                encode_varint(y_key as u64, &mut buffer);
                let y_val: f32 = values[1].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&y_val.to_le_bytes());

                // z字段编号为3，类型为float (wire type 5 = 32-bit)
                let z_key: u32 = (3 << 3) | 5;
                encode_varint(z_key as u64, &mut buffer);
                let z_val: f32 = values[2].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&z_val.to_le_bytes());

                // w字段编号为4，类型为float (wire type 5 = 32-bit)
                let w_key: u32 = (4 << 3) | 5;
                encode_varint(w_key as u64, &mut buffer);
                let w_val: f32 = values[3].parse().unwrap_or(0.0);
                buffer.extend_from_slice(&w_val.to_le_bytes());
            }
        }
        _ => {}
    }

    Ok(buffer)
}

/**
 * Protocol Buffers 中的 varint 编码算法
 * @param value 要编码的整数值
 * @param buffer 用于存储编码结果的缓冲区
*/
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
