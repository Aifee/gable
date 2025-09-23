pub fn run_import(cli_args: &[String]) -> Result<(), eframe::Error> {
    // 解析参数
    let mut input_files: Vec<String> = Vec::new();
    let mut target_platform: Option<String> = None;
    let mut generate_script = false;
    let mut i = 0;

    while i < cli_args.len() {
        let arg: &String = &cli_args[i];

        match arg.as_str() {
            "-i" | "--input" => {
                i += 1;
                if i < cli_args.len() {
                    // 收集输入文件，支持多个文件
                    while i < cli_args.len() && !cli_args[i].starts_with('-') {
                        input_files.push(cli_args[i].clone());
                        i += 1;
                    }
                    continue; // 已经增加了i，所以继续循环
                } else {
                    println!("Error: {} 参数需要指定输入文件", arg);
                    _usage();
                    return Ok(());
                }
            }
            "-t" => {
                i += 1;
                if i < cli_args.len() {
                    target_platform = Some(cli_args[i].clone());
                } else {
                    println!("Error: -t 参数需要指定目标平台");
                    _usage();
                    return Ok(());
                }
            }
            "--script" => {
                generate_script = true;
            }
            _ => {
                if !arg.starts_with('-') {
                    // 如果不是以-开头，可能是输入文件
                    input_files.push(arg.clone());
                }
            }
        }
        i += 1;
    }

    // 执行导入操作
    if input_files.is_empty() {
        println!("Error: 导入数据需要指定输入文件");
        _usage();
        return Ok(());
    }

    println!("正在导入数据...");
    println!("输入文件: {:?}", input_files);
    if let Some(platform) = &target_platform {
        println!("目标平台: {}", platform);
    }

    if generate_script {
        println!("正在生成导入脚本...");
        // 这里添加实际的脚本生成逻辑
    }

    // 这里添加实际的数据导入逻辑
    println!("导入完成");

    Ok(())
}

fn _usage() {
    println!(
        r#"Gable CLI 导入工具

用法:
  gable --import [选项]

选项:
  -i, --input <文件列表>  指定输入文件（可以指定多个）
  -t <平台>              指定目标平台
  --script               生成导入脚本
  --help                 显示帮助信息

示例:
  gable --import -i file1.xlsx file2.xlsx
  gable --import --script --input config.xlsx -t unity"#
    );
}
