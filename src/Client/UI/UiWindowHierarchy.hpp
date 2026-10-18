#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>
#include <unordered_map>
#include <memory_resource>
#include <span>
#include <optional>
#include <functional>
#include <algorithm>
#include <ranges>

#include "../Core/StrongTypes.h"

namespace Client::UI {

struct WindowIdTag {};
using WindowId = Client::Core::StrongType<WindowIdTag, uint32_t, 0>;

enum class HorizontalAlign : uint8_t {
    Left = 0,
    Center = 1,
    Right = 2
};

enum class VerticalAlign : uint8_t {
    Top = 0,
    Center = 1,
    Bottom = 2
};

enum class WindowFlags : uint32_t {
    None        = 0,
    Movable     = 1 << 0,
    Limit       = 1 << 1,
    Snap        = 1 << 2,
    Dragable    = 1 << 3,
    Attach      = 1 << 4,
    RestrictX   = 1 << 5,
    RestrictY   = 1 << 6,
    NotCapture  = 1 << 7,
    Float       = 1 << 8,
    NotPick     = 1 << 9,
    IgnoreSize  = 1 << 10,
    Rtl         = 1 << 11,
    Modal       = 1 << 12,
    Focusable   = 1 << 13,
};

constexpr WindowFlags operator|(WindowFlags a, WindowFlags b) noexcept {
    return static_cast<WindowFlags>(static_cast<uint32_t>(a) | static_cast<uint32_t>(b));
}
constexpr WindowFlags operator&(WindowFlags a, WindowFlags b) noexcept {
    return static_cast<WindowFlags>(static_cast<uint32_t>(a) & static_cast<uint32_t>(b));
}
constexpr WindowFlags operator~(WindowFlags a) noexcept {
    return static_cast<WindowFlags>(~static_cast<uint32_t>(a));
}
constexpr bool HasFlag(WindowFlags value, WindowFlags flag) noexcept {
    return (static_cast<uint32_t>(value) & static_cast<uint32_t>(flag)) != 0;
}

struct UiRect {
    int32_t left{0};
    int32_t top{0};
    int32_t right{0};
    int32_t bottom{0};

    constexpr int32_t Width() const noexcept { return right - left; }
    constexpr int32_t Height() const noexcept { return bottom - top; }
    constexpr bool Contains(int32_t x, int32_t y) const noexcept {
        return x >= left && x <= right && y >= top && y <= bottom;
    }
    constexpr bool IsEmpty() const noexcept {
        return right <= left || bottom <= top;
    }
    constexpr auto operator<=>(const UiRect&) const = default;

    std::optional<UiRect> Intersect(const UiRect& other) const noexcept {
        UiRect res{
            (std::max)(left, other.left),
            (std::max)(top, other.top),
            (std::min)(right, other.right),
            (std::min)(bottom, other.bottom)
        };
        if (res.right <= res.left || res.bottom <= res.top) {
            return std::nullopt;
        }
        return res;
    }
};

struct WindowNode {
    WindowId id{0};
    std::pmr::string name;
    WindowId parentId{0};
    std::pmr::vector<WindowId> children;

    int32_t localX{0};
    int32_t localY{0};
    int32_t width{0};
    int32_t height{0};

    HorizontalAlign hAlign{HorizontalAlign::Left};
    VerticalAlign vAlign{VerticalAlign::Top};

    UiRect globalRect{0, 0, 0, 0};

    bool visible{true};
    bool enableScissor{false};
    WindowFlags flags{WindowFlags::None};
    int32_t zIndex{0};
    void* userData{nullptr};

    explicit WindowNode(std::pmr::memory_resource* mr = std::pmr::get_default_resource())
        : name(mr), children(mr) {}

    WindowNode(WindowId wId, std::string_view wName, WindowFlags wFlags, int32_t wZIndex, void* uData, std::pmr::memory_resource* mr)
        : id(wId), name(wName, mr), children(mr), flags(wFlags), zIndex(wZIndex), userData(uData) {}
};

class UiWindowHierarchy {
public:
    explicit UiWindowHierarchy(std::pmr::memory_resource* mr = std::pmr::get_default_resource());
    ~UiWindowHierarchy() = default;

    UiWindowHierarchy(const UiWindowHierarchy&) = delete;
    UiWindowHierarchy& operator=(const UiWindowHierarchy&) = delete;

    UiWindowHierarchy(UiWindowHierarchy&&) noexcept = default;
    UiWindowHierarchy& operator=(UiWindowHierarchy&&) noexcept = default;

    // Node Lifecycle
    WindowId CreateNode(std::string_view name, WindowFlags flags = WindowFlags::None, int32_t zIndex = 0, void* userData = nullptr);
    WindowId CreateChildNode(WindowId parentId, std::string_view name, WindowFlags flags = WindowFlags::None, int32_t zIndex = 0, void* userData = nullptr);
    bool DestroyNode(WindowId id, bool recursive = true);
    void Clear();

