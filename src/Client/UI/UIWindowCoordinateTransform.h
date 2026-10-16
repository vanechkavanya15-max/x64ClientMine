#pragma once
#include <cstdint>

namespace Client::UI {

struct Point2D { int32_t x{0}; int32_t y{0}; };

class UIWindowCoordinateTransform {
public:
    UIWindowCoordinateTransform(int32_t x = 0, int32_t y = 0, int32_t w = 0, int32_t h = 0)
        : m_x(x), m_y(y), m_width(w), m_height(h) {}

    [[nodiscard]] Point2D LocalToGlobal(const Point2D& local, const Point2D& parentGlobal) const noexcept {
        return { parentGlobal.x + m_x + local.x, parentGlobal.y + m_y + local.y };
    }
    [[nodiscard]] bool ContainsPoint(int32_t localX, int32_t localY) const noexcept {
        return localX >= 0 && localX < m_width && localY >= 0 && localY < m_height;
    }

private:
    int32_t m_x{0};
    int32_t m_y{0};
    int32_t m_width{0};
    int32_t m_height{0};
};

} // namespace Client::UI
