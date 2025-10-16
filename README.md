# Gable

Gable 是一个用于处理Excel文件的图形化工具，支持将Excel数据转换为多种格式的配置文件，并能够根据Excel模板生成各种编程语言的代码。它的初衷是为了解决在使用多版本维护的项目中存在的Excel数据源冲突后不容易合并的得问题。此工具的原理是将json作为数据源，在编辑时把json序列化成Excel文件，并进行编辑，编辑完成后再把Excel文件序列化成json，并保存到文件中。Json文件的冲突解决起来要比Excel数据源冲突后容易合并。

## 功能特性

- 图形浏览数据
- 支持创建文件夹、Excel文件和Sheet工作表
- 可以导入/导出Excel文件
- 将Excel数据转换为多种格式的配置文件（如JSON、CSV、Protobuf等）
- 根据Excel模板自动生成多种编程语言的代码（C/C++、C#、Java、Python、JavaScript、Typescript、Golang、Python、Rust）
- 支持文件监控和实时更新
- 提供命令行接口进行批处理操作
- 支持自定义构建设置和模板

## 从源码构建

确保您已安装Rust开发环境：

```bash
git clone <repository-url>
cd gable-rust
cargo build --release
```

构建后的可执行文件位于 target/release/ 目录下。

## 使用方式

### 图形界面模式

直接运行Gable可执行文件启动图形界面：

```bash
./target/release/gable-rust
```

在图形界面中，您可以通过文件浏览器管理Excel文件：

- 创建文件夹 - 右键点击空白区域或文件夹，选择"新建文件夹"
- 创建Excel文件 - 右键点击空白区域或文件夹，选择"新建文件"
- 创建Sheet工作表 - 右键点击Excel文件，选择"新建文件"
- 导入Excel文件 - 右键点击文件夹，选择"导入"
- 编辑Excel文件 - 双击Excel文件或右键选择"编辑"
- 重命名项目 - 右键点击任意项目，选择"重命名"
- 删除项目 - 右键点击任意项目，选择"删除"
- 导出配置 - 右键点击Excel文件或Sheet，选择"导出"
- 生成代码 - 右键点击Excel文件或Sheet，选择"生成代码"

### 命令行模式

Gable也支持命令行操作：

```bash
--data:导出数据
--script：生成脚本

# 导出模式
./gable-rust export [参数]

# 导入模式
./gable-rust import [参数]

# 使用别名
./gable-rust e [参数]  # export
./gable-rust i [参数]  # import
```

### excel 结构

excel 类型：
- 普通配置表：前5行是表头,有效数据从第6行开始
  - 第一行：描述
  - 第二行：字段名
  - 第三行：字段类型
  - 第四行：导出平台关键字
  - 第五行：枚举类型和本地化key关联表。
  
- KV表：固定列数，第一行是表头，用作列说明，有效数据从第2行开始
  - 第一列：唯一Key（string类型）
  - 第二列：数据类型
  - 第三列：导出平台关键字
  - 第四列：枚举类型和本地化key关联表。
  - 第五列：数据值
  - 第六列：注释
  
- 枚举表：固定列数，第一行是表头，用作列说明，有效数据从第2行开始
  - 第一列：唯一Key（string类型）
  - 第二列：数据值（int类型）
  - 第三列：注释
  
- 本地化表：前5行是表头,有效数据从第6行开始，注意：同普通表单在数据类型上有区别
  - 第一行：描述
  - 第二行：字段名
  - 第三行：字段类型（只能是string类型）
  - 第四行：导出平台关键字
  - 第五行：枚举类型和本地化key关联表。

### 支持的数据类型

- int：32位整形
- long：64位整形
- string：字符串
- bool：布尔
- float：单精度浮点数
- vector2：二维向量，由";"分割开来，示例：1;2
- vector3：三维向量，由";"分割开来，示例：1.1;2.1;3.1
- vector4：四维向量，由";"分割开来，示例：1.1;2.1;3.1;4.1
- int[]：32位整形数组，由";"分割开来，示例：1;2
- long[]：64位整形数组，由";"分割开来
- string[]：字符串数组，由";"分割开来
- bool[]：布尔数组，由";"分割开来
- float[]：单精度浮点数数组，由";"分割开来
- vector2[]：二维向量数组，由";"和"|"分割开来,示例：1;2|5;3
- vector3[]：三维向量数组，由";"和"|"分割开来
- vector4[]：四维向量数组，由";"和"|"分割开来
- %：百分比，保留2位小数的单精度浮点数
- ‰：千分比，保留3位小数的单精度浮点数
- ‱：万分比，保留4位小数的单精度浮点数
- time：时间，已秒位单位的32位整形
- date：日期，已秒位单位的64位整形
- enum：枚举，前提要先创建对应的枚举表，由关联行或列的数据去查找关联的表单
- loc：本地化，本地化表key，由关联行或列的数据去查找关联的表单

### 支持的导出格式

Gable支持将Excel数据导出为以下格式：

- JSON - JavaScript Object Notation，广泛用于Web开发
- CSV - Comma-Separated Values，通用的数据交换格式
- XML - Extensible Markup Language，可扩展的标记语言
- YAML - YAML Ain't Markup Language，可读性高的数据序列化格式
- Protobuf - Google的Protocol Buffers，高效的序列化格式

### 支持的代码生成语言

Gable可以根据Excel模板自动生成以下编程语言的代码：

- C/C++ - 适用于系统编程和高性能应用
- C# - 适用于.NET平台和Unity游戏开发
- Cangjie - 仓颉编程语言
- Go - Google开发的现代编程语言
- Java - 广泛应用的企业级编程语言
- JavaScript - Web前端开发的主要语言
- Lua - 轻量级脚本语言，常用于游戏开发
- Python - 简洁易读的高级编程语言
- Rust - 语言设计者RalfG.Bjarne的现代语言
- TypeScript - JavaScript的超集，提供类型检查

### 构建设置

在"构建设置"中，您可以配置各种导出和代码生成选项：

- 选择目标开发环境
- 设置导出格式
- 配置导出路径
- 设置代码生成选项
- 自定义模板路径
- 配置后处理命令
- 项目结构

### 项目结构

```bash
gable-rust/
├── assets/           # 资源文件
│   ├── fonts/        # 字体文件
│   ├── icons/        # 图标文件
│   └── templates/    # 模板文件
├── src/              # 源代码
│   ├── cli/          # 命令行接口
│   ├── common/       # 公共模块
│   │   ├── convert/  # 数据转换模块
│   │   └── generate/ # 代码生成模块
│   └── gui/          # 图形界面
│       └── datas/    # GUI数据模型
└── Cargo.toml        # 项目配置文件
```

### 技术栈

- 语言: Rust
- GUI框架: eframe/egui
- Excel处理: calamine, umya-spreadsheet, rust_xlsxwriter
- 命令行解析: clap
- 文件监控: notify
- 模板引擎: tera
- 序列化: serde, serde_json
- 日志系统: log
  
### 使用流程

- 创建项目结构：使用文件浏览器创建文件夹组织您的Excel文件
- 创建Excel文件：右键菜单创建新的Excel文件和Sheet
- 编辑数据：在Excel编辑器中填充您的配置数据
- 配置构建设置：在"构建设置"中配置导出格式和代码生成选项
- 导出配置：右键菜单选择导出，将数据转换为目标格式
- 生成代码：右键菜单选择生成代码，根据模板自动生成代码文件

### 许可证

[待添加许可证信息]

联系方式
如有问题，请联系项目维护者（329737941@qq.com）