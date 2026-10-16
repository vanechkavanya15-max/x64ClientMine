import os
import sys
import time
import json
import struct
import argparse
import subprocess
from pathlib import Path

try:
    import win32file
    import win32gui
    import win32process
    import pywintypes
except ImportError:
    win32file = None
    win32gui = None
    win32process = None
    pywintypes = None

PIPE_NAME = r"\\.\pipe\M2ClientEngineHarness"
WORKSPACE_DIR = Path(r"E:\m2dev client backlup")
CLIENT_DIR = WORKSPACE_DIR / "m2dev-client-main"
EXE_PATH = CLIENT_DIR / "metin2client.exe"
DUMPER_EXE = WORKSPACE_DIR / "tools" / "dump_threads.exe"
DEFAULT_DUMP_PATH = WORKSPACE_DIR / "tools" / "client_deadlock.dmp"
REPORT_PATH = WORKSPACE_DIR / "tools" / "diagnosis_report.json"

DEFAULT_CLIENT_ARGS = [
    "--server=m2dev",
    "--server-ip=127.0.0.1",
    "--auth-port=11000",
    "--game-port=11011",
    "--width=1024",
    "--height=768",
]

def find_running_client_pids():
    pids = []
    try:
        output = subprocess.check_output(
            ["tasklist", "/FI", "IMAGENAME eq metin2client.exe", "/FO", "CSV", "/NH"],
            text=True, errors="ignore"
        )
        for line in output.strip().splitlines():
            parts = line.split('","')
            if len(parts) >= 2:
                pid_str = parts[1].replace('"', '').strip()
                if pid_str.isdigit():
                    pids.append(int(pid_str))
    except Exception as e:
        print(f"[WARN] Blad sprawdzania tasklist: {e}")
    return pids

def launch_client():
    print(f"[INFO] Uruchamianie klienta: {EXE_PATH}")
    print(f"[INFO] Parametry: {' '.join(DEFAULT_CLIENT_ARGS)}")
    
    log_out = open(CLIENT_DIR / "log_live_stdout.txt", "w", encoding="utf-8", errors="ignore")
    log_err = open(CLIENT_DIR / "log_live_stderr.txt", "w", encoding="utf-8", errors="ignore")
    
    cmd = [str(EXE_PATH)] + DEFAULT_CLIENT_ARGS
    creation_flags = 0
    if hasattr(subprocess, "DETACHED_PROCESS") and hasattr(subprocess, "CREATE_NEW_PROCESS_GROUP"):
        creation_flags = subprocess.DETACHED_PROCESS | subprocess.CREATE_NEW_PROCESS_GROUP

    proc = subprocess.Popen(
        cmd,
        cwd=str(CLIENT_DIR),
        stdout=log_out,
        stderr=log_err,
        creationflags=creation_flags
    )
    print(f"[INFO] Uruchomiono proces PID: {proc.pid}")
    return proc

def check_window_for_pid(pid):
    windows = []
    if not win32gui or not win32process:
        return windows

    def enum_cb(hwnd, extra):
        _, w_pid = win32process.GetWindowThreadProcessId(hwnd)
        if w_pid == pid:
            title = win32gui.GetWindowText(hwnd)
            cls = win32gui.GetClassName(hwnd)
            visible = bool(win32gui.IsWindowVisible(hwnd))
            rect = win32gui.GetWindowRect(hwnd)
            windows.append({
                "hwnd": hwnd,
                "title": title,
                "class": cls,
                "visible": visible,
                "rect": rect
            })

    win32gui.EnumWindows(enum_cb, None)
    return windows

