#pragma once

#include <cstdint>
#include <expected>
#include <functional>
#include <vector>
#include <unordered_map>
#include <memory>
#include <type_traits>

namespace Client::UI {

enum class WindowFlag : uint64_t {
    None       = 0,
    Show       = 1ULL << 0,
    Enabled    = 1ULL << 1,
    Modal      = 1ULL << 2,
    Dragable   = 1ULL << 3,
    TopMost    = 1ULL << 4,
    NotPick    = 1ULL << 5,
    Alpha      = 1ULL << 6,
    Hovered    = 1ULL << 7,
    Pressed    = 1ULL << 8,
    Focused    = 1ULL << 9,
    Restricted = 1ULL << 10,
    Collapsed  = 1ULL << 11,
    Animating  = 1ULL << 12,
    Locked     = 1ULL << 13,
    All        = ~0ULL
};

constexpr WindowFlag operator|(WindowFlag a, WindowFlag b) noexcept {
    return static_cast<WindowFlag>(static_cast<uint64_t>(a) | static_cast<uint64_t>(b));
}

constexpr WindowFlag operator&(WindowFlag a, WindowFlag b) noexcept {
    return static_cast<WindowFlag>(static_cast<uint64_t>(a) & static_cast<uint64_t>(b));
}

constexpr WindowFlag operator^(WindowFlag a, WindowFlag b) noexcept {
    return static_cast<WindowFlag>(static_cast<uint64_t>(a) ^ static_cast<uint64_t>(b));
}

constexpr WindowFlag operator~(WindowFlag a) noexcept {
    return static_cast<WindowFlag>(~static_cast<uint64_t>(a));
}

constexpr WindowFlag& operator|=(WindowFlag& a, WindowFlag b) noexcept {
    return a = a | b;
}

constexpr WindowFlag& operator&=(WindowFlag& a, WindowFlag b) noexcept {
    return a = a & b;
}

constexpr WindowFlag& operator^=(WindowFlag& a, WindowFlag b) noexcept {
    return a = a ^ b;
}

enum class WindowStateError {
    None,
    Locked,
    TransitionConstraintViolated,
    SnapshotNotFound,
    WindowNotFound,
    GlobalModalActive
};

template <typename T = void>
using StateResult = std::expected<T, WindowStateError>;

using WindowObserverCallback = std::function<void(WindowFlag oldFlags, WindowFlag newFlags)>;

struct WindowStateSnapshot {
    WindowFlag Flags;
    float Alpha;
    uint64_t Timestamp;
};

class UIWindowStateFlags {
public:
    UIWindowStateFlags();
    ~UIWindowStateFlags() = default;

    StateResult<void> SetFlag(WindowFlag flag) noexcept;
    StateResult<void> RemoveFlag(WindowFlag flag) noexcept;
    StateResult<void> ToggleFlag(WindowFlag flag) noexcept;
    StateResult<void> SetFlags(WindowFlag flags) noexcept;
    
    [[nodiscard]] bool HasFlag(WindowFlag flag) const noexcept;
    [[nodiscard]] WindowFlag GetFlags() const noexcept;
    
    [[nodiscard]] bool IsVisible() const noexcept;
    [[nodiscard]] bool IsEnabled() const noexcept;

    StateResult<void> SetAlpha(float alpha) noexcept;
    [[nodiscard]] float GetAlpha() const noexcept;

    void AddObserver(uint32_t id, WindowObserverCallback callback) noexcept;
    void RemoveObserver(uint32_t id) noexcept;

    void SaveSnapshot(uint32_t snapshotId, uint64_t timestamp) noexcept;
    StateResult<void> RestoreSnapshot(uint32_t snapshotId) noexcept;

private:
    StateResult<void> InternalUpdateFlags(WindowFlag newFlags) noexcept;
    bool ValidateTransition(WindowFlag currentFlags, WindowFlag newFlags) const noexcept;
    void NotifyObservers(WindowFlag oldFlags, WindowFlag newFlags) const noexcept;

    WindowFlag m_flags;
    float m_alpha;
    
    std::unordered_map<uint32_t, WindowObserverCallback> m_observers;
    std::unordered_map<uint32_t, WindowStateSnapshot> m_snapshots;
};

using WindowId = uint32_t;

class UIWindowStateManager {
public:
    UIWindowStateManager() = default;
    ~UIWindowStateManager() = default;

    void RegisterWindow(WindowId id) noexcept;
    void UnregisterWindow(WindowId id) noexcept;
    
    UIWindowStateFlags* GetWindowFlags(WindowId id) noexcept;
    const UIWindowStateFlags* GetWindowFlags(WindowId id) const noexcept;

    StateResult<void> PushGlobalModal(WindowId id) noexcept;
    StateResult<void> PopGlobalModal() noexcept;
    [[nodiscard]] WindowId GetActiveGlobalModal() const noexcept;
    
    void BulkHideAll() noexcept;
    void BulkShowAll() noexcept;
    
private:
    std::unordered_map<WindowId, std::unique_ptr<UIWindowStateFlags>> m_windows;
    std::vector<WindowId> m_modalStack;
    std::vector<WindowId> m_bulkHiddenWindows;
};

} // namespace Client::UI
