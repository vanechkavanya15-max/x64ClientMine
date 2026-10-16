#include "TextGlyphAtlasManager.h"
#include <algorithm>
#include <cstring>

namespace Client::Text {

TextGlyphAtlasManager::TextGlyphAtlasManager(uint32_t atlasWidth, uint32_t atlasHeight)
    : width_(atlasWidth),
      height_(atlasHeight),
      pixelBuffer_(atlasWidth * atlasHeight, 0),
      currentX_(0),
      currentY_(0),
      currentShelfHeight_(0) {
}

Client::Core::ExpectedResult<GlyphRect, GlyphAtlasError> TextGlyphAtlasManager::AddGlyph(
    uint32_t width,
    uint32_t height,
    std::span<const uint8_t> pixelData) noexcept {

    if (width == 0 || height == 0) {
        return Client::Core::MakeUnexpected(GlyphAtlasError::InvalidDimensions);
    }

    if (width > width_ || height > height_) {
        return Client::Core::MakeUnexpected(GlyphAtlasError::GlyphTooLarge);
    }

    if (pixelData.size() != static_cast<size_t>(width * height)) {
        return Client::Core::MakeUnexpected(GlyphAtlasError::InvalidPixelDataSize);
    }

    // Check if it fits on the current shelf
    if (currentX_ + width > width_) {
        // Move to the next shelf
        currentY_ += currentShelfHeight_;
        currentX_ = 0;
        currentShelfHeight_ = 0;
    }

    // Check if it fits vertically
    if (currentY_ + height > height_) {
        return Client::Core::MakeUnexpected(GlyphAtlasError::AtlasFull);
    }

    GlyphRect rect{currentX_, currentY_, width, height};

    // Update shelf height
    currentShelfHeight_ = std::max(currentShelfHeight_, height);

    // Copy pixel data
    for (uint32_t row = 0; row < height; ++row) {
        size_t destIndex = (currentY_ + row) * width_ + currentX_;
        size_t srcIndex = row * width;
        std::memcpy(&pixelBuffer_[destIndex], &pixelData[srcIndex], width);
    }

    // Advance X for the next glyph
    currentX_ += width;
    glyphCount_++;

    return Client::Core::ExpectedResult<GlyphRect, GlyphAtlasError>(rect);
}

void TextGlyphAtlasManager::Clear() noexcept {
    std::fill(pixelBuffer_.begin(), pixelBuffer_.end(), 0);
    currentX_ = 0;
    currentY_ = 0;
    currentShelfHeight_ = 0;
    glyphCount_ = 0;
}

Client::Core::ExpectedResult<void, GlyphAtlasError> TextGlyphAtlasManager::ResizeAtlas(uint32_t newWidth, uint32_t newHeight) noexcept {
    if (newWidth < width_ || newHeight < height_) {
        return Client::Core::MakeUnexpected(GlyphAtlasError::InvalidDimensions);
    }

    std::vector<uint8_t> newBuffer(newWidth * newHeight, 0);

    // Copy old texture row by row
    for (uint32_t row = 0; row < height_; ++row) {
        std::memcpy(&newBuffer[row * newWidth], &pixelBuffer_[row * width_], width_);
    }

    pixelBuffer_ = std::move(newBuffer);
    width_ = newWidth;
    height_ = newHeight;

    return {};
}

std::span<const uint8_t> TextGlyphAtlasManager::GetAtlasPixelData() const noexcept {
    return pixelBuffer_;
}

uint32_t TextGlyphAtlasManager::GetAtlasWidth() const noexcept {
    return width_;
}

uint32_t TextGlyphAtlasManager::GetAtlasHeight() const noexcept {
    return height_;
}

uint32_t TextGlyphAtlasManager::GetGlyphCount() const noexcept {
    return glyphCount_;
}

} // namespace Client::Text
