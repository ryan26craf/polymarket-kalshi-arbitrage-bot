@echo off
setlocal enabledelayedexpansion

echo Building Polymarket-Kalshi Arbitrage Bot for Windows...

:: Check Rust installation
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is not installed. Please install from https://rustup.rs/
    exit /b 1
)

:: Detect architecture
if "%PROCESSOR_ARCHITECTURE%"=="AMD64" (
    set TARGET=x86_64-pc-windows-msvc
    echo Building for x86_64...
) else if "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
    set TARGET=aarch64-pc-windows-msvc
    echo Building for ARM64...
) else (
    echo Unknown architecture: %PROCESSOR_ARCHITECTURE%
    exit /b 1
)

:: Install target if not present
rustup target add %TARGET%

:: Build release binary
echo Compiling release build...
cargo build --release --target %TARGET%

:: Create distribution directory
set DIST_DIR=dist\windows-%PROCESSOR_ARCHITECTURE%
if not exist "%DIST_DIR%" mkdir "%DIST_DIR%"

:: Copy binary
copy target\%TARGET%\release\polymarket-kalshi-arbitrage-bot.exe "%DIST_DIR%\"

:: Copy configuration files
copy config\default.toml "%DIST_DIR%\"
copy .env.example "%DIST_DIR%\.env"

:: Create README for distribution
(
echo Polymarket-Kalshi Arbitrage Bot - Windows Distribution
echo.
echo Installation:
echo 1. Edit .env file and configure your API keys
echo 2. Run: polymarket-kalshi-arbitrage-bot.exe --help
echo.
echo For full documentation, visit:
echo https://github.com/yourusername/polymarket-kalshi-arbitrage-bot
) > "%DIST_DIR%\README.txt"

echo.
echo Build complete! Binary located at: %DIST_DIR%\polymarket-kalshi-arbitrage-bot.exe
echo Run: cd %DIST_DIR% ^&^& polymarket-kalshi-arbitrage-bot.exe --help

endlocal
