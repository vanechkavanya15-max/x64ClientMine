#pragma once

#include <string_view>
#include <cstdint>
#include <expected>
#include "UIRenderBatcher.h"
#include "UITooltipRenderer.h" // For IFontProvider and GlyphMetrics

namespace Client::UI {

enum class SlotCountRenderError {
    InvalidFontProvider,
    InvalidGlyph,
    BatcherOverflow,
    InvalidGeometry
};

struct SlotCountLayoutConfig {
    float x = 0.0f;
    float y = 0.0f;
    TextureId fontTextureId = {1};
    Color color = {255, 255, 255, 255};
};

class SlotCountTextRenderer {
public:
    explicit SlotCountTextRenderer(const IFontProvider* fontProvider);

    std::expected<void, SlotCountRenderError> RenderCount(UIRenderBatcher& batcher, uint32_t count, const SlotCountLayoutConfig& config) const;
    std::expected<void, SlotCountRenderError> RenderText(UIRenderBatcher& batcher, std::string_view text, const SlotCountLayoutConfig& config) const;

private:
    const IFontProvider* m_fontProvider;
};

} // namespace Client::UI
