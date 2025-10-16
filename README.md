# Gable

Gable 是一个用于处理Excel文件的图形化工具，支持将Excel数据转换为多种格式的配置文件，并能够根据Excel模板生成各种编程语言的代码。它的初衷是为了解决在使用多版本维护的项目中存在的Excel数据源冲突后不容易合并的得问题。此工具的原理是将json作为数据源，在编辑时把json序列化成Excel文件，并进行编辑，编辑完成后再把Excel文件序列化成json，并保存到文件中。Json文件的冲突解决起来要比Excel数据源冲突后容易合并。

# 功能特性

- 图形浏览数据
- 支持创建文件夹、Excel文件和Sheet工作表
- 可以导入/导出Excel文件
- 将Excel数据转换为多种格式的配置文件（如JSON、CSV、XML,YAML,Protobuf）
- 根据Excel模板自动生成多种编程语言的代码（C/C++、C#、Java、Python、JavaScript、Typescript、Golang、Python、Rust）
- 支持文件监控和实时更新
- 提供命令行接口进行批处理操作
- 支持自定义构建设置和模板

# 从源码构建

确保您已安装Rust开发环境：

```bash
git clone <repository-url>
cd gable-rust
cargo build --release
```

构建后的可执行文件位于 target/release/ 目录下。

# 使用方式

## 图形界面模式

直接运行Gable可执行文件启动图形界面,图形界面分成五个模块：

- 顶部菜单栏
- 左侧导航栏
- 资源管理器
- 表单预览
- 日志

### 顶部菜单介绍

- 菜单
  - 新建文件：工作目录的根目录下创建一个Excel文件
  - 新建文件夹：工作目录的根目录下创建一个文件夹
  - 打开工程目录：设置工作目录
  - 设置：暂无实现
  - 退出
- 编译
  - 编译设置：打开编译设置界面
  - 快速编译：根据编译设置全量编译（导出配置 & 生成胶水代码）
- 选择
  - 导入Excel：工作目录的根目录下导入Excel文件
- 帮助
  - 关于
  - Language：语言切换
  - 主题: 主题切换

### 左侧导航栏

- 资源管理器：参见文件管理器
- 搜索：暂未实现

### 资源管理器

- 管理工作空间下的所有gable文件，支持创建、删除、重名民、导入、编译、预览。通过右键菜单进行操作。
- 目录的树分类有三种：文件夹，Excel，Sheet。点击文件夹和Excel是展开，双击Excel，Sheet是进行预览
- 目录树的类型不同，右键菜单不同，区别不大
- 特殊目录：kvs,enums,localizes，三个保留目录，用来区分表单类型，分别是：KV表，枚举表，本地化表（要导入的表单类型通过在不同目录下右键导入来控制），其他目录下的表单都是普通表单，项目中的保留目录被过滤掉了："__Data","__Temp"

### 表单预览

- 双击Excel或Sheet，都会进入预览。顶部是打开的Excel文件，底部是每个Excel中的Sheet表单。

### 日志

- 一些表要的操作会记录到日志中，日志文件存储在工作空间/__Temp/__Logs/ 目录下

## 命令行模式

Gable也支持命令行操作：

```bash
# 导出配置
export:导出配置
--data:导出数据
--script：生成脚本
-f：指定文件名，参数是Sheetname列表
# 案例
./gable.exe export --data --script -f Sheet1 Sheet2

# 导入配置
import:导入配置
-f:导入指定的excel文件，优先级低于 -d 参数，当-d有参数时，-f参数无效
-d:导入指定目录下的所有excel
-t:导入的目录，根据目录不同导入的数据表类型不同，参见配置表类型规则
# 案例
./gable.exe import -d "E:/projects/test" -t "E:/projects/configs"
```

### Excel 结构

Excel 类型：使用目录进行区分，工作空间下的一级目录来区分配置表类型，./enums：枚举表，./kvs：kv表，./localizes：本地化表。其他任意目录下的文件都是普通表。
注意：普通配置表的[字段名]带有“*”的含义是主键，每个表单都需要一个或两个主键的（枚举和kv表除外），本地化表理论上只有一个主键。本地化表[字段名]带有“#”的，是loc数据类型所关联要显示的值。

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

Gable支持将Excel数据导出为以下格式。
注意：普通配置表中的行列有效数据规则是，行：主键是空的行数据无效。列：表头中的字段名，数据类型，导出平台不符合规则的，列数据无效。

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

在"构建设置"中，您可以配置各种导出和代码生成选项，注意设置中的所有目录都是工作空间的相对目录

- 添加开发环境：以开发语种来区分，同一种语种支持多个。
- 标识（Tag）：建议不要有重复的标识
- 关键字（Keyword）: Excel中的关键字，只有匹配上了才会把当前有效数据导出
- 导出类型（Export Type）：支持 json,csv,xml,yaml,protobuff
- 导出路径（Export Psath）:导出的数据指定目录
- 是否生成脚本（Generate Script）:勾选后构建时会生成脚本
- 脚本路径(Script Path):生成的脚本路径，注意：Protobuff类型是proto文件的路径，Protobuff生成脚本流程是先生成proto文件，再通过后处理调用命令行工具去使用ProtoGen去生成脚本
- 自定义模板(Custom Template)：脚本模板使用的是Tera来生成的，需要自定义模板的请参考官方文档 ([Tera](https://docs.rs/tera/latest/tera/#getting-started))
- 后处理(Post Processing)：导出的数据或者脚本需要进行二次处理的，可在这里编写命令，原理是调用系统命令行参数，注意：其所执行的目录是工作空间目录。

### 自定义模板

需要自定脚本模板的除了需要熟悉Tera语法外，工具还抛出了一些参数以供扩展需要：

- CLASS_NAME: 脚本名称
- info: 脚本信息(List)
  - primary_num:主键个数
  - main_fields：主键List
    - field_type：字段类型
    - field_name：字段名称
  - fields：字段List（包括主键）
    - field_type：字段类型
    - field_name：字段名称
    - field_desc: 字段描述
    - field_index: 字段序号
    - field_extend：字段扩展信息（protobuff 2 版本中的枚举默认值）
    - data_type：数据类型，和field_type却别在于如果是枚举类型时，它的值是枚举的名字
- imports：string list，不同语言的到导入的语法不通，需要自定义导入处理


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