    [[nodiscard]] bool ContainsNode(WindowId id) const noexcept;
    [[nodiscard]] WindowNode* GetNode(WindowId id) noexcept;
    [[nodiscard]] const WindowNode* GetNode(WindowId id) const noexcept;
    [[nodiscard]] size_t GetNodeCount() const noexcept;

    // Parent-Child Relationships & Reparenting
    bool SetParent(WindowId childId, WindowId newParentId);
    bool DetachFromParent(WindowId childId);
    [[nodiscard]] WindowId GetParent(WindowId id) const noexcept;
    [[nodiscard]] WindowId GetRoot(WindowId id) const noexcept;
    [[nodiscard]] std::span<const WindowId> GetChildren(WindowId id) const noexcept;
    [[nodiscard]] size_t GetChildCount(WindowId id) const noexcept;
    [[nodiscard]] bool HasParent(WindowId id) const noexcept;
    [[nodiscard]] bool HasChildren(WindowId id) const noexcept;
    [[nodiscard]] bool IsChildOf(WindowId childId, WindowId potentialParentId) const noexcept;
    [[nodiscard]] bool IsDescendantOf(WindowId childId, WindowId ancestorId) const noexcept;
    [[nodiscard]] bool IsAncestorOf(WindowId ancestorId, WindowId childId) const noexcept;

    // Z-Order & Sorting
    bool BringToFront(WindowId id);
    bool SendToBack(WindowId id);
    bool SetZIndex(WindowId id, int32_t zIndex);
    void SortChildrenByZOrder(WindowId parentId);
    void SortAllChildrenByZOrder();

    // Layout, Positioning & Scissor Rects
    void SetPosition(WindowId id, int32_t x, int32_t y);
    void SetSize(WindowId id, int32_t width, int32_t height);
    void SetAlign(WindowId id, HorizontalAlign hAlign, VerticalAlign vAlign);
    void SetVisible(WindowId id, bool visible);
    void SetEnableScissor(WindowId id, bool enable);
    void SetFlags(WindowId id, WindowFlags flags);
    void AddFlags(WindowId id, WindowFlags flags);
    void RemoveFlags(WindowId id, WindowFlags flags);
    [[nodiscard]] bool HasFlag(WindowId id, WindowFlags flag) const noexcept;

    void UpdateRect(WindowId id);
    void UpdateAllRects();
    [[nodiscard]] std::optional<UiRect> ComputeActiveScissor(WindowId id) const noexcept;
    [[nodiscard]] bool IsEffectivelyVisible(WindowId id) const noexcept;

    // Hit-Testing / Picking
    [[nodiscard]] std::optional<WindowId> PickWindow(int32_t x, int32_t y, WindowId rootId = WindowId{0}) const noexcept;
    [[nodiscard]] std::optional<WindowId> PickTopWindow(int32_t x, int32_t y, WindowId rootId = WindowId{0}) const noexcept;

    // Focus Stack Management
    [[nodiscard]] WindowId GetActiveWindow() const noexcept;
    bool SetActiveWindow(WindowId id);
    void ClearActiveWindow();
    bool DeactivateWindow();
    [[nodiscard]] std::span<const WindowId> GetActiveWindowHistory() const noexcept;

    // Modal Lock Stack
    bool LockWindow(WindowId id);
    bool UnlockWindow();
    [[nodiscard]] WindowId GetLockWindow() const noexcept;
    [[nodiscard]] bool IsModalLocked() const noexcept;

    // Focus Traversal
    [[nodiscard]] std::optional<WindowId> GetNextFocusable(WindowId currentId) const noexcept;
    [[nodiscard]] std::optional<WindowId> GetPrevFocusable(WindowId currentId) const noexcept;

    // Iteration & Debugging
    void TraversePreOrder(WindowId rootId, const std::function<bool(const WindowNode&)>& visitor) const;
    void TraversePostOrder(WindowId rootId, const std::function<void(const WindowNode&)>& visitor) const;
    [[nodiscard]] std::string DumpTree(WindowId rootId = WindowId{0}) const;

    [[nodiscard]] std::pmr::memory_resource* GetMemoryResource() const noexcept { return m_mr; }

private:
    std::pmr::memory_resource* m_mr;
    uint32_t m_nextId{1};
    std::pmr::unordered_map<WindowId, WindowNode> m_nodes;

    WindowId m_activeWindow{0};
    std::pmr::vector<WindowId> m_activeHistory;

    WindowId m_lockWindow{0};
    std::pmr::vector<WindowId> m_lockStack;
};

} // namespace Client::UI
