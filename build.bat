@echo off
setlocal EnableExtensions EnableDelayedExpansion
cd /d "%~dp0"

where cargo >nul 2>&1
if errorlevel 1 (
  echo ERROR: cargo not found on PATH.
  echo Install Rust and add cargo to PATH, or check your Environment Variables.
  echo.
  pause
  exit /b 1
)

if not exist Cargo.toml (
  echo ERROR: Cargo.toml not found in current directory.
  echo.
  pause
  exit /b 1
)

set "APP_NAME=CLSIDShellFolderCreator"
set "APP_VERSION="
for /f "usebackq tokens=1,* delims==" %%A in ("version.ini") do (
  if /i "%%A"=="version" set "APP_VERSION=%%B"
)
if not defined APP_VERSION goto :fail
set "APP_VERSION=!APP_VERSION: =!"

set "OUTDIR=dist"
set "EXE_OUT=%OUTDIR%\%APP_NAME%_%APP_VERSION%.exe"

cls

if exist "%OUTDIR%" rmdir /s /q "%OUTDIR%" >nul 2>&1
mkdir "%OUTDIR%" 2>nul

cargo build --release
if errorlevel 1 goto :fail

copy /y "target\release\%APP_NAME%.exe" "%EXE_OUT%" >nul 2>&1
if errorlevel 1 goto :fail
if not exist "%EXE_OUT%" goto :fail

echo.
echo BUILD OK  -^> %EXE_OUT%
echo.

"%SystemRoot%\System32\timeout.exe" /t 3 /nobreak >nul 2>&1
exit /b 0

:fail
echo.
echo BUILD FAILED
echo.
pause
exit /b 1
