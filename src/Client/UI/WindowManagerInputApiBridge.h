#pragma once
#include <cstdint>
#include <optional>
#include <vector>
#include <expected>
#include <chrono>
#include <cmath>
#include <algorithm>

namespace Client::UI {

enum class MouseButton : uint8_t {
    Left = 0,
    Right = 1,
    Middle = 2,
    XButton1 = 3,
    XButton2 = 4,
    Unknown = 255
};

enum class ButtonState : uint8_t {
    Released = 0,
    Pressed = 1
};

enum class KeyModifier : uint8_t {
    None = 0,
    Shift = 1 << 0,
    Ctrl = 1 << 1,
    Alt = 1 << 2
};

enum class CursorType : uint8_t {
    Normal = 0,
    Attack = 1,
    Talk = 2,
    Pick = 3,
    Camera = 4,
    No = 5,
    Skill = 6,
    Chair = 7,
    Count = 8
};

enum class InputError : uint8_t {
    Success = 0,
    InvalidWindow = 1,
    AlreadyCaptured = 2,
    NotCaptured = 3,
    InputLocked = 4,
    OutsideRestriction = 5,
    IconNotAttached = 6
};

struct Point2D {
    int32_t x{0};
    int32_t y{0};

    bool operator==(const Point2D& other) const noexcept = default;
};

struct Rect2D {
    int32_t x{0};
    int32_t y{0};
    int32_t width{0};
    int32_t height{0};

    [[nodiscard]] bool Contains(const Point2D& point) const noexcept {
        return point.x >= x && point.x < x + width &&
               point.y >= y && point.y < y + height;
    }
    bool operator==(const Rect2D& other) const noexcept = default;
};

struct AttachingIconData {
    uint32_t type{0};
    uint32_t slotNumber{0};
    uint32_t itemVnum{0};
    uint32_t count{0};
    std::optional<uint32_t> sourceWindowId;
    bool isAttaching{false};
};

struct DragContext {
    bool isDragging{false};
    Point2D startPosition{};
    uint32_t sourceWindowId{0};
    MouseButton button{MouseButton::Unknown};
};

class WindowManagerInputApiBridge {
public:
    WindowManagerInputApiBridge() = default;
    ~WindowManagerInputApiBridge() = default;

    // Mouse Position
    void SetMousePosition(int32_t x, int32_t y) noexcept { m_mousePos = {x, y}; }
    [[nodiscard]] Point2D GetMousePosition() const noexcept { return m_mousePos; }

    // Window Capture
    std::expected<void, InputError> SetCapture(uint32_t windowId) noexcept;
    std::expected<void, InputError> ReleaseCapture() noexcept;
    [[nodiscard]] std::optional<uint32_t> GetCaptureWindow() const noexcept { return m_captureWindowId; }

    // Window Focus
    void SetFocus(uint32_t windowId) noexcept;
    void KillFocus() noexcept;
    void KillFocus(uint32_t windowId) noexcept;
    [[nodiscard]] std::optional<uint32_t> GetFocusWindow() const noexcept;
    [[nodiscard]] std::vector<uint32_t> GetFocusStack() const noexcept { return m_focusStack; }

    // Icon Attaching
    void AttachIcon(uint32_t type, uint32_t slotNumber, uint32_t itemVnum, uint32_t count, std::optional<uint32_t> sourceWindowId = std::nullopt);
    void DeattachIcon() noexcept;
    [[nodiscard]] bool IsAttaching() const noexcept { return m_attachingIcon.isAttaching; }
    [[nodiscard]] AttachingIconData GetAttachingData() const noexcept { return m_attachingIcon; }

    // Drag and Drop
    void BeginDrag(uint32_t windowId, MouseButton button) noexcept;
    void EndDrag() noexcept;
    [[nodiscard]] const DragContext& GetDragContext() const noexcept { return m_dragContext; }
    [[nodiscard]] double CalculateDragDistance() const noexcept;

    // Clicks & Input Lock
    std::expected<void, InputError> OnButtonDown(uint32_t windowId, MouseButton button, uint64_t timestampMs) noexcept;
    std::expected<bool, InputError> OnButtonUp(uint32_t windowId, MouseButton button, uint64_t timestampMs) noexcept; // returns true if double click
    void LockInput(uint64_t durationMs, uint64_t currentTimestampMs) noexcept;
    [[nodiscard]] bool IsInputLocked(uint64_t currentTimestampMs) const noexcept;

    // Coordinate Restrictions
    void SetInputRestrictionRect(const Rect2D& rect) noexcept { m_restrictionRect = rect; }
    void ClearInputRestrictionRect() noexcept { m_restrictionRect.reset(); }
    [[nodiscard]] std::expected<void, InputError> CheckInputAllowed(const Point2D& point) const noexcept;

    // Cleanup on window destruction
    void OnWindowDestroyed(uint32_t windowId) noexcept;

private:
    Point2D m_mousePos{};
    std::optional<uint32_t> m_captureWindowId;
    std::vector<uint32_t> m_focusStack;
    AttachingIconData m_attachingIcon{};
    DragContext m_dragContext{};

    // Click tracking
    struct ClickData {
        uint64_t lastClickTime{0};
        uint32_t clickCount{0};
        uint32_t windowId{0};
    };
    ClickData m_leftClickData{};
    ClickData m_rightClickData{};

    // Restrictions
    std::optional<Rect2D> m_restrictionRect;
    uint64_t m_inputLockEndTime{0};

    // Constants
    static constexpr uint64_t DOUBLE_CLICK_THRESHOLD_MS = 500;
};

} // namespace Client::UI
