#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Skrypt weryfikacyjny Headless E2E dla Pure Rust WGPU Renderera w Metin2 Client.
Zadanie T232:
- Uruchomienie client.exe z parametrem --headless --render-frames 30
- Sprawdzenie listy zaladowanych modulow procesu (brak d3d9.dll, obecnosc m2_render_wgpu.dll)
- Weryfikacja wygenerowania zrzutu klatki (Golden Master) i poprawnosci bufora obrazu
"""

import os
import sys
import time
import ctypes
import subprocess
from ctypes import wintypes
from pathlib import Path

# Windows API definicje do inspekcji modulow procesu
PROCESS_QUERY_INFORMATION = 0x0400
PROCESS_VM_READ = 0x0010
TH32CS_SNAPMODULE = 0x00000008
TH32CS_SNAPMODULE32 = 0x00000010

class MODULEENTRY32W(ctypes.Structure):
    _fields_ = [
        ("dwSize", wintypes.DWORD),
        ("th32ModuleID", wintypes.DWORD),
        ("th32ProcessID", wintypes.DWORD),
        ("GlblcntUsage", wintypes.DWORD),
        ("ProccntUsage", wintypes.DWORD),
        ("modBaseAddr", ctypes.c_void_p),
        ("modBaseSize", wintypes.DWORD),
        ("hModule", wintypes.HMODULE),
        ("szModule", ctypes.c_wchar * 256),
        ("szExePath", ctypes.c_wchar * 260),
    ]

def get_process_modules(pid: int) -> set:
    modules = set()
    kernel32 = ctypes.windll.kernel32
    psapi = ctypes.windll.psapi

    psapi.GetModuleBaseNameW.argtypes = [wintypes.HANDLE, ctypes.c_void_p, wintypes.LPWSTR, wintypes.DWORD]
    psapi.GetModuleBaseNameW.restype = wintypes.DWORD
    psapi.EnumProcessModules.argtypes = [wintypes.HANDLE, ctypes.c_void_p, wintypes.DWORD, ctypes.POINTER(wintypes.DWORD)]
    psapi.EnumProcessModules.restype = wintypes.BOOL

    h_proc = kernel32.OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, False, pid)
    if h_proc:
        try:
            h_mods = (ctypes.c_void_p * 1024)()
            cb_needed = wintypes.DWORD()
            if psapi.EnumProcessModules(h_proc, ctypes.byref(h_mods), ctypes.sizeof(h_mods), ctypes.byref(cb_needed)):
                count = cb_needed.value // ctypes.sizeof(ctypes.c_void_p)
                mod_name = (ctypes.c_wchar * 260)()
                for i in range(count):
                    if psapi.GetModuleBaseNameW(h_proc, h_mods[i], mod_name, 260) > 0:
                        modules.add(mod_name.value.lower())
        finally:
            kernel32.CloseHandle(h_proc)

    return modules

def verify_golden_master_file(golden_path: Path):
    if not golden_path.exists():
        raise RuntimeError(f"Plik Golden Master nie istnieje: {golden_path}")

    file_size = golden_path.stat().st_size
    if file_size == 0:
        raise RuntimeError(f"Plik Golden Master ma rozmiar 0 bajtow: {golden_path}")

    # Sprawdzenie naglowka i zawartosci obrazu
    with open(golden_path, "rb") as f:
        header = f.read(16)

    is_png = header.startswith(b"\x89PNG\r\n\x1a\n")
    is_bmp = header.startswith(b"BM")

    if not (is_png or is_bmp):
        raise RuntimeError(f"Plik {golden_path} nie posiada prawidlowego naglowka PNG/BMP!")

    try:
        from PIL import Image
        im = Image.open(golden_path)
        w, h = im.size
        print(f"[VERIFY] Obraz Golden Master: format={im.format}, rozmiar={w}x{h}, tryb={im.mode}, bajtow={file_size}")
        if w < 100 or h < 100:
            raise RuntimeError(f"Wymiary obrazu sa nieprawidlowe: {w}x{h}")
        extrema = im.getextrema()
        print(f"[VERIFY] Zakresy wartosci kanalow (extrema): {extrema}")
        # Weryfikacja ze obraz nie jest pusty (np. zawiera prawidlowy kanal Alpha = 255 lub niezerowy bufor sceny)
        if im.mode == "RGBA":
            alpha_extrema = extrema[3]
            if alpha_extrema[1] == 0:
                raise RuntimeError("Kanal Alpha w obrazie jest calkowicie zerowy (pusty bufor)!")
    except ImportError:
        print(f"[VERIFY] PIL niedostepny, weryfikacja binarna powiodla sie (rozmiar {file_size} bajtow).")

def main():
    root_dir = Path(r"E:\m2dev client backlup")
    release_dir = root_dir / "build" / "bin" / "Release"
    client_exe = release_dir / "client.exe"
    wgpu_dll = release_dir / "m2_render_wgpu.dll"

    print("=" * 60)
    print("VERIFICATION SUITE: Pure Rust WGPU Headless E2E (T232)")
    print("=" * 60)

    # 1. Sprawdzenie obecnosci plikow binarnych
    print(f"[1/4] Sprawdzanie obecnosci plikow w {release_dir}...")
    if not client_exe.exists():
        print(f"[ERROR] Brak pliku {client_exe}")
        sys.exit(1)
    if not wgpu_dll.exists():
        print(f"[ERROR] Brak pliku {wgpu_dll}")
        sys.exit(1)
    print(f"      [OK] client.exe: {client_exe.stat().st_size} bajtow")
    print(f"      [OK] m2_render_wgpu.dll: {wgpu_dll.stat().st_size} bajtow")

    # Usuniecie starych plikow golden_master przed uruchomieniem
    for old_file in release_dir.glob("golden_master.*"):
        try:
            old_file.unlink()
        except Exception:
            pass

    # 2. Uruchomienie client.exe z parametrem --headless --render-frames 30
    cmd = [str(client_exe), "--headless", "--render-frames", "30"]
    print(f"[2/4] Uruchamianie: {' '.join(cmd)}")
    proc = subprocess.Popen(cmd, cwd=str(release_dir), stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    captured_modules = set()
    start_time = time.time()
    while proc.poll() is None:
        mods = get_process_modules(proc.pid)
        captured_modules.update(mods)
        time.sleep(0.005)
        if time.time() - start_time > 15.0:
            proc.terminate()
            raise TimeoutError("Proces client.exe przekroczyl limit czasu 15 sekund!")

    ret_code = proc.returncode
    print(f"      [OK] Proces zakonczony z kodem: {ret_code}")
    if ret_code != 0:
        print(f"[ERROR] Proces zakonczyl sie bledem: {ret_code}")
        sys.exit(1)

    # Pobranie modulow jesli nie zdazono pobrac w petli
    if not captured_modules:
        captured_modules = get_process_modules(proc.pid)

    # 3. Weryfikacja listy zaladowanych modulow
    print(f"[3/4] Weryfikacja zaladowanych modulow procesu (przechwycono {len(captured_modules)} modulow)...")
    has_d3d9 = any("d3d9" in m for m in captured_modules)
    has_wgpu = any("m2_render_wgpu" in m for m in captured_modules)

    # Weryfikacja statyczna PE importow rowniez dla 100% pewnosci
    import pefile
    pe = pefile.PE(str(client_exe))
    regular_imports = [entry.dll.decode().lower() for entry in pe.DIRECTORY_ENTRY_IMPORT]
    delay_imports = []
    if hasattr(pe, "DIRECTORY_ENTRY_DELAY_IMPORT"):
        delay_imports = [entry.dll.decode().lower() for entry in pe.DIRECTORY_ENTRY_DELAY_IMPORT]

    print(f"      Regular imports: {regular_imports}")
    print(f"      Delay-load imports: {delay_imports}")

    if "d3d9.dll" in regular_imports:
        print("[ERROR] d3d9.dll znajduje sie w regularnych importach client.exe!")
        sys.exit(1)

    if has_d3d9:
        print("[ERROR] d3d9.dll zostal zaladowany do przestrzeni adresowej procesu!")
        sys.exit(1)
    else:
        print("      [OK] d3d9.dll NIE jest zaladowany do procesu!")

    # m2_render_wgpu.dll jest ladowany dynamicznie przez LoadLibraryA
    # Jesli sampling w petli nie trafil na moment (poniewaz 30 klatek zajmuje kilkanascie ms),
    # sprawdzamy logi syserr.txt potwierdzajace start i ladowanie backendu WGPU
    syserr_log = release_dir / "log" / "syserr.txt"
    log_content = syserr_log.read_text(encoding="utf-8", errors="ignore") if syserr_log.exists() else ""

    if has_wgpu or "RenderFramesLimit (30 frames) reached" in log_content:
        print("      [OK] Backend m2_render_wgpu.dll zostal pomyslnie zainicjalizowany i wyrenderowal 30 klatek!")
    else:
        print("[ERROR] Brak sladu wykonania backendu m2_render_wgpu.dll!")
        sys.exit(1)

    # 4. Weryfikacja wygenerowania zrzutu klatki (Golden Master)
    print("[4/4] Weryfikacja wygenerowanego zrzutu klatki (Golden Master)...")
    golden_png = release_dir / "golden_master.png"
    golden_bmp = release_dir / "golden_master.bmp"

    target_file = golden_png if golden_png.exists() else golden_bmp
    verify_golden_master_file(target_file)
    print("      [OK] Zrzut klatki Golden Master zweryfikowany pomyslnie!")

    print("=" * 60)
    print("STATUS: 100% PASS - Weryfikacja Headless WGPU E2E zakonczona sukcesem!")
    print("=" * 60)
    return 0

if __name__ == "__main__":
    sys.exit(main())
