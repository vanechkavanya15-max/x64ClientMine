#include "UIRenderBatcher.h"

namespace Client::UI {

UIRenderBatcher::UIRenderBatcher(std::size_t maxVertices) 
    : m_maxVertices(maxVertices) 
{
    m_vertices.reserve(1024);
    m_batches.reserve(128);
}

std::expected<void, UIBatchError> UIRenderBatcher::EnsureCapacity(std::size_t additionalVertices) {
    if (m_vertices.size() + additionalVertices > m_maxVertices) {
        return std::unexpected(UIBatchError::BufferOverflow);
    }
    return {};
}

void UIRenderBatcher::AddBatchIfRequired(PrimitiveType type, TextureId texture) {
    if (m_batches.empty() || m_currentPrimitiveType != type || m_currentTexture != texture) {
        m_batches.push_back(UIBatch{
            .type = type,
            .texture = texture,
            .vertexStart = m_vertices.size(),
            .vertexCount = 0
        });
        m_currentPrimitiveType = type;
        m_currentTexture = texture;
    }
}

std::expected<void, UIBatchError> UIRenderBatcher::DrawRect(float x, float y, float w, float h, Color color) {
    if (w <= 0.0f || h <= 0.0f) {
        return std::unexpected(UIBatchError::InvalidGeometry);
    }

    if (auto res = EnsureCapacity(6); !res) {
        return res;
    }

    TextureId noTexture{0};
    AddBatchIfRequired(PrimitiveType::TriangleList, noTexture);

    // Quad (2 triangles)
    UIVertex v0{x, y, 0.0f, 0.0f, 0.0f, color};
    UIVertex v1{x + w, y, 0.0f, 0.0f, 0.0f, color};
    UIVertex v2{x, y + h, 0.0f, 0.0f, 0.0f, color};
    UIVertex v3{x + w, y + h, 0.0f, 0.0f, 0.0f, color};

    m_vertices.insert(m_vertices.end(), {v0, v1, v2, v2, v1, v3});
    m_batches.back().vertexCount += 6;

    return {};
}

std::expected<void, UIBatchError> UIRenderBatcher::DrawTexturedRect(float x, float y, float w, float h, float u, float v, float uw, float vh, TextureId texture, Color color) {
    if (w <= 0.0f || h <= 0.0f) {
        return std::unexpected(UIBatchError::InvalidGeometry);
    }
    if (texture.id == 0) {
        return std::unexpected(UIBatchError::InvalidTexture);
    }

    if (auto res = EnsureCapacity(6); !res) {
        return res;
    }

    AddBatchIfRequired(PrimitiveType::TriangleList, texture);

    UIVertex v0{x, y, 0.0f, u, v, color};
    UIVertex v1{x + w, y, 0.0f, u + uw, v, color};
    UIVertex v2{x, y + h, 0.0f, u, v + vh, color};
    UIVertex v3{x + w, y + h, 0.0f, u + uw, v + vh, color};

    m_vertices.insert(m_vertices.end(), {v0, v1, v2, v2, v1, v3});
    m_batches.back().vertexCount += 6;

    return {};
}

std::expected<void, UIBatchError> UIRenderBatcher::DrawLine(float x1, float y1, float x2, float y2, Color color) {
    if (auto res = EnsureCapacity(2); !res) {
        return res;
    }

    TextureId noTexture{0};
    AddBatchIfRequired(PrimitiveType::LineList, noTexture);

    UIVertex v0{x1, y1, 0.0f, 0.0f, 0.0f, color};
    UIVertex v1{x2, y2, 0.0f, 0.0f, 0.0f, color};

    m_vertices.push_back(v0);
    m_vertices.push_back(v1);
    m_batches.back().vertexCount += 2;

    return {};
}

void UIRenderBatcher::Flush() {
    m_vertices.clear();
    m_batches.clear();
    m_currentPrimitiveType.reset();
    m_currentTexture.reset();
}

std::span<const UIVertex> UIRenderBatcher::GetVertices() const {
    return m_vertices;
}

std::span<const UIBatch> UIRenderBatcher::GetBatches() const {
    return m_batches;
}

} // namespace Client::UI
