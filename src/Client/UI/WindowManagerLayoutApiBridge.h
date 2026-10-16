#pragma once
#include <cstdint>
#include <unordered_map>

namespace Client::UI {

enum class HorizontalAlign : uint8_t { Left = 0, Center = 1, Right = 2 };
enum class VerticalAlign : uint8_t { Top = 0, Center = 1, Bottom = 2 };

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
};

struct WindowLayoutData {
    uint32_t windowId{0};
    int32_t x{0};
    int32_t y{0};
    int32_t width{0};
    int32_t height{0};
    HorizontalAlign hAlign{HorizontalAlign::Left};
    VerticalAlign vAlign{VerticalAlign::Top};
    bool scissorEnabled{false};
    Rect2D scissorRect{};
};

class WindowManagerLayoutApiBridge {
public:
    WindowManagerLayoutApiBridge() = default;
    ~WindowManagerLayoutApiBridge() = default;

    bool SetWindowPosition(uint32_t windowId, int32_t x, int32_t y);
    bool SetWindowSize(uint32_t windowId, int32_t width, int32_t height);
    bool SetHorizontalAlign(uint32_t windowId, HorizontalAlign align);
    bool SetVerticalAlign(uint32_t windowId, VerticalAlign align);
    bool EnableScissorRect(uint32_t windowId, bool enable);
    bool SetScissorRect(uint32_t windowId, const Rect2D& rect);
    [[nodiscard]] Rect2D CalculateGlobalRect(uint32_t windowId, int32_t parentWidth, int32_t parentHeight) const;

private:
    std::unordered_map<uint32_t, WindowLayoutData> m_layouts;
};

} // namespace Client::UI
