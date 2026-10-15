#pragma once

#include <vector>
#include <cstdint>
#include <expected>
#include <optional>
#include <span>

namespace Client::UI {

enum class UIBatchError {
    BufferOverflow,
    InvalidGeometry,
    InvalidTexture
};

struct Color {
    uint8_t r, g, b, a;
    
    constexpr bool operator==(const Color&) const = default;
};

struct UIVertex {
    float x, y, z;
    float u, v;
    Color color;

    constexpr bool operator==(const UIVertex&) const = default;
};

struct TextureId {
    uint32_t id;
    constexpr bool operator==(const TextureId&) const = default;
    constexpr bool operator!=(const TextureId&) const = default;
};

enum class PrimitiveType {
    TriangleList,
    LineList
};

struct UIBatch {
    PrimitiveType type;
    TextureId texture;
    std::size_t vertexStart;
    std::size_t vertexCount;
};

class UIRenderBatcher {
public:
    explicit UIRenderBatcher(std::size_t maxVertices = 65536);

    std::expected<void, UIBatchError> DrawRect(float x, float y, float w, float h, Color color);
    std::expected<void, UIBatchError> DrawTexturedRect(float x, float y, float w, float h, float u, float v, float uw, float vh, TextureId texture, Color color);
    std::expected<void, UIBatchError> DrawLine(float x1, float y1, float x2, float y2, Color color);
    
    void Flush();
    
    [[nodiscard]] std::span<const UIVertex> GetVertices() const;
    [[nodiscard]] std::span<const UIBatch> GetBatches() const;

private:
    std::expected<void, UIBatchError> EnsureCapacity(std::size_t additionalVertices);
    void AddBatchIfRequired(PrimitiveType type, TextureId texture);

    std::vector<UIVertex> m_vertices;
    std::vector<UIBatch> m_batches;
    std::size_t m_maxVertices;
    
    std::optional<PrimitiveType> m_currentPrimitiveType;
    std::optional<TextureId> m_currentTexture;
};

} // namespace Client::UI
