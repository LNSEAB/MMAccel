@echo off

set MMDDIR="MikuMikuDance_v932x64"

if not exist "%MMDDIR%\MMAccel" (
    mkdir "%MMDDIR%\MMAccel"
)
if exist "%MMDIR%\MMAccel\key_map.json" (
    del "%MMDIR%\MMAccel\key_map.json"
)

copy /Y /B "target\%1\d3d9.dll" "%MMDDIR%"
copy /Y /B "target\%1\mmaccel.dll" "%MMDDIR%\MMAccel"
copy /Y /B "target\%1\key_config.exe" "%MMDDIR%\MMAccel"
copy /Y /B "mmaccel\src\mmd_map.json" "%MMDDIR%\MMAccel"
copy /Y /B "key_config\src\order.json" "%MMDDIR%\MMAccel"
