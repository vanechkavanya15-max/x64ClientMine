@echo off
setlocal enabledelayedexpansion

echo ========================================================
echo [JULES VERIFICATION GATE] Sprawdzanie kompilacji C++20
echo ========================================================

if not exist build mkdir build

cmake -B build -A x64 -DCMAKE_BUILD_TYPE=Release
if %ERRORLEVEL% neq 0 (
    echo [BLAD] Konfiguracja CMake nie powiodla sie!
    exit /b 1
)

cmake --build build --config Release --target UserInterface -j 8
if %ERRORLEVEL% neq 0 (
    echo [BLAD] Kompilacja UserInterface zakonczona niepowodzeniem!
    exit /b 1
)

echo ========================================================
echo [SUKCES] Projekt kompiluje sie bez bledow!
echo ========================================================
exit /b 0
