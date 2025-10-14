@echo off
setlocal enabledelayedexpansion

rem 创建输出目录（如果不存在）
if not exist "..\sample\Assets\Scripts\Tables" mkdir "..\sample\Assets\Scripts\Tables"

rem 为每个.proto文件生成C#代码
for %%i in (*.proto) do (
    echo Generating C# code for %%i
    "ProtoGen3/protogen.exe" --csharp_out=../sample/Assets/Scripts/Tables +names=original --proto_path=. %%i
)
exit