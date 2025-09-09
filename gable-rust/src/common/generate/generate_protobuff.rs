use crate::{
    common::{setting::BuildSetting, utils},
    gui::datas::tree_data::TreeData,
};
use std::{path::PathBuf, process::Command};

pub fn to(build_setting: &BuildSetting, tree_data: &TreeData) {
    if build_setting.postprocessing.is_empty() {
        return;
    }
    // let target_path: PathBuf = utils::get_absolute_path(&build_setting.proto_target_path);

    // #[cfg(target_os = "windows")]
    // {
    //     if let Err(e) = Command::new("cmd")
    //         .current_dir(&target_path)
    //         .args(&["/C", &build_setting.postprocessing])
    //         .spawn()
    //     {
    //         log::error!("无法执行后处理命令: {}", e);
    //     }
    // }

    // #[cfg(target_os = "macos")]
    // {
    //     if let Err(e) = Command::new("sh")
    //         .current_dir(&target_path)
    //         .arg("-c")
    //         .arg(&build_setting.postprocessing)
    //         .spawn()
    //     {
    //         log::error!("无法执行后处理命令: {}", e);
    //     }
    // }

    // #[cfg(target_os = "linux")]
    // {
    //     if let Err(e) = Command::new("sh")
    //         .current_dir(&target_path)
    //         .arg("-c")
    //         .arg(&build_setting.postprocessing)
    //         .spawn()
    //     {
    //         log::error!("无法执行后处理命令: {}", e);
    //     }
    // }
}
