@echo off
set tool = ProtoGen
set proto = protocol/HeroQuality.proto
"protogen.exe" -i:HeroQuality.proto -o:protocol/HeroQuality.cs -ns:ProtoBuf
"protogen.exe" -i:Unit.proto -o:protocol/Unit.cs -ns:ProtoBuf
"protogen.exe" -i:Global.proto -o:protocol/Global.cs -ns:ProtoBuf
pause