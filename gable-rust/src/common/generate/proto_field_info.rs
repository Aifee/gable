use crate::{
    common::{
        constant,
        generate::generate::{GenerateFieldInfo, GenerateFieldItem, GenerateMainFieldItem},
    },
    gui::datas::{edata_type::EDataType, gables, tree_data::FieldInfo},
};

/**
 * 通用字段转换成Protobuff字段
 * @param fields 通用字段
 * @param isproto2 是否是版本2
 * @return Protobuff字段
 */
pub fn transition_fields(
    info: &FieldInfo,
    isproto2: bool,
) -> (Vec<String>, GenerateFieldInfo, Vec<&EDataType>) {
    let mut common_proto: Vec<&EDataType> = Vec::new();
    let mut imports: Vec<String> = Vec::new();
    let mut main_fields: Vec<GenerateMainFieldItem> = Vec::new();
    for field in info.main_fields.iter() {
        let field_type = match field.field_type {
            EDataType::Int => "int32",
            EDataType::Long => "int64",
            EDataType::Float => "float",
            _ => "string",
        };
        let main_field: GenerateMainFieldItem = GenerateMainFieldItem {
            field_type: field_type.to_string(),
            field_name: field.field_name.clone(),
        };
        main_fields.push(main_field);
    }

    let mut fields: Vec<GenerateFieldItem> = Vec::new();
    for field in info.fields.iter() {
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
        let proto_field: GenerateFieldItem = GenerateFieldItem {
            field_name: field.field_name.clone(),
            field_type: proto_type.to_string(),
            field_desc: field.field_desc.clone(),
            field_index: field.field_index,
            field_extend: field_extend,
            data_type: data_type.to_string(),
        };
        fields.push(proto_field);
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
    let generate_info = GenerateFieldInfo {
        primary_num: main_fields.len(),
        main_fields,
        fields,
    };
    return (imports, generate_info, common_proto);
}
