#pragma once

#include <string_view>
#include <vector>
#include <cstdint>
#include <expected>

namespace Client::Text {

enum class LineBreakError : uint8_t {
    None = 0,
    InvalidInput,
    MetricsProviderMissing
};

class ILineBreakMetricsProvider {
public:
    virtual ~ILineBreakMetricsProvider() = default;
    
    /**
     * @brief Gets the advance width for a single unicode codepoint.
     * @param codepoint The UTF-8 decoded codepoint
     * @return The advance width in logical pixels.
     */
    virtual float GetGlyphAdvance(char32_t codepoint) const = 0;
};

struct TextLineInfo {
    std::string_view content;
    float width{0.0f};
};

class TextLineBreakFormatter {
public:
    /**
     * @brief Formats a block of UTF-8 text into separate lines according to a max width, 
     *        handling explicit newlines and skipping UI formatting tags width calculation.
     * @param text The input UTF-8 text, possibly containing tags like |cffffffff, |r, |H, |h
     * @param metrics The metrics provider for width computation
     * @param maxLineWidth The maximum width per line
     * @return Expected list of lines or error
     */
    static std::expected<std::vector<TextLineInfo>, LineBreakError> BreakLines(
        std::string_view text,
        const ILineBreakMetricsProvider& metrics,
        float maxLineWidth
    );
};

} // namespace Client::Text
