#include "IMEClipboardBridge.h"
#include <stdexcept>
#include <algorithm>
#include <cstring>

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#endif

namespace Client::Input {

ClipboardResult<void> IMEClipboardBridge::SetClipboardText(std::string_view text, ClipboardEncoding encoding) {
    if (encoding == ClipboardEncoding::UTF8) {
        m_clipboard = SanitizeText(text);
    } else if (encoding == ClipboardEncoding::UTF16) {
        // Technically text is std::string_view, if we passed UTF16, we'd need a u16string_view or similar.
        // For simplicity, we assume text holds the raw bytes of UTF16.
        auto utf16_view = std::u16string_view(reinterpret_cast<const char16_t*>(text.data()), text.size() / 2);
        auto utf8_result = Utf16ToUtf8(utf16_view);
        if (!utf8_result) {
            return std::unexpected(utf8_result.error());
        }
        m_clipboard = SanitizeText(*utf8_result);
    } else if (encoding == ClipboardEncoding::ANSI) {
        auto utf8_result = AnsiToUtf8(text);
        if (!utf8_result) {
            return std::unexpected(utf8_result.error());
        }
        m_clipboard = SanitizeText(*utf8_result);
    } else {
        return std::unexpected(ClipboardError::InvalidEncoding);
    }
    
    // Reset selection if text changes to avoid out-of-bounds selection
    m_selection = {0, 0};
    return {};
}

ClipboardResult<std::string> IMEClipboardBridge::GetClipboardText(ClipboardEncoding targetEncoding) const {
    if (m_clipboard.empty()) {
        return std::unexpected(ClipboardError::EmptyClipboard);
    }

    if (targetEncoding == ClipboardEncoding::UTF8) {
        return m_clipboard;
    } else if (targetEncoding == ClipboardEncoding::UTF16) {
        auto utf16_result = Utf8ToUtf16(m_clipboard);
        if (!utf16_result) {
            return std::unexpected(utf16_result.error());
        }
        // Return raw bytes in a string
        return std::string(reinterpret_cast<const char*>(utf16_result->data()), utf16_result->size() * 2);
    } else if (targetEncoding == ClipboardEncoding::ANSI) {
        return Utf8ToAnsi(m_clipboard);
    }

    return std::unexpected(ClipboardError::InvalidEncoding);
}

std::string IMEClipboardBridge::SanitizeText(std::string_view input) {
    std::string out;
    out.reserve(input.size());

    auto it = input.begin();
    while (it != input.end()) {
        auto decode_result = DecodeUtf8Codepoint(it, input.end());
        if (decode_result) {
            char32_t codepoint = *decode_result;
            if (IsPrintable(codepoint) || codepoint == '\n' || codepoint == '\t') {
                EncodeUtf8Codepoint(codepoint, out);
            }
        } else {
            // Invalid UTF-8 sequence, skip 1 byte
            ++it;
        }
    }
    return out;
}

bool IMEClipboardBridge::IsPrintable(char32_t codepoint) noexcept {
    // Basic printable checks, ASCII printable is 32 to 126
    if (codepoint >= 32 && codepoint <= 126) return true;
    
    // Allow non-ascii codepoints as printable by default, unless they are C0/C1 control codes
    if (codepoint < 32) return false;
    if (codepoint >= 0x7F && codepoint <= 0x9F) return false;

    // Specifically filter out Metin2 color codes if needed?
    // We filter control characters here. 
    return true;
}

ClipboardResult<void> IMEClipboardBridge::SetSelection(size_t start, size_t end) {
    if (start > m_clipboard.size() || end > m_clipboard.size()) {
        return std::unexpected(ClipboardError::InvalidSelection);
    }
    m_selection = {start, end};
    return {};
}

ClipboardResult<TextSelection> IMEClipboardBridge::GetSelection() const {
    return m_selection;
}

ClipboardResult<std::string> IMEClipboardBridge::GetSelectedText() const {
    if (m_selection.IsEmpty()) {
        return std::string{};
    }
    size_t min_pos = m_selection.Min();
    size_t length = m_selection.Length();
    if (min_pos + length > m_clipboard.size()) {
         return std::unexpected(ClipboardError::InvalidSelection);
    }
    return m_clipboard.substr(min_pos, length);
}

ClipboardResult<void> IMEClipboardBridge::DeleteSelection() {
    if (m_selection.IsEmpty()) {
        return {};
    }
    size_t min_pos = m_selection.Min();
    size_t length = m_selection.Length();
    if (min_pos + length > m_clipboard.size()) {
         return std::unexpected(ClipboardError::InvalidSelection);
    }
    
    m_clipboard.erase(min_pos, length);
    m_selection = {min_pos, min_pos};
    return {};
}

ClipboardResult<void> IMEClipboardBridge::ReplaceSelection(std::string_view replacementText) {
    auto sanitize_str = SanitizeText(replacementText);
    
    size_t min_pos = m_selection.Min();
    size_t length = m_selection.Length();
    if (min_pos + length > m_clipboard.size()) {
         return std::unexpected(ClipboardError::InvalidSelection);
    }

    m_clipboard.replace(min_pos, length, sanitize_str);
    size_t new_pos = min_pos + sanitize_str.size();
    m_selection = {new_pos, new_pos};
    return {};
}

ClipboardResult<void> IMEClipboardBridge::SelectAll() {
    m_selection = {0, m_clipboard.size()};
    return {};
}

ClipboardResult<std::u16string> IMEClipboardBridge::Utf8ToUtf16(std::string_view utf8) {
    try {
        std::u16string out;
        out.reserve(utf8.size());
        auto it = utf8.begin();
        while (it != utf8.end()) {
            auto decode_result = DecodeUtf8Codepoint(it, utf8.end());
            if (!decode_result) {
                 return std::unexpected(ClipboardError::ConversionFailed);
            }
            char32_t cp = *decode_result;
            if (cp <= 0xFFFF) {
                out.push_back(static_cast<char16_t>(cp));
            } else {
                cp -= 0x10000;
                out.push_back(static_cast<char16_t>((cp >> 10) + 0xD800));
                out.push_back(static_cast<char16_t>((cp & 0x3FF) + 0xDC00));
            }
        }
        return out;
    } catch (const std::bad_alloc&) {
        return std::unexpected(ClipboardError::OutOfMemory);
    }
}

ClipboardResult<std::string> IMEClipboardBridge::Utf16ToUtf8(std::u16string_view utf16) {
    try {
        std::string out;
        out.reserve(utf16.size() * 3);
        for (size_t i = 0; i < utf16.size(); ++i) {
            char16_t c1 = utf16[i];
            char32_t cp = c1;
            if (c1 >= 0xD800 && c1 <= 0xDBFF) {
                if (i + 1 < utf16.size()) {
                    char16_t c2 = utf16[i + 1];
                    if (c2 >= 0xDC00 && c2 <= 0xDFFF) {
                        cp = ((c1 - 0xD800) << 10) + (c2 - 0xDC00) + 0x10000;
                        ++i;
                    } else {
                        return std::unexpected(ClipboardError::ConversionFailed);
                    }
                } else {
                    return std::unexpected(ClipboardError::ConversionFailed);
                }
            } else if (c1 >= 0xDC00 && c1 <= 0xDFFF) {
                 return std::unexpected(ClipboardError::ConversionFailed);
            }
            EncodeUtf8Codepoint(cp, out);
        }
        return out;
    } catch (const std::bad_alloc&) {
        return std::unexpected(ClipboardError::OutOfMemory);
    }
}

ClipboardResult<std::string> IMEClipboardBridge::AnsiToUtf8(std::string_view ansi) {
    try {
        std::string out;
        out.reserve(ansi.size() * 2);
        for (char c : ansi) {
            // Simplified ANSI (ISO-8859-1) to UTF-8
            unsigned char uc = static_cast<unsigned char>(c);
            if (uc < 128) {
                out.push_back(c);
            } else {
                out.push_back(static_cast<char>(0xC0 | (uc >> 6)));
                out.push_back(static_cast<char>(0x80 | (uc & 0x3F)));
            }
        }
        return out;
    } catch (const std::bad_alloc&) {
        return std::unexpected(ClipboardError::OutOfMemory);
    }
}

ClipboardResult<std::string> IMEClipboardBridge::Utf8ToAnsi(std::string_view utf8) {
    try {
        std::string out;
        out.reserve(utf8.size());
        auto it = utf8.begin();
        while (it != utf8.end()) {
            auto decode_result = DecodeUtf8Codepoint(it, utf8.end());
            if (!decode_result) {
                return std::unexpected(ClipboardError::ConversionFailed);
            }
            char32_t cp = *decode_result;
            if (cp < 256) {
                out.push_back(static_cast<char>(cp));
            } else {
                out.push_back('?'); // unmappable character
            }
        }
        return out;
    } catch (const std::bad_alloc&) {
        return std::unexpected(ClipboardError::OutOfMemory);
    }
}

void IMEClipboardBridge::Clear() noexcept {
    m_clipboard.clear();
    m_selection = {0, 0};
}

ClipboardResult<char32_t> IMEClipboardBridge::DecodeUtf8Codepoint(std::string_view::const_iterator& it, std::string_view::const_iterator end) {
    if (it == end) return std::unexpected(ClipboardError::ConversionFailed);

    unsigned char c = static_cast<unsigned char>(*it);
    int extraBytes = 0;
    char32_t codepoint = 0;
    char32_t minCodepoint = 0;

    if (c < 0x80) {
        ++it;
        return c;
    } else if ((c & 0xE0) == 0xC0) {
        extraBytes = 1;
        codepoint = c & 0x1F;
        minCodepoint = 0x80;
    } else if ((c & 0xF0) == 0xE0) {
        extraBytes = 2;
        codepoint = c & 0x0F;
        minCodepoint = 0x800;
    } else if ((c & 0xF8) == 0xF0) {
        extraBytes = 3;
        codepoint = c & 0x07;
        minCodepoint = 0x10000;
    } else {
        return std::unexpected(ClipboardError::ConversionFailed);
    }

    ++it;
    for (int i = 0; i < extraBytes; ++i) {
        if (it == end) return std::unexpected(ClipboardError::ConversionFailed);
        unsigned char nextC = static_cast<unsigned char>(*it);
        if ((nextC & 0xC0) != 0x80) return std::unexpected(ClipboardError::ConversionFailed);
        codepoint = (codepoint << 6) | (nextC & 0x3F);
        ++it;
    }

    if (codepoint < minCodepoint || (codepoint >= 0xD800 && codepoint <= 0xDFFF) || codepoint > 0x10FFFF) {
        return std::unexpected(ClipboardError::ConversionFailed);
    }

    return codepoint;
}

void IMEClipboardBridge::EncodeUtf8Codepoint(char32_t codepoint, std::string& out) {
    if (codepoint < 0x80) {
        out.push_back(static_cast<char>(codepoint));
    } else if (codepoint < 0x800) {
        out.push_back(static_cast<char>(0xC0 | (codepoint >> 6)));
        out.push_back(static_cast<char>(0x80 | (codepoint & 0x3F)));
    } else if (codepoint < 0x10000) {
        out.push_back(static_cast<char>(0xE0 | (codepoint >> 12)));
        out.push_back(static_cast<char>(0x80 | ((codepoint >> 6) & 0x3F)));
        out.push_back(static_cast<char>(0x80 | (codepoint & 0x3F)));
    } else {
        out.push_back(static_cast<char>(0xF0 | (codepoint >> 18)));
        out.push_back(static_cast<char>(0x80 | ((codepoint >> 12) & 0x3F)));
        out.push_back(static_cast<char>(0x80 | ((codepoint >> 6) & 0x3F)));
        out.push_back(static_cast<char>(0x80 | (codepoint & 0x3F)));
    }
}

bool IMEClipboardBridge::IsValidAnsi(char /*c*/) noexcept {
    // Any byte is valid in ISO-8859-1.
    return true;
}

ClipboardResult<void> IMEClipboardBridge::SetClipboardTextWide(std::wstring_view text) {
    std::u16string u16(reinterpret_cast<const char16_t*>(text.data()), text.size());
    auto u8res = Utf16ToUtf8(u16);
    if (!u8res) {
        return std::unexpected(u8res.error());
    }
    return SetClipboardText(*u8res, ClipboardEncoding::UTF8);
}

ClipboardResult<std::wstring> IMEClipboardBridge::GetClipboardTextWide() const {
    if (m_clipboard.empty()) {
        return std::unexpected(ClipboardError::EmptyClipboard);
    }
    auto u16res = Utf8ToUtf16(m_clipboard);
    if (!u16res) {
        return std::unexpected(u16res.error());
    }
    return std::wstring(reinterpret_cast<const wchar_t*>(u16res->data()), u16res->size());
}

std::wstring IMEClipboardBridge::SanitizeTextWide(std::wstring_view input) {
    std::wstring out;
    out.reserve(input.size());
    for (wchar_t c : input) {
        char32_t cp = static_cast<char32_t>(c);
        if (IsPrintable(cp) || c == L'\n' || c == L'\t') {
            out.push_back(c);
        }
    }
    return out;
}

ClipboardResult<void> IMEClipboardBridge::CopyToWindowsClipboard(void* hWnd, std::wstring_view text) {
#ifdef _WIN32
    if (text.empty()) {
        return {};
    }
    HWND hWindow = static_cast<HWND>(hWnd);
    if (!OpenClipboard(hWindow)) {
        return std::unexpected(ClipboardError::EmptyClipboard);
    }
    EmptyClipboard();
    const SIZE_T bytes = (text.size() + 1) * sizeof(wchar_t);
    HGLOBAL hMem = GlobalAlloc(GMEM_MOVEABLE, bytes);
    if (!hMem) {
        CloseClipboard();
        return std::unexpected(ClipboardError::OutOfMemory);
    }
    wchar_t* dst = static_cast<wchar_t*>(GlobalLock(hMem));
    if (!dst) {
        GlobalFree(hMem);
        CloseClipboard();
        return std::unexpected(ClipboardError::OutOfMemory);
    }
    std::memcpy(dst, text.data(), text.size() * sizeof(wchar_t));
    dst[text.size()] = L'\0';
    GlobalUnlock(hMem);

    if (!SetClipboardData(CF_UNICODETEXT, hMem)) {
        GlobalFree(hMem);
    }
    CloseClipboard();
    return {};
#else
    (void)hWnd;
    (void)text;
    return std::unexpected(ClipboardError::ConversionFailed);
#endif
}

ClipboardResult<std::wstring> IMEClipboardBridge::GetFromWindowsClipboard(void* hWnd) {
#ifdef _WIN32
    HWND hWindow = static_cast<HWND>(hWnd);
    if (!OpenClipboard(hWindow)) {
        return std::unexpected(ClipboardError::EmptyClipboard);
    }
    HANDLE hUni = GetClipboardData(CF_UNICODETEXT);
    if (!hUni) {
        CloseClipboard();
        return std::unexpected(ClipboardError::EmptyClipboard);
    }
    const wchar_t* wbuf = static_cast<const wchar_t*>(GlobalLock(hUni));
    if (!wbuf) {
        CloseClipboard();
        return std::unexpected(ClipboardError::EmptyClipboard);
    }
    std::wstring raw(wbuf);
    GlobalUnlock(hUni);
    CloseClipboard();

    return SanitizeTextWide(raw);
#else
    (void)hWnd;
    return std::unexpected(ClipboardError::EmptyClipboard);
#endif
}

} // namespace Client::Input
