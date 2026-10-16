#pragma once

#include "UIRenderBatcher.h"
#include <cstdint>
#include <expected>
#include <string_view>
#include <format>
#include <optional>

namespace Client::UI {

enum class SlotHighlightError : uint8_t {
    InvalidBatcher,
    InvalidDimensions,
    OutOfBounds,
    BatchError
};

[[nodiscard]] constexpr std::string_view to_string(SlotHighlightError error) noexcept {
    switch (error) {
        case SlotHighlightError::InvalidBatcher: return "InvalidBatcher";
        case SlotHighlightError::InvalidDimensions: return "InvalidDimensions";
        case SlotHighlightError::OutOfBounds: return "OutOfBounds";
        case SlotHighlightError::BatchError: return "BatchError";
        default: return "Unknown";
    }
}

struct GridConfig {
    float startX = 0.0f;
    float startY = 0.0f;
    float slotWidth = 32.0f;
    float slotHeight = 32.0f;
    uint8_t gridCols = 1;
    uint8_t gridRows = 1;
};

class SlotHighlightRenderer {
public:
    SlotHighlightRenderer() = default;

    [[nodiscard]] std::expected<void, SlotHighlightError> RenderHighlight(
        UIRenderBatcher& batcher,
        const GridConfig& config,
        uint8_t col,
        uint8_t row,
        uint8_t width,
        uint8_t height,
        Color color,
        std::optional<TextureId> texture = std::nullopt
    ) const;
};

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::SlotHighlightError> : std::formatter<std::string_view> {
    auto format(Client::UI::SlotHighlightError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::to_string(err), ctx);
    }
};
