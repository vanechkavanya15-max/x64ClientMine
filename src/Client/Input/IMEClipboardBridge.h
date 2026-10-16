#pragma once
#include <string>

namespace Client::Input {

class IMEClipboardBridge {
public:
    IMEClipboardBridge() = default;
    ~IMEClipboardBridge() = default;

    void SetClipboardText(const std::string& text) { m_clipboard = SanitizeText(text); }
    [[nodiscard]] std::string GetClipboardText() const { return m_clipboard; }

    [[nodiscard]] static std::string SanitizeText(const std::string& input);

private:
    std::string m_clipboard{};
};

} // namespace Client::Input
