@echo off
setlocal enabledelayedexpansion
for %%i in ( *.proto ) do (
	rem echo %%~ni.cs
	"ProtoGen2/protogen.exe" -i:%%i -o:../sample/Assets/Scripts/Tables/%%~ni.cs -ns:ProtoBuf
)
exit