#pragma once

#include "Client/Core/ExpectedResult.h"
#include <cstdint>
#include <vector>
#include <span>
#include <optional>

namespace Client::Text {

enum class GlyphAtlasError {
    AtlasFull,
    GlyphTooLarge,
    InvalidDimensions,
    InvalidPixelDataSize
};

struct GlyphRect {
    uint32_t x;
    uint32_t y;
    uint32_t width;
    uint32_t height;

    constexpr auto operator<=>(const GlyphRect&) const = default;
};

class TextGlyphAtlasManager {
public:
    TextGlyphAtlasManager(uint32_t atlasWidth, uint32_t atlasHeight);
    ~TextGlyphAtlasManager() = default;

    // Delete copy/move to prevent accidental deep copies or state corruption
    TextGlyphAtlasManager(const TextGlyphAtlasManager&) = delete;
    TextGlyphAtlasManager& operator=(const TextGlyphAtlasManager&) = delete;
    TextGlyphAtlasManager(TextGlyphAtlasManager&&) noexcept = default;
    TextGlyphAtlasManager& operator=(TextGlyphAtlasManager&&) noexcept = default;

    Client::Core::ExpectedResult<GlyphRect, GlyphAtlasError> AddGlyph(
        uint32_t width,
        uint32_t height,
        std::span<const uint8_t> pixelData
    ) noexcept;

    void Clear() noexcept;

    [[nodiscard]] std::span<const uint8_t> GetAtlasPixelData() const noexcept;
    [[nodiscard]] uint32_t GetAtlasWidth() const noexcept;
    [[nodiscard]] uint32_t GetAtlasHeight() const noexcept;

private:
    uint32_t width_;
    uint32_t height_;
    
    // 1 byte per pixel (grayscale/alpha for fonts)
    std::vector<uint8_t> pixelBuffer_;

    // Shelf packing state
    uint32_t currentX_;
    uint32_t currentY_;
    uint32_t currentShelfHeight_;
};

} // namespace Client::Text
