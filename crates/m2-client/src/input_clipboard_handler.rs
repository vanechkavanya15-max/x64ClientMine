#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use std::ptr;

pub struct ClipboardHandler;

impl ClipboardHandler {
    pub fn new() -> Self {
        Self
    }

    #[cfg(windows)]
    pub fn copy_text(&self, text: &str) -> bool {
        unsafe {
            if OpenClipboard(ptr::null_mut()) == 0 {
                return false;
            }

            if EmptyClipboard() == 0 {
                CloseClipboard();
                return false;
            }

            let wide_text: Vec<u16> = OsStr::new(text).encode_wide().chain(Some(0)).collect();
            let size_in_bytes = wide_text.len() * std::mem::size_of::<u16>();

            let h_mem = GlobalAlloc(GMEM_MOVEABLE, size_in_bytes);
            if h_mem.is_null() {
                CloseClipboard();
                return false;
            }

            let dest = GlobalLock(h_mem) as *mut u16;
            if dest.is_null() {
                GlobalFree(h_mem);
                CloseClipboard();
                return false;
            }

            ptr::copy_nonoverlapping(wide_text.as_ptr(), dest, wide_text.len());
            GlobalUnlock(h_mem);

            if SetClipboardData(CF_UNICODETEXT, h_mem).is_null() {
                GlobalFree(h_mem);
                CloseClipboard();
                return false;
            }

            CloseClipboard();
            true
        }
    }

    #[cfg(not(windows))]
    pub fn copy_text(&self, _text: &str) -> bool {
        // Fallback for non-windows to satisfy compiler if ever compiled on linux,
        // though requirement is specifically Windows MSVC without mocks.
        // We will just return false on non-windows to avoid mocks.
        false
    }

    #[cfg(windows)]
    pub fn paste_text(&self) -> Option<String> {
        unsafe {
            if IsClipboardFormatAvailable(CF_UNICODETEXT) == 0 {
                return None;
            }

            if OpenClipboard(ptr::null_mut()) == 0 {
                return None;
            }

            let h_mem = GetClipboardData(CF_UNICODETEXT);
            if h_mem.is_null() {
                CloseClipboard();
                return None;
            }

            let ptr = GlobalLock(h_mem) as *const u16;
            if ptr.is_null() {
                CloseClipboard();
                return None;
            }

            let mut len = 0;
            while *ptr.add(len) != 0 {
                len += 1;
            }

            let slice = std::slice::from_raw_parts(ptr, len);
            let string = String::from_utf16_lossy(slice);

            GlobalUnlock(h_mem);
            CloseClipboard();

            Some(string)
        }
    }

    #[cfg(not(windows))]
    pub fn paste_text(&self) -> Option<String> {
        None
    }
}

#[cfg(windows)]
type HANDLE = *mut std::ffi::c_void;
#[cfg(windows)]
type HWND = *mut std::ffi::c_void;
#[cfg(windows)]
type UINT = u32;
#[cfg(windows)]
type SIZE_T = usize;
#[cfg(windows)]
type BOOL = i32;
#[cfg(windows)]
type HGLOBAL = HANDLE;

#[cfg(windows)]
const CF_UNICODETEXT: UINT = 13;
#[cfg(windows)]
const GMEM_MOVEABLE: UINT = 0x0002;

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn OpenClipboard(hWndNewOwner: HWND) -> BOOL;
    fn CloseClipboard() -> BOOL;
    fn EmptyClipboard() -> BOOL;
    fn SetClipboardData(uFormat: UINT, hMem: HANDLE) -> HANDLE;
    fn GetClipboardData(uFormat: UINT) -> HANDLE;
    fn IsClipboardFormatAvailable(format: UINT) -> BOOL;
}

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn GlobalAlloc(uFlags: UINT, dwBytes: SIZE_T) -> HGLOBAL;
    fn GlobalLock(hMem: HGLOBAL) -> *mut std::ffi::c_void;
    fn GlobalUnlock(hMem: HGLOBAL) -> BOOL;
    fn GlobalFree(hMem: HGLOBAL) -> HGLOBAL;
}
