#pragma once
#include <cstdint>
#include <unordered_map>
#include <vector>
#include <expected>
#include <optional>
#include <algorithm>

#ifdef CreateWindow
#undef CreateWindow
#endif
#ifdef DestroyWindow
#undef DestroyWindow
#endif

namespace Client::UI {

enum class LayoutError {
    WindowNotFound,
    WindowAlreadyExists,
    CircularDependencyDetected,
    InvalidParent,
    HierarchyError
};

enum class HorizontalAlign : uint8_t { Left = 0, Center = 1, Right = 2, Stretch = 3 };
enum class VerticalAlign : uint8_t { Top = 0, Center = 1, Bottom = 2, Stretch = 3 };

struct Rect2D {
    int32_t left{0};
    int32_t top{0};
    int32_t right{0};
    int32_t bottom{0};
    [[nodiscard]] int32_t Width() const noexcept { return right - left; }
    [[nodiscard]] int32_t Height() const noexcept { return bottom - top; }
    [[nodiscard]] bool Contains(int32_t x, int32_t y) const noexcept {
        return x >= left && x <= right && y >= top && y <= bottom;
    }
    
    // Helper to calculate intersection
    [[nodiscard]] std::optional<Rect2D> Intersect(const Rect2D& other) const noexcept {
        int32_t l = std::max(left, other.left);
        int32_t t = std::max(top, other.top);
        int32_t r = std::min(right, other.right);
        int32_t b = std::min(bottom, other.bottom);
        if (l < r && t < b) return Rect2D{l, t, r, b};
        return std::nullopt;
    }
};

struct Margin {
    int32_t left{0};
    int32_t top{0};
    int32_t right{0};
    int32_t bottom{0};
};

struct WindowLayoutData {
    uint32_t windowId{0};
    uint32_t parentId{0}; // 0 implies root
    std::vector<uint32_t> children;
    
    int32_t x{0};
    int32_t y{0};
    int32_t width{0};
    int32_t height{0};
    
    Margin margin{};
    HorizontalAlign hAlign{HorizontalAlign::Left};
    VerticalAlign vAlign{VerticalAlign::Top};
    
    bool scissorEnabled{false};
    Rect2D scissorRect{};
    
    int32_t zOrder{0};
};

class WindowManagerLayoutApiBridge {
public:
    WindowManagerLayoutApiBridge() = default;
    ~WindowManagerLayoutApiBridge() = default;

    static WindowManagerLayoutApiBridge& Instance();
    [[nodiscard]] bool HasWindow(uint32_t windowId) const noexcept { return m_layouts.contains(windowId); }
    void Clear() { m_layouts.clear(); }

    // Lifetime
    std::expected<void, LayoutError> CreateWindow(uint32_t windowId);
    std::expected<void, LayoutError> DestroyWindow(uint32_t windowId);

    // Geometry Setters
    std::expected<void, LayoutError> SetWindowPosition(uint32_t windowId, int32_t x, int32_t y);
    std::expected<void, LayoutError> SetWindowSize(uint32_t windowId, int32_t width, int32_t height);
    std::expected<void, LayoutError> SetHorizontalAlign(uint32_t windowId, HorizontalAlign align);
    std::expected<void, LayoutError> SetVerticalAlign(uint32_t windowId, VerticalAlign align);
    std::expected<void, LayoutError> SetMargin(uint32_t windowId, const Margin& margin);
    std::expected<void, LayoutError> EnableScissorRect(uint32_t windowId, bool enable);
    std::expected<void, LayoutError> SetScissorRect(uint32_t windowId, const Rect2D& rect);

    // Hierarchy
    std::expected<void, LayoutError> AttachWindow(uint32_t windowId, uint32_t parentId);
    std::expected<void, LayoutError> DetachWindow(uint32_t windowId);

    // Z-Order
    std::expected<void, LayoutError> SetZOrder(uint32_t windowId, int32_t zOrder);
    std::expected<void, LayoutError> BringToFront(uint32_t windowId);
    std::expected<void, LayoutError> SendToBack(uint32_t windowId);

    // Queries
    [[nodiscard]] std::expected<Rect2D, LayoutError> CalculateGlobalRect(uint32_t windowId, int32_t rootWidth, int32_t rootHeight) const;
    [[nodiscard]] std::expected<std::optional<Rect2D>, LayoutError> CalculateEffectiveScissorRect(uint32_t windowId, int32_t rootWidth, int32_t rootHeight) const;
    
    [[nodiscard]] std::vector<uint32_t> GetRenderOrder() const;
    [[nodiscard]] std::optional<uint32_t> HitTest(int32_t x, int32_t y, int32_t rootWidth, int32_t rootHeight) const;

private:
    std::unordered_map<uint32_t, WindowLayoutData> m_layouts;
    
    bool CheckCircularDependency(uint32_t windowId, uint32_t potentialParentId) const;
    void BuildRenderOrderRecursive(uint32_t windowId, std::vector<uint32_t>& order) const;
    void RemoveChildFromParent(uint32_t windowId, uint32_t parentId);
};

} // namespace Client::UI
