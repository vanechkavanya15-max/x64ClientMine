#pragma once

#include "UIRenderBatcher.h"
#include <string>
#include <string_view>
#include <vector>
#include <expected>
#include <variant>

namespace Client::UI {

enum class TooltipRenderError {
    InvalidBatcher,
    EmptyContent,
    BatcherOverflow,
    InvalidGeometry,
    InvalidGlyph
};

struct TooltipTextLine {
    std::string text;
    Color color;
};

struct TooltipSeparator {
    Color color;
};

using TooltipElement = std::variant<TooltipTextLine, TooltipSeparator>;

struct GlyphMetrics {
    float u, v, uw, vh;
    float width, height;
};

class IFontProvider {
public:
    virtual ~IFontProvider() = default;
    virtual std::expected<GlyphMetrics, TooltipRenderError> GetGlyphMetrics(char c) const = 0;
};

struct TooltipLayoutConfig {
    float x = 0.0f;
    float y = 0.0f;
    float padding = 5.0f;
    float lineSpacing = 2.0f;
    float separatorHeight = 4.0f;
    Color backgroundColor = {0, 0, 0, 200};
    TextureId fontTextureId = {1};
    const IFontProvider* fontProvider = nullptr;
};

class UITooltipRenderer {
public:
    UITooltipRenderer() = default;

    void AddTextLine(std::string_view text, Color color = {255, 255, 255, 255});
    void AddSeparator(Color color = {150, 150, 150, 255});
    void Clear();

    [[nodiscard]] bool IsEmpty() const;
    [[nodiscard]] const std::vector<TooltipElement>& GetElements() const;

    std::expected<void, TooltipRenderError> Render(UIRenderBatcher& batcher, const TooltipLayoutConfig& config) const;

private:
    std::vector<TooltipElement> m_elements;
};

} // namespace Client::UI
