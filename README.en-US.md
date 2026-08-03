

# Gable

Gable is a GUI tool for processing Excel files. It supports converting Excel data into various configuration file formats and generating code in multiple programming languages based on Excel templates. Its original purpose is to solve the problem of difficult merge conflicts when using Excel data sources in projects with multi-version maintenance. The tool's principle is to use JSON as the data source: during editing, JSON is serialized into an Excel file for editing, and after editing, the Excel file is serialized back into JSON and saved to a file. Resolving conflicts in JSON files is much easier than merging Excel data source conflicts.

# Features

- Graphical data browsing
- Supports creating folders, Excel files, and Sheet worksheets
- Can import/export Excel files
- Converts Excel data into various configuration file formats (e.g., JSON, CSV, XML, YAML, Protobuf)
- Automatically generates code in multiple programming languages based on Excel templates (C/C++, C#, Java, Python, JavaScript, TypeScript, Golang, Python, Rust)
- Supports file monitoring and real-time updates
- Provides a command-line interface for batch operations
- Supports custom build settings and templates

# Build from Source

Ensure you have the Rust development environment installed:

```bash
git clone <repository-url>
cd gable-rust
cargo build --release
```

The built executable is located in the `target/release/` directory.

# Usage

## GUI Mode

Run the Gable executable directly to start the GUI. The interface is divided into five modules:

- Top menu bar
- Left navigation bar
- Resource Explorer
- Form Preview
- Logs

### Top Menu Introduction

- Menu
  - New File: Create an Excel file in the root directory of the working directory
  - New Folder: Create a folder in the root directory of the working directory
  - Open Project Directory: Set the working directory
  - Settings: Not yet implemented
  - Exit
- Build
  - Build Settings: Open the build settings interface
  - Quick Build: Full build based on build settings (export configs & generate glue code)
- Select
  - Import Excel: Import Excel files into the root directory of the working directory
- Help
  - About
  - Language: Language switch
  - Theme: Theme switch

### Left Navigation Bar

- Resource Explorer: See File Explorer
- Search: Not yet implemented

### Resource Explorer

- Manages all Gable files under the workspace, supporting creation, deletion, renaming, import, compilation, and preview. Operate via the right-click context menu.
- The directory tree has three categories: Folders, Excel, and Sheet. Clicking a folder or Excel file expands it; double-clicking an Excel file or Sheet opens the preview.
- Different directory tree types have slightly different right-click context menus.
- Special directories: `kvs`, `enums`, `localizes` are three reserved directories used to distinguish form types: KV tables, Enum tables, and Localization tables. The imported form type is controlled by importing via right-click in different directories. Forms under other directories are regular forms. Reserved directories filtered in the project: `__Data`, `__Temp`.

### Form Preview

- Double-clicking an Excel file or Sheet enters preview mode. The top shows the opened Excel file, and the bottom shows the Sheet forms within each Excel file.

### Logs

- Operations on some forms are recorded in the logs. Log files are stored in the `workspace/__Temp/__Logs/` directory.

## CLI Mode

Gable also supports command-line operations:

```bash
# Export Config
export: Export configuration
--data: Export data
--script: Generate script
-f: Specify file name, argument is a list of Sheet names
# Example
./gable.exe export --data --script -f Sheet1 Sheet2

# Import Config
import: Import configuration
-f: Import specified Excel file, lower priority than -d parameter; -f is invalid when -d has arguments
-d: Import all Excel files under the specified directory
-t: Import directory, data table types imported vary based on the directory, see config table type rules
# Example
./gable.exe import -d "E:/projects/test" -t "E:/projects/configs"
```

### Excel Structure

Excel Types: Distinguished by directories. The first-level directories under the workspace distinguish configuration table types: `./enums`: Enum tables, `./kvs`: KV tables, `./localizes`: Localization tables. Files under any other directory are regular tables.
Note: For regular configuration tables, the `[Field Name]` with a `*` denotes the primary key. Each form requires one or two primary keys (except for Enum and KV tables). Localization tables theoretically have only one primary key. For localization tables, `[Field Name]` with a `#` denotes the value to be displayed associated with the `loc` data type.

- Regular Configuration Table: The first 5 rows are headers; valid data starts from row 6.
  - Row 1: Description
  - Row 2: Field Name
  - Row 3: Field Type
  - Row 4: Export Platform Keyword
  - Row 5: Enum type and localization key association table.
  
- KV Table: Fixed columns, first row is header used for column description, valid data starts from row 2.
  - Column 1: Unique Key (string type)
  - Column 2: Data Type
  - Column 3: Export Platform Keyword
  - Column 4: Enum type and localization key association table.
  - Column 5: Data Value
  - Column 6: Comment
  
- Enum Table: Fixed columns, first row is header used for column description, valid data starts from row 2.
  - Column 1: Unique Key (string type)
  - Column 2: Data Value (int type)
  - Column 3: Comment
  
- Localization Table: The first 5 rows are headers; valid data starts from row 6. Note: Differs from regular forms in data type.
  - Row 1: Description
  - Row 2: Field Name
  - Row 3: Field Type (must be string type)
  - Row 4: Export Platform Keyword
  - Row 5: Enum type and localization key association table.

### Supported Data Types

- `int`: 32-bit integer
- `long`: 64-bit integer
- `string`: String
- `bool`: Boolean
- `float`: Single-precision floating-point number
- `vector2`: 2D vector, separated by `;`, example: `1;2`
- `vector3`: 3D vector, separated by `;`, example: `1.1;2.1;3.1`
- `vector4`: 4D vector, separated by `;`, example: `1.1;2.1;3.1;4.1`
- `int[]`: 32-bit integer array, separated by `;`, example: `1;2`
- `long[]`: 64-bit integer array, separated by `;`
- `string[]`: String array, separated by `;`
- `bool[]`: Boolean array, separated by `;`
- `float[]`: Single-precision floating-point array, separated by `;`
- `vector2[]`: 2D vector array, separated by `;` and `|`, example: `1;2|5;3`
- `vector3[]`: 3D vector array, separated by `;` and `|`
- `vector4[]`: 4D vector array, separated by `;` and `|`
- `%`: Percentage, single-precision float with 2 decimal places
- `‰`: Per mille, single-precision float with 3 decimal places
- `‱`: Percentage point (1/10000), single-precision float with 4 decimal places
- `time`: Time, 32-bit integer in seconds
- `date`: Date, 64-bit integer in seconds
- `enum`: Enum, requires creating a corresponding enum table first. Looks up the associated form via data in the associated row or column.
- `loc`: Localization, localization table key. Looks up the associated form via data in the associated row or column.

### Supported Export Formats

Gable supports exporting Excel data to the following formats.
Note: For regular configuration tables, the valid data rule for rows/columns is: Row: rows with empty primary keys are invalid. Column: columns where the header's field name, data type, or export platform do not match the rules are invalid.

- JSON - JavaScript Object Notation, widely used in Web development
- CSV - Comma-Separated Values, a universal data exchange format
- XML - Extensible Markup Language, an extensible markup language
- YAML - YAML Ain't Markup Language, a highly readable data serialization format
- Protobuf - Google's Protocol Buffers, an efficient serialization format

### Supported Code Generation Languages

Gable can automatically generate code for the following programming languages based on Excel templates:

- C/C++ - Suitable for system programming and high-performance applications
- C# - Suitable for .NET platform and Unity game development
- Cangjie - Cangjie programming language
- Go - Modern programming language developed by Google
- Java - Widely used enterprise-level programming language
- JavaScript - Primary language for Web frontend development
- Lua - Lightweight scripting language, commonly used in game development
- Python - Concise and readable high-level programming language
- Rust - Modern language
- TypeScript - Superset of JavaScript, providing type checking

### Build Settings

In "Build Settings", you can configure various export and code generation options. Note that all directories in the settings are relative to the workspace.

- Add Development Environment: Distinguished by development language; multiple environments are supported for the same language.
- Tag: It is recommended to avoid duplicate tags.
- Keyword: Keywords in Excel; only matching valid data will be exported.
- Export Type: Supports json, csv, xml, yaml, protobuf
- Export Path: Target directory for exported data
- Generate Script: When checked, scripts will be generated during the build process.
- Script Path: Path for generated scripts. Note: For Protobuf type, this is the path for proto files. The Protobuf script generation process first generates the proto file, then calls command-line tools via post-processing to use ProtoGen to generate scripts.
- Custom Template: Script templates are generated using Tera. For custom templates, please refer to the official documentation ([Tera](https://docs.rs/tera/latest/tera/#getting-started))
- Post Processing: If exported data or scripts require secondary processing, commands can be written here. The principle is to call system command-line parameters. Note: The execution directory is the workspace directory.

### Custom Templates

Besides being familiar with Tera syntax, the tool exposes some parameters for extension needs:

- `CLASS_NAME`: Script name
- `info`: Script information (List)
  - `primary_num`: Number of primary keys
  - `main_fields`: Primary key List
    - `field_type`: Field type
    - `field_name`: Field name
  - `fields`: Field List (including primary keys)
    - `field_type`: Field type
    - `field_name`: Field name
    - `field_desc`: Field description
    - `field_index`: Field index
    - `field_extend`: Field extension information (default value for enums in Protobuf v2)
    - `data_type`: Data type. The difference from `field_type` is that if it's an enum type, its value is the enum's name.
- `imports`: String list. Import syntax varies by language, requiring custom import handling.

### Tech Stack

- Language: Rust
- GUI Framework: eframe/egui
- Excel Processing: calamine, umya-spreadsheet, rust_xlsxwriter
- CLI Parsing: clap
- File Monitoring: notify
- Template Engine: tera
- Serialization: serde, serde_json
- Logging System: log
  
### Usage Workflow

- Create Project Structure: Use the file browser to create folders to organize your Excel files
- Create Excel Files: Use the right-click context menu to create new Excel files and Sheets
- Edit Data: Fill in your configuration data in the Excel editor
- Configure Build Settings: Configure export formats and code generation options in "Build Settings"
- Export Config: Use the right-click context menu to select export, converting data to the target format
- Generate Code: Use the right-click context menu to select generate code, automatically creating code files based on templates

### Community

QQ Group: 1050754370
