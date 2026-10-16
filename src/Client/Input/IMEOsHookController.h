#pragma once
#include <cstdint>

namespace Client::Input {

class IMEOsHookController {
public:
    IMEOsHookController() = default;
    ~IMEOsHookController() = default;

    void EnableInputCapture(bool enable) noexcept { m_capturing = enable; }
    [[nodiscard]] bool IsCapturing() const noexcept { return m_capturing; }

    void SetReadingInformation(bool active) noexcept { m_reading = active; }
    [[nodiscard]] bool HasReadingInformation() const noexcept { return m_reading; }

private:
    bool m_capturing{true};
    bool m_reading{false};
};

} // namespace Client::Input
