#pragma once
#include <cstdint>
#include <optional>

namespace Client::UI {

struct AttachingIconData {
    uint32_t type{0};
    uint32_t slotNumber{0};
    uint32_t itemVnum{0};
    uint32_t count{0};
    bool isAttaching{false};
};

class WindowManagerInputApiBridge {
public:
    WindowManagerInputApiBridge() = default;
    ~WindowManagerInputApiBridge() = default;

    void SetMousePosition(int32_t x, int32_t y) noexcept { m_mouseX = x; m_mouseY = y; }
    [[nodiscard]] std::pair<int32_t, int32_t> GetMousePosition() const noexcept { return {m_mouseX, m_mouseY}; }

    void SetCapture(uint32_t windowId) noexcept { m_captureWindowId = windowId; }
    void ReleaseCapture() noexcept { m_captureWindowId.reset(); }
    [[nodiscard]] std::optional<uint32_t> GetCaptureWindow() const noexcept { return m_captureWindowId; }

    void SetFocus(uint32_t windowId) noexcept { m_focusWindowId = windowId; }
    void KillFocus() noexcept { m_focusWindowId.reset(); }
    [[nodiscard]] std::optional<uint32_t> GetFocusWindow() const noexcept { return m_focusWindowId; }

    void AttachIcon(uint32_t type, uint32_t slotNumber, uint32_t itemVnum, uint32_t count);
    void DeattachIcon() noexcept { m_attachingIcon.isAttaching = false; }
    [[nodiscard]] bool IsAttaching() const noexcept { return m_attachingIcon.isAttaching; }
    [[nodiscard]] AttachingIconData GetAttachingData() const noexcept { return m_attachingIcon; }

private:
    int32_t m_mouseX{0};
    int32_t m_mouseY{0};
    std::optional<uint32_t> m_captureWindowId;
    std::optional<uint32_t> m_focusWindowId;
    AttachingIconData m_attachingIcon{};
};

} // namespace Client::UI
