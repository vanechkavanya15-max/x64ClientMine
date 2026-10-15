#pragma once

#include <vector>
#include <string>
#include <string_view>
#include <expected>
#include <optional>
#include <cstdint>

namespace Client::Text {

enum class HorizontalAlignment : uint8_t {
    Left = 0x01,
    Center = 0x02,
    Right = 0x03
};

enum class VerticalAlignment : uint8_t {
    Top = 0x10,
    Center = 0x20,
    Bottom = 0x30
};

enum class TextDirection : uint8_t {
    Auto = 0,
    LTR = 1,
    RTL = 2
};

struct TextLayoutGlyph {
    float x{0.0f};
    float y{0.0f};
    float width{0.0f};
    std::string_view content;
    uint32_t color{0xFFFFFFFF};
    std::optional<std::string> hyperlink;
    bool isVisible{true};
};

struct TextLayoutLine {
    float width{0.0f};
    std::vector<TextLayoutGlyph> glyphs;
};

struct TextLayoutHyperlinkBounds {
    float sx{0.0f};
    float ex{0.0f};
    float sy{0.0f};
    float ey{0.0f};
    std::string text;
};

struct TextLayoutResult {
    float width{0.0f};
    float height{0.0f};
    std::vector<TextLayoutLine> lines;
    std::vector<TextLayoutHyperlinkBounds> hyperlinks;
    bool isRtlComputed{false};
};

enum class LayoutError : uint8_t {
    InvalidInput,
    FontMetricsMissing
};

class IFontMetricsProvider {
public:
    virtual ~IFontMetricsProvider() = default;
    virtual float GetGlyphAdvance(char32_t codepoint) const = 0;
    virtual float GetLineHeight() const = 0;
};

struct LayoutConfig {
    HorizontalAlignment horizontalAlign{HorizontalAlignment::Left};
    VerticalAlignment verticalAlign{VerticalAlignment::Top};
    TextDirection textDirection{TextDirection::Auto};
    std::optional<float> limitWidth{std::nullopt};
    uint32_t defaultColor{0xFFFFFFFF};
    bool multiLine{false};
};

class TextLayoutEngine {
public:
    static std::expected<TextLayoutResult, LayoutError> BuildLayout(
        std::string_view text,
        const IFontMetricsProvider& fontMetrics,
        const LayoutConfig& config = LayoutConfig{}
    );
};

} // namespace Client::Text
