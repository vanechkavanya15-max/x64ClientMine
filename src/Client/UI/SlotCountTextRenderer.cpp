#include "SlotCountTextRenderer.h"
#include <string>

namespace Client::UI {

SlotCountTextRenderer::SlotCountTextRenderer(const IFontProvider* fontProvider)
    : m_fontProvider(fontProvider) 
{
}

std::expected<void, SlotCountRenderError> SlotCountTextRenderer::RenderText(UIRenderBatcher& batcher, std::string_view text, const SlotCountLayoutConfig& config) const {
    if (!m_fontProvider) {
        return std::unexpected(SlotCountRenderError::InvalidFontProvider);
    }

    float currentX = config.x;

    for (char c : text) {
        auto glyphRes = m_fontProvider->GetGlyphMetrics(c);
        if (!glyphRes) {
            return std::unexpected(SlotCountRenderError::InvalidGlyph);
        }

        auto res = batcher.DrawTexturedRect(
            currentX, config.y, 
            glyphRes->width, glyphRes->height, 
            glyphRes->u, glyphRes->v, 
            glyphRes->uw, glyphRes->vh, 
            config.fontTextureId, config.color
        );

        if (!res) {
            if (res.error() == UIBatchError::BufferOverflow) {
                return std::unexpected(SlotCountRenderError::BatcherOverflow);
            } else {
                return std::unexpected(SlotCountRenderError::InvalidGeometry);
            }
        }

        currentX += glyphRes->width;
    }

    return {};
}

std::expected<void, SlotCountRenderError> SlotCountTextRenderer::RenderCount(UIRenderBatcher& batcher, uint32_t count, const SlotCountLayoutConfig& config) const {
    std::string countStr = std::to_string(count);
    return RenderText(batcher, countStr, config);
}

} // namespace Client::UI
