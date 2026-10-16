#pragma once

#include <span>
#include <optional>
#include <string_view>
#include "TextLayoutEngine.h"

namespace Client::Text {

class TextHyperlinkDetector {
public:
    [[nodiscard]] static std::optional<std::string_view> GetHyperlinkAtPosition(
        std::span<const TextLayoutHyperlinkBounds> bounds,
        float x,
        float y
    ) noexcept;
};

} // namespace Client::Text
