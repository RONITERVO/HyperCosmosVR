@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0"

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\stage_runtime_deps.ps1"
if errorlevel 1 (
    echo [HyperCosmosVR] ERROR: Runtime dependency staging failed.
    exit /b 1
)

set "EXE_PATH=%~dp0target\release\cosmos_app.exe"

where cmake >nul 2>nul
if errorlevel 1 if not defined CMAKE (
    for %%V in (2026 2022 2019) do (
        for %%E in (Community Professional Enterprise BuildTools) do (
            for %%A in ("Program Files" "Program Files (x86)") do (
                set "_CAND=C:\%%~A\Microsoft Visual Studio\%%V\%%E\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe"
                if exist "!_CAND!" if not defined CMAKE set "CMAKE=!_CAND!"
            )
        )
    )
)

where cargo >nul 2>nul
if not errorlevel 1 (
    echo [HyperCosmosVR] Building release binary (cosmos_app^)...
    cargo build --release -p cosmos_app
    if errorlevel 1 (
        echo [HyperCosmosVR] ERROR: cargo build --release -p cosmos_app failed.
        exit /b 1
    )
    powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\stage_runtime_deps.ps1" >nul
)

if not exist "%EXE_PATH%" (
    echo [HyperCosmosVR] ERROR: Release executable not found at "%EXE_PATH%".
    exit /b 1
)

echo [HyperCosmosVR] Launching "%EXE_PATH%" %*
"%EXE_PATH%" %*
exit /b %errorlevel%

