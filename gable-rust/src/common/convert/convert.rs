use crate::{
    common::setting::BuildSetting,
    gui::datas::{eitem_type::EItemType, tree_item::TreeItem},
};

// 全量构建
// pub fn all(setting: &BuildSetting) {}

pub fn from_items(item: &TreeItem) {
    let datas = item.get_datas();
    if datas.len() <= 0 {
        log::error!("获取数据为空:{}", item.display_name);
        return;
    }
    for (display_name, data) in datas.iter() {
        log::info!("开始构建:{}", display_name);
    }
}

pub fn form_setting(setting: &BuildSetting) {}
