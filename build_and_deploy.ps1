# ==============================================================================
# Automatyczny skrypt budowania i wdrazania silnika klienta do srodowiska testowego
# Kod zrodlowy: E:\SourceCodeClient
# Srodowisko testowe: E:\Metin2KlientCustom
# ==============================================================================
param(
    [switch]$SkipTests = $false
)

$ErrorActionPreference = "Stop"
$clientSourceDir = "E:\SourceCodeClient"
$testClientDir   = "E:\Metin2KlientCustom"
$msbuildPath     = "C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe"

Write-Host "=== 1. TESTY JEDNOSTKOWE RUST (cargo test) ===" -ForegroundColor Cyan
if (-not $SkipTests) {
    Set-Location $clientSourceDir
    cargo test --workspace
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Testy jednostkowe Rust nie przeszly! Przerywam wdrozenie."
    }
}

Write-Host "=== 2. BUDOWANIE 32-BITOWEJ BIBLIOTEKI RUST (i686-pc-windows-msvc) ===" -ForegroundColor Cyan
Set-Location $clientSourceDir
cargo build --release --target i686-pc-windows-msvc --manifest-path "$clientSourceDir\rust_bridge\Cargo.toml"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Blad kompilacji mostka Rusta!"
}

# Kopiowanie import library do extern/library dla MSVC Linkera
Copy-Item "$clientSourceDir\target\i686-pc-windows-msvc\release\m2_client_bridge.dll.lib" "$clientSourceDir\extern\library\m2_client_bridge.lib" -Force
Copy-Item "$clientSourceDir\target\i686-pc-windows-msvc\release\m2_client_bridge.dll" "$clientSourceDir\binary\m2_client_bridge.dll" -Force

Write-Host "=== 3. KOMPILACJA KLIENTA C++ (client.sln - Distribute Win32) ===" -ForegroundColor Cyan
& $msbuildPath "$clientSourceDir\client.sln" /p:Configuration=Distribute /p:Platform=Win32 /m /v:m
if ($LASTEXITCODE -ne 0) {
    Write-Error "Blad kompilacji rozwiazania C++ w MSBuild!"
}


Write-Host "=== 4. WDRAZANIE DO SRODOWISKA TESTOWEGO ($testClientDir) ===" -ForegroundColor Cyan
if (-not (Test-Path "$testClientDir\Metin2Distribute_original_backup.exe")) {
    if (Test-Path "$testClientDir\Metin2Distribute.exe") {
        Copy-Item "$testClientDir\Metin2Distribute.exe" "$testClientDir\Metin2Distribute_original_backup.exe" -Force
    }
}

Copy-Item "$clientSourceDir\binary\Metin2Distribute.exe" "$testClientDir\Metin2Distribute.exe" -Force
Copy-Item "$clientSourceDir\binary\m2_client_bridge.dll" "$testClientDir\m2_client_bridge.dll" -Force

Write-Host "=== SUKCES: Nowy silnik z mostkiem Rusta zostal wdroznoy do $testClientDir! ===" -ForegroundColor Green
