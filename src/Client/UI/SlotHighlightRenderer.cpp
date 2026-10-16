#include "SlotHighlightRenderer.h"

namespace Client::UI {

std::expected<void, SlotHighlightError> SlotHighlightRenderer::RenderHighlight(
    UIRenderBatcher& batcher,
    const GridConfig& config,
    uint8_t col,
    uint8_t row,
    uint8_t width,
    uint8_t height,
    Color color,
    std::optional<TextureId> texture
) const {
    if (width == 0 || height == 0) {
        return std::unexpected(SlotHighlightError::InvalidDimensions);
    }

    if (col >= config.gridCols || row >= config.gridRows || 
        col + width > config.gridCols || row + height > config.gridRows) {
        return std::unexpected(SlotHighlightError::OutOfBounds);
    }

    float x = config.startX + (col * config.slotWidth);
    float y = config.startY + (row * config.slotHeight);
    float w = width * config.slotWidth;
    float h = height * config.slotHeight;

    std::expected<void, UIBatchError> result;

    if (texture) {
        // Map full texture UVs
        result = batcher.DrawTexturedRect(x, y, w, h, 0.0f, 0.0f, 1.0f, 1.0f, *texture, color);
    } else {
        result = batcher.DrawRect(x, y, w, h, color);
    }

    if (!result.has_value()) {
        return std::unexpected(SlotHighlightError::BatchError);
    }

    return {};
}

} // namespace Client::UI