def query_named_pipe(method, params={}, timeout_sec=6.0):
    if not win32file:
        return {"error": "Brak modulu win32file"}

    h_pipe = None
    t0 = time.time()
    while time.time() - t0 < timeout_sec:
        try:
            h_pipe = win32file.CreateFile(
                PIPE_NAME,
                win32file.GENERIC_READ | win32file.GENERIC_WRITE,
                0, None,
                win32file.OPEN_EXISTING,
                0, None
            )
            break
        except Exception:
            time.sleep(0.2)

    if not h_pipe:
        return {"error": f"Nie udalo sie polaczyc z potokiem {PIPE_NAME} w ciagu {timeout_sec}s"}

    try:
        req = {"jsonrpc": "2.0", "id": int(time.time() * 1000) % 100000, "method": method, "params": params}
        payload = json.dumps(req).encode("utf-8")
        framed = struct.pack("<I", len(payload)) + payload

        win32file.WriteFile(h_pipe, framed)
        
        # Oczekiwanie na odpowiedz z naglowkiem 4B
        hr, hdr = win32file.ReadFile(h_pipe, 4)
        if len(hdr) < 4:
            win32file.CloseHandle(h_pipe)
            return {"error": "Niepelny naglowek z potoku"}

        resp_len = struct.unpack("<I", hdr)[0]
        hr, body = win32file.ReadFile(h_pipe, resp_len)
        win32file.CloseHandle(h_pipe)

        return json.loads(body.decode("utf-8", errors="ignore"))
    except Exception as e:
        if h_pipe:
            try:
                win32file.CloseHandle(h_pipe)
            except Exception:
                pass
        return {"error": f"Wyjatek podczas komunikacji RPC: {e}"}

def run_thread_dumper(pid, dump_path):
    if not DUMPER_EXE.exists():
        return None, f"Brak pliku {DUMPER_EXE}"

    cmd = [str(DUMPER_EXE), str(pid), str(dump_path)]
    print(f"[INFO] Uruchamianie analizatora stosu Win32 DbgHelp: {' '.join(cmd)}")
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, cwd=str(WORKSPACE_DIR))
        raw_out = proc.stdout.strip()
        stderr_out = proc.stderr.strip()
        if stderr_out:
            print(f"[DUMPER STDERR] {stderr_out}")

        # Parsowanie wyjscia JSON
        # Obcinamy ewentualne komunikaty przed JSON
        json_start = raw_out.find("{")
        if json_start != -1:
            data = json.loads(raw_out[json_start:])
            return data, None
        else:
            return None, f"Niepoprawne wyjscie dumpera: {raw_out}"
    except Exception as e:
        return None, f"Blad wykonania dump_threads: {e}"

def analyze_threads(thread_data):
    analysis = {
        "main_thread": None,
        "worker_threads": [],
        "root_cause": None,
        "hanging_function": None,
        "hanging_file": None,
        "hanging_line": None,
        "wait_type": None,
    }

    if not thread_data or "threads" not in thread_data:
        return analysis

    threads = thread_data["threads"]
    for t in threads:
        is_main = t.get("is_main", False)
        tid = t.get("tid")
        frames = t.get("frames", [])

        t_summary = {
            "tid": tid,
            "is_main": is_main,
            "top_frames": frames[:5] if frames else [],
            "state_desc": "Nieznany"
        }

        if is_main:
            analysis["main_thread"] = t_summary
            # Szukanie dokladnej przyczyny wiszenia watku glownego
            for f in frames:
                sym = f.get("symbol", "")
                mod = f.get("module", "")
                file = f.get("file", "")
                line = f.get("line", 0)

                # Sprawdzenie czy to funkcja czekajaca
                if "WaitForSingleObject" in sym or "NtWaitForSingleObject" in sym:
                    analysis["wait_type"] = "WaitForSingleObject"
                elif "WaitForMultipleObjects" in sym or "NtWaitForMultipleObjects" in sym:
                    analysis["wait_type"] = "WaitForMultipleObjects"
                elif "WaitOnAddress" in sym or "RtlpWaitOnAddress" in sym or "NtWaitForAlertByAddress" in sym:
                    analysis["wait_type"] = "std::mutex / WaitOnAddress (blokada watku)"
                elif "EnterCriticalSection" in sym or "RtlEnterCriticalSection" in sym:
                    analysis["wait_type"] = "EnterCriticalSection"
                elif "Sleep" in sym or "NtDelayExecution" in sym:
                    analysis["wait_type"] = "Sleep / DelayExecution"
                elif "recv" in sym or "WSARecv" in sym:
                    analysis["wait_type"] = "Gniazdo sieciowe (blokujacy recv)"
                elif "GetMessage" in sym or "PeekMessage" in sym:
                    analysis["wait_type"] = "Petla komunikatow Win32"

                # Pierwsza ramka pochodzaca z kodu Metin2 (lub z biblioteki powiazanej)
                if mod in ("metin2client", "Metin2_Release", "Metin2") and not analysis["hanging_function"]:
                    analysis["hanging_function"] = sym
                    analysis["hanging_file"] = file
                    analysis["hanging_line"] = line

            # Jesli nadal brak powiazania z Metin2, bierzemy pierwsza nie-ntdll ramke
            if not analysis["hanging_function"] and frames:
                for f in frames:
                    if f.get("module") not in ("ntdll", "KERNELBASE", "KERNEL32"):
                        analysis["hanging_function"] = f.get("symbol")
                        analysis["hanging_file"] = f.get("file")
                        analysis["hanging_line"] = f.get("line")
                        break
        else:
            analysis["worker_threads"].append(t_summary)

    return analysis

