#include "TextHyperlinkDetector.h"

namespace Client::Text {

std::optional<std::string_view> TextHyperlinkDetector::GetHyperlinkAtPosition(
    std::span<const TextLayoutHyperlinkBounds> bounds,
    float x,
    float y
) noexcept {
    for (const auto& bound : bounds) {
        if (x >= bound.sx && x <= bound.ex && y >= bound.sy && y <= bound.ey) {
            return std::string_view{bound.text};
        }
    }
    return std::nullopt;
}

} // namespace Client::Text
