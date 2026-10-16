import win32gui, win32process

def enum_cb(hwnd, extra):
    _, pid = win32process.GetWindowThreadProcessId(hwnd)
    if pid == 51032:
        print(f"HWND: {hwnd}, Title: '{win32gui.GetWindowText(hwnd)}', Class: '{win32gui.GetClassName(hwnd)}', Visible: {win32gui.IsWindowVisible(hwnd)}, Rect: {win32gui.GetWindowRect(hwnd)}")

win32gui.EnumWindows(enum_cb, None)
