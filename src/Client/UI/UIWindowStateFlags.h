#pragma once
#include <cstdint>

namespace Client::UI {

enum WindowFlags : uint32_t {
    FLAG_NONE    = 0,
    FLAG_SHOW    = 1 << 0,
    FLAG_ENABLED = 1 << 1,
    FLAG_MODAL   = 1 << 2,
    FLAG_DRAG    = 1 << 3
};

class UIWindowStateFlags {
public:
    UIWindowStateFlags() : m_flags(FLAG_SHOW | FLAG_ENABLED) {}

    void SetFlag(uint32_t flag) noexcept { m_flags |= flag; }
    void RemoveFlag(uint32_t flag) noexcept { m_flags &= ~flag; }
    [[nodiscard]] bool HasFlag(uint32_t flag) const noexcept { return (m_flags & flag) == flag; }
    [[nodiscard]] bool IsVisible() const noexcept { return HasFlag(FLAG_SHOW); }
    [[nodiscard]] bool IsEnabled() const noexcept { return HasFlag(FLAG_ENABLED); }

private:
    uint32_t m_flags;
};

} // namespace Client::UI
