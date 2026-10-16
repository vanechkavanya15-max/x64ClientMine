#pragma once

#include <cstdint>
#include <algorithm>
#include <optional>

namespace Client::UI {

struct Point2D {
    int32_t x{0};
    int32_t y{0};

    [[nodiscard]] bool operator==(const Point2D&) const = default;
};

struct Size2D {
    int32_t width{0};
    int32_t height{0};

    [[nodiscard]] bool operator==(const Size2D&) const = default;
};

struct Rect2D {
    int32_t left{0};
    int32_t top{0};
    int32_t right{0};
    int32_t bottom{0};

    [[nodiscard]] int32_t GetWidth() const noexcept { return right - left; }
    [[nodiscard]] int32_t GetHeight() const noexcept { return bottom - top; }

    [[nodiscard]] bool Contains(int32_t x, int32_t y) const noexcept {
        return x >= left && x < right && y >= top && y < bottom;
    }

    [[nodiscard]] bool Contains(const Point2D& pt) const noexcept {
        return Contains(pt.x, pt.y);
    }

    [[nodiscard]] Rect2D Intersect(const Rect2D& other) const noexcept {
        return {
            std::max(left, other.left),
            std::max(top, other.top),
            std::min(right, other.right),
            std::min(bottom, other.bottom)
        };
    }

    [[nodiscard]] bool IsValid() const noexcept {
        return left < right && top < bottom;
    }

    [[nodiscard]] bool operator==(const Rect2D&) const = default;
};

struct Scale2D {
    float x{1.0f};
    float y{1.0f};

    [[nodiscard]] bool operator==(const Scale2D&) const = default;
};

struct Padding2D {
    int32_t left{0};
    int32_t top{0};
    int32_t right{0};
    int32_t bottom{0};
    
    [[nodiscard]] bool operator==(const Padding2D&) const = default;
};

enum class AnchorType : uint8_t {
    TopLeft = 0,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    StretchAll,
    StretchHorizontal, // Stretches width, aligns to top (can add Top/Middle/Bottom variants if needed, keeping it simple as Top for now or requiring margins)
    StretchVertical    // Stretches height, aligns to left
};

struct TransformContext {
    Rect2D parentGlobalRect{0, 0, 0, 0};
    Rect2D parentScissorRect{0, 0, 0, 0}; // Can be same as global if no clipping, or zero/invalid if disabled
    Scale2D globalDpiScale{1.0f, 1.0f};
    bool enableScissorClip{true};
};

struct TransformState {
    Rect2D globalRect{0, 0, 0, 0};
    Rect2D localRect{0, 0, 0, 0};      // Base rect in local space (unscaled)
    Rect2D scissorRect{0, 0, 0, 0};    // Final clipped rect in global space
    Point2D anchorOffset{0, 0};        // Calculated offset from parent
    Scale2D currentScale{1.0f, 1.0f};  // Inherited + local scale
    bool isVisible{true};              // False if completely scissored out
};

class UIWindowCoordinateTransform {
public:
    UIWindowCoordinateTransform() = default;
    UIWindowCoordinateTransform(int32_t x, int32_t y, int32_t w, int32_t h)
        : m_baseLocalRect{x, y, x + w, y + h} {}

    // Configuration
    void SetLocalRect(const Rect2D& rect) noexcept { m_baseLocalRect = rect; }
    void SetLocalRect(int32_t x, int32_t y, int32_t w, int32_t h) noexcept { 
        m_baseLocalRect = {x, y, x + w, y + h}; 
    }
    
    [[nodiscard]] Rect2D GetLocalRect() const noexcept { return m_baseLocalRect; }
    
    void SetAnchorType(AnchorType type) noexcept { m_anchorType = type; }
    [[nodiscard]] AnchorType GetAnchorType() const noexcept { return m_anchorType; }

    void SetMargins(const Padding2D& margins) noexcept { m_margins = margins; }
    [[nodiscard]] Padding2D GetMargins() const noexcept { return m_margins; }

    void SetPadding(const Padding2D& padding) noexcept { m_padding = padding; }
    [[nodiscard]] Padding2D GetPadding() const noexcept { return m_padding; }

    void SetLocalScale(const Scale2D& scale) noexcept { m_localScale = scale; }
    [[nodiscard]] Scale2D GetLocalScale() const noexcept { return m_localScale; }

    // Processing
    [[nodiscard]] TransformState UpdateTransform(const TransformContext& parentCtx) const noexcept;
    
    // Generates the context for children of this window
    [[nodiscard]] TransformContext GetContentContext(const TransformState& currentState) const noexcept;

    // Coordinate conversions (using a pre-calculated state)
    [[nodiscard]] Point2D LocalToGlobal(const Point2D& local, const TransformState& state) const noexcept;
    [[nodiscard]] Point2D GlobalToLocal(const Point2D& global, const TransformState& state) const noexcept;
    
    [[nodiscard]] bool ContainsGlobalPoint(const Point2D& global, const TransformState& state) const noexcept;

private:
    Rect2D m_baseLocalRect{0, 0, 0, 0};
    AnchorType m_anchorType{AnchorType::TopLeft};
    Padding2D m_margins{0, 0, 0, 0};
    Padding2D m_padding{0, 0, 0, 0};
    Scale2D m_localScale{1.0f, 1.0f};

    [[nodiscard]] Rect2D CalculateAnchoredRect(const TransformContext& parentCtx, const Scale2D& finalScale) const noexcept;
};

} // namespace Client::UI