def main():
    parser = argparse.ArgumentParser(description="Live Process Deadlock Analyzer dla metin2client.exe")
    parser.add_argument("--pid", type=int, default=None, help="Istniejacy PID procesu metin2client.exe")
    parser.add_argument("--launch", action="store_true", help="Wymus uruchomienie nowej instancji metin2client.exe")
    parser.add_argument("--wait-sec", type=float, default=6.0, help="Czas oczekiwania na ustabilizowanie procesu")
    parser.add_argument("--dump-path", type=str, default=str(DEFAULT_DUMP_PATH), help="Sciezka zapisu minidumpa .dmp")
    args = parser.parse_args()

    pid = args.pid
    spawned_proc = None

    if not pid:
        existing_pids = find_running_client_pids()
        if existing_pids and not args.launch:
            pid = existing_pids[0]
            print(f"[INFO] Znaleziono dzialajacy proces metin2client.exe PID: {pid}")
        else:
            spawned_proc = launch_client()
            pid = spawned_proc.pid

    print(f"\n=======================================================")
    print(f"  Live Process Deadlock Analyzer - Metin2 Client 2026")
    print(f"  Cel PID: {pid}")
    print(f"=======================================================\n")

    print(f"[INFO] Czekanie {args.wait_sec}s na inicjalizacje i potencjalne zawieszenie...")
    time.sleep(args.wait_sec)

    # 1. Sprawdzenie okna Win32
    print("\n--- [1] Sprawdzanie okien Win32 procesu ---")
    windows = check_window_for_pid(pid)
    if windows:
        for w in windows:
            print(f"  HWND: {w['hwnd']}, Tytul: '{w['title']}', Klasa: '{w['class']}', Widoczne: {w['visible']}, Rozmiar: {w['rect']}")
    else:
        print("  [UWAGA] Brak jakichkolwiek okien Win32 utworzonych przez ten PID!")

    # 2. Testowanie potoku Named Pipe
    print("\n--- [2] Testowanie Named Pipe (\\\\.\\pipe\\M2ClientEngineHarness) ---")
    
    print("  -> Wysylanie RPC 'game_ping' (immediate)...")
    t_start = time.time()
    ping_resp = query_named_pipe("game_ping", timeout_sec=2.0)
    t_ping = time.time() - t_start
    print(f"     Odpowiedz ({t_ping*1000:.1f}ms): {ping_resp}")

    print("  -> Wysylanie RPC 'get_engine_state' (kolejkowane do glownego watku)...")
    t_start = time.time()
    eng_state_resp = query_named_pipe("get_engine_state", timeout_sec=6.0)
    t_eng = time.time() - t_start
    print(f"     Odpowiedz ({t_eng*1000:.1f}ms): {eng_state_resp}")

    print("  -> Wysylanie RPC 'get_state' (immediate w serwerze)...")
    t_start = time.time()
    state_resp = query_named_pipe("get_state", timeout_sec=6.0)
    t_state = time.time() - t_start
    print(f"     Odpowiedz ({t_state*1000:.1f}ms): {state_resp}")

    print("  -> Wysylanie RPC 'tick_frame' (kolejkowane do glownego watku CPythonApplication)...")
    t_start = time.time()
    tick_resp = query_named_pipe("tick_frame", {"count": 1}, timeout_sec=6.0)
    t_tick = time.time() - t_start
    print(f"     Odpowiedz ({t_tick*1000:.1f}ms): {tick_resp}")

    # 3. Zrzut i analiza watkow za pomoca Win32 API i DbgHelp
    print("\n--- [3] Zrzut i analiza watkow (DbgHelp Stack Walk) ---")
    thread_data, err = run_thread_dumper(pid, args.dump_path)
    if err:
        print(f"  [BLAD] {err}")
        return

    analysis = analyze_threads(thread_data)

    print(f"\nLaczna liczba watkow w procesie: {thread_data.get('thread_count', 0)}")
    
    main_th = analysis.get("main_thread")
    if main_th:
        print(f"\n=======================================================")
        print(f"  GLOWNY WATEK (Main Thread TID: {main_th['tid']})")
        print(f"  Typ oczekiwania / blokady: {analysis.get('wait_type', 'Nieznany')}")
        print(f"  Funkcja zatrzymania: {analysis.get('hanging_function', 'Brak')}")
        print(f"  Plik zrodlowy: {analysis.get('hanging_file', 'Brak')}:{analysis.get('hanging_line', 0)}")
        print(f"=======================================================")
        print("  Kompletny stos wywolan glownego watku:")
        for idx, f in enumerate(thread_data["threads"][0].get("frames", [])):
            pc = f.get("pc")
            mod = f.get("module")
            sym = f.get("symbol")
            disp = f.get("disp")
            file = f.get("file")
            line = f.get("line")
            loc_str = f" ({file}:{line})" if file and line else ""
            print(f"    #{idx:02d} {pc} [{mod}] {sym} + 0x{disp:x}{loc_str}")

    print("\nWatki pomocnicze (Worker Threads):")
    for idx, t in enumerate(analysis.get("worker_threads", [])):
        print(f"  Thread #{idx+1} TID {t['tid']}:")
        for fidx, f in enumerate(t.get("top_frames", [])):
            loc_str = f" ({f.get('file')}:{f.get('line')})" if f.get('file') and f.get('line') else ""
            print(f"      #{fidx} {f.get('pc')} [{f.get('module')}] {f.get('symbol')}{loc_str}")

    # Zapis raportu do pliku JSON
    report = {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "pid": pid,
        "windows": windows,
        "rpc": {
            "game_ping": {"elapsed_ms": t_ping * 1000, "result": ping_resp},
            "get_engine_state": {"elapsed_ms": t_eng * 1000, "result": eng_state_resp},
            "get_state": {"elapsed_ms": t_state * 1000, "result": state_resp},
            "tick_frame": {"elapsed_ms": t_tick * 1000, "result": tick_resp},
        },
        "minidump": args.dump_path,
        "analysis": analysis,
        "raw_thread_data": thread_data
    }

    with open(REPORT_PATH, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    print(f"\n[INFO] Raport diagnostyczny zapisano w: {REPORT_PATH}")
    print(f"[INFO] Minidump zapisano w: {args.dump_path}")

    return report

if __name__ == "__main__":
    main()
