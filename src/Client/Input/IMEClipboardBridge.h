#pragma once

#include <string>
#include <string_view>
#include <expected>
#include <cstdint>
#include <vector>

namespace Client::Input {

enum class ClipboardEncoding {
    UTF8,
    UTF16,
    ANSI
};

enum class ClipboardError {
    None,
    InvalidEncoding,
    ConversionFailed,
    OutOfMemory,
    InvalidSelection,
    EmptyClipboard
};

template <typename T>
using ClipboardResult = std::expected<T, ClipboardError>;

struct TextSelection {
    size_t start{0};
    size_t end{0};

    [[nodiscard]] constexpr size_t Length() const noexcept {
        return (end > start) ? (end - start) : (start > end ? start - end : 0);
    }

    [[nodiscard]] constexpr bool IsEmpty() const noexcept {
        return start == end;
    }

    [[nodiscard]] constexpr size_t Min() const noexcept {
        return start < end ? start : end;
    }

    [[nodiscard]] constexpr size_t Max() const noexcept {
        return start > end ? start : end;
    }
};

class IMEClipboardBridge {
public:
    IMEClipboardBridge() = default;
    ~IMEClipboardBridge() = default;

    IMEClipboardBridge(const IMEClipboardBridge&) = default;
    IMEClipboardBridge& operator=(const IMEClipboardBridge&) = default;
    IMEClipboardBridge(IMEClipboardBridge&&) noexcept = default;
    IMEClipboardBridge& operator=(IMEClipboardBridge&&) noexcept = default;

    // Clipboard Content Management
    ClipboardResult<void> SetClipboardText(std::string_view text, ClipboardEncoding encoding = ClipboardEncoding::UTF8);
    [[nodiscard]] ClipboardResult<std::string> GetClipboardText(ClipboardEncoding targetEncoding = ClipboardEncoding::UTF8) const;

    // Text Sanitization and Validation
    [[nodiscard]] static std::string SanitizeText(std::string_view input);
    [[nodiscard]] static bool IsPrintable(char32_t codepoint) noexcept;

    // Selection Management
    ClipboardResult<void> SetSelection(size_t start, size_t end);
    [[nodiscard]] ClipboardResult<TextSelection> GetSelection() const;
    [[nodiscard]] ClipboardResult<std::string> GetSelectedText() const;
    ClipboardResult<void> DeleteSelection();
    ClipboardResult<void> ReplaceSelection(std::string_view replacementText);
    ClipboardResult<void> SelectAll();

    // Encoding Conversions
    [[nodiscard]] static ClipboardResult<std::u16string> Utf8ToUtf16(std::string_view utf8);
    [[nodiscard]] static ClipboardResult<std::string> Utf16ToUtf8(std::u16string_view utf16);
    [[nodiscard]] static ClipboardResult<std::string> AnsiToUtf8(std::string_view ansi);
    [[nodiscard]] static ClipboardResult<std::string> Utf8ToAnsi(std::string_view utf8);

    // Advanced Text Operations
    [[nodiscard]] size_t GetTextLength() const noexcept { return m_clipboard.length(); }
    void Clear() noexcept;

private:
    std::string m_clipboard{};
    TextSelection m_selection{};

    [[nodiscard]] static ClipboardResult<char32_t> DecodeUtf8Codepoint(std::string_view::const_iterator& it, std::string_view::const_iterator end);
    static void EncodeUtf8Codepoint(char32_t codepoint, std::string& out);
    [[nodiscard]] static bool IsValidAnsi(char c) noexcept;
};

} // namespace Client::Input
