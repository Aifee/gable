use crate::{
    common::constant,
    gui::datas::{edata_type::EDataType, gables, tree_data::FieldInfo},
};

/**
 * proto字段信息
*/
#[derive(serde::Serialize)]
pub struct ProtoFieldInfo {
    // 是否是主键
    pub is_key: bool,
    // 字段名称
    pub field_name: String,
    // 字段类型
    pub field_type: String,
    // 字段描述
    pub field_desc: String,
    // 字段序号
    pub field_index: i32,
    // 扩展信息：枚举需要默认值
    pub field_extend: String,
    // 数据类型
    pub data_type: String,
}

impl ProtoFieldInfo {
    /**
     * 通用字段转换成Protobuff字段
     * @param fields 通用字段
     * @param isproto2 是否是版本2
     * @return Protobuff字段
     */
    pub fn transition_fields(
        fields: &Vec<FieldInfo>,
        isproto2: bool,
    ) -> (Vec<String>, Vec<ProtoFieldInfo>, Vec<&EDataType>) {
        let mut common_proto: Vec<&EDataType> = Vec::new();
        let mut imports: Vec<String> = Vec::new();
        let mut proto_fields: Vec<ProtoFieldInfo> = Vec::new();
        for field in fields {
            let mut field_extend: String = String::new();
            let proto_type = match field.field_type {
                EDataType::Int | EDataType::Time => "int32",
                EDataType::Date | EDataType::Long => "int64",
                EDataType::Unknown | EDataType::String | EDataType::Loc => "string",
                EDataType::Boolean => "bool",
                EDataType::Float
                | EDataType::Percentage
                | EDataType::Permillage
                | EDataType::Permian => "float",
                EDataType::Vector2 => {
                    if !common_proto.contains(&&field.field_type) {
                        common_proto.push(&field.field_type);
                    }
                    "Vector2"
                }
                EDataType::Vector3 => {
                    if !common_proto.contains(&&field.field_type) {
                        common_proto.push(&field.field_type);
                    }
                    "Vector3"
                }
                EDataType::Vector4 => {
                    if !common_proto.contains(&&field.field_type) {
                        common_proto.push(&field.field_type);
                    }
                    "Vector4"
                }
                EDataType::IntArr => "repeated int32",
                EDataType::LongArr => "repeated int64",
                EDataType::StringArr => "repeated string",
                EDataType::BooleanArr => "repeated bool",
                EDataType::FloatArr => "repeated float",
                EDataType::Vector2Arr => {
                    if !common_proto.contains(&&EDataType::Vector2) {
                        common_proto.push(&EDataType::Vector2);
                    }
                    "repeated Vector2"
                }
                EDataType::Vector3Arr => {
                    if !common_proto.contains(&&EDataType::Vector3) {
                        common_proto.push(&EDataType::Vector3);
                    }
                    "repeated Vector3"
                }
                EDataType::Vector4Arr => {
                    if !common_proto.contains(&&EDataType::Vector4) {
                        common_proto.push(&EDataType::Vector4);
                    }
                    "repeated Vector4"
                }
                EDataType::Enum => {
                    let mut enum_name = "int32";
                    if !field.field_link.is_empty() {
                        if isproto2 {
                            gables::get_enum_cells(&field.field_link, |enum_datas| {
                                for r_d in enum_datas.cells.iter() {
                                    if let Some(r_c) = r_d.get(constant::TABLE_ENUM_COL_FIELD) {
                                        if !r_c.value.is_empty() {
                                            field_extend = format!(" [default = {}]", r_c.value);
                                            break;
                                        }
                                    }
                                }
                            });
                        }
                        if let Some(pos) = field.field_link.find("@") {
                            enum_name = &field.field_link[pos + 1..];
                        } else {
                            enum_name = &field.field_link;
                        };

                        imports.push(enum_name.to_string());
                    }
                    enum_name
                }
            };
            let data_type = if field.field_type == EDataType::Enum {
                "enum"
            } else {
                proto_type
            };
            let proto_field: ProtoFieldInfo = ProtoFieldInfo {
                is_key: field.is_key,
                field_name: field.field_name.clone(),
                field_type: proto_type.to_string(),
                field_desc: field.field_desc.clone(),
                field_index: field.field_index,
                field_extend: field_extend,
                data_type: data_type.to_string(),
            };
            proto_fields.push(proto_field);
        }
        if common_proto.len() > 0 {
            for common_type in common_proto.iter() {
                match common_type {
                    EDataType::Vector2 => {
                        imports.push("Vector2".to_string());
                    }
                    EDataType::Vector3 => {
                        imports.push("Vector3".to_string());
                    }
                    EDataType::Vector4 => {
                        imports.push("Vector4".to_string());
                    }
                    _ => {}
                }
            }
        }
        return (imports, proto_fields, common_proto);
    }
}
