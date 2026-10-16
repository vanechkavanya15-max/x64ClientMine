import subprocess
import time
import json
import os
import win32gui
import win32process
import win32file
import win32pipe

def test_client():
    cwd = r"E:\m2dev client backlup\m2dev-client-main"
    exe_path = os.path.join(cwd, "metin2client.exe")
    args = [
        exe_path,
        "--server=m2dev",
        "--server-ip=127.0.0.1",
        "--auth-port=11000",
        "--game-port=11011",
        "--width=1024",
        "--height=768"
    ]

    print(f"[TEST] Uruchamianie: {' '.join(args)}")
    proc = subprocess.Popen(args, cwd=cwd)
    pid = proc.pid
    print(f"[TEST] Proces wystartowal z PID: {pid}")

    time.sleep(4.0)

    # Sprawdzenie czy proces zyje
    poll_res = proc.poll()
    if poll_res is not None:
        print(f"[TEST] BŁĄD: Proces zamknal sie wczesnie z kodem wyjscia: {poll_res}")
        return

    # Enumeracja okien
    found_windows = []
    def enum_cb(hwnd, extra):
        _, w_pid = win32process.GetWindowThreadProcessId(hwnd)
        if w_pid == pid:
            title = win32gui.GetWindowText(hwnd)
            cls_name = win32gui.GetClassName(hwnd)
            visible = win32gui.IsWindowVisible(hwnd)
            rect = win32gui.GetWindowRect(hwnd)
            found_windows.append({
                "hwnd": hwnd,
                "title": title,
                "class": cls_name,
                "visible": visible,
                "rect": rect
            })
    win32gui.EnumWindows(enum_cb, None)
    print(f"[TEST] Znalezione okna dla PID {pid}: {json.dumps(found_windows, indent=2)}")

    # Sprawdzenie komunikacji IPC
    pipe_name = r"\\.\pipe\M2ClientEngineHarness"
    ipc_ok = False
    engine_state = None
    try:
        handle = win32file.CreateFile(
            pipe_name,
            win32file.GENERIC_READ | win32file.GENERIC_WRITE,
            0,
            None,
            win32file.OPEN_EXISTING,
            0,
            None
        )
        req = json.dumps({"jsonrpc": "2.0", "method": "get_engine_state", "id": 1}) + "\n"
        win32file.WriteFile(handle, req.encode("utf-8"))
        _, resp_bytes = win32file.ReadFile(handle, 4096)
        resp = json.loads(resp_bytes.decode("utf-8").strip())
        engine_state = resp.get("result")
        ipc_ok = True
        win32file.CloseHandle(handle)
        print(f"[TEST] IPC Sukces: {json.dumps(engine_state, indent=2)}")
    except Exception as e:
        print(f"[TEST] IPC informacja: {e}")

    # Sprawdzenie logow z katalogu runtime
    log_path = os.path.join(cwd, "log", "log.txt")
    if os.path.exists(log_path):
        with open(log_path, "r", errors="ignore") as f:
            log_lines = f.readlines()
        print(f"[TEST] Ostatnie 10 linii log.txt:")
        for line in log_lines[-10:]:
            print("  ", line.strip())

    # Zakonczenie procesu testowego
    try:
        proc.terminate()
        proc.wait(timeout=3)
    except Exception:
        proc.kill()
    print("[TEST] Proces testowy pomyslnie zakonczony.")

if __name__ == "__main__":
    test_client()
