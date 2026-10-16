#include "SpeedTreeBillboardRenderer.h"
#include <algorithm>

namespace Client::Vegetation {

void SpeedTreeBillboardRenderer::AddBillboard(const BillboardInstance& instance) {
    m_billboards.push_back(instance);
}

void SpeedTreeBillboardRenderer::Clear() noexcept {
    m_billboards.clear();
}

RenderResult SpeedTreeBillboardRenderer::GenerateQuads(const Vector3& cameraDir, const Vector3& worldUp) const {
    if (m_billboards.empty()) {
        return std::vector<BillboardQuad>{};
    }

    if (cameraDir.Length() < 0.000001f || worldUp.Length() < 0.000001f) {
        return std::unexpected(RenderError::InvalidCameraVector);
    }

    Vector3 normalizedCameraDir = cameraDir.Normalize();
    Vector3 right = worldUp.Cross(normalizedCameraDir).Normalize();
    if (right.Length() < 0.000001f) {
        return std::unexpected(RenderError::InvalidCameraVector);
    }
    
    Vector3 up = normalizedCameraDir.Cross(right).Normalize();

    std::vector<BillboardInstance> sortedBillboards = m_billboards;
    std::sort(sortedBillboards.begin(), sortedBillboards.end());

    std::vector<BillboardQuad> quads;
    quads.reserve(sortedBillboards.size());

    for (const auto& instance : sortedBillboards) {
        float halfWidth = instance.width * 0.5f;
        float halfHeight = instance.height * 0.5f;

        Vector3 rightScaled = right * halfWidth;
        Vector3 upScaled = up * halfHeight;

        Vector3 v0 = instance.position - rightScaled - upScaled; // Bottom-left
        Vector3 v1 = instance.position + rightScaled - upScaled; // Bottom-right
        Vector3 v2 = instance.position - rightScaled + upScaled; // Top-left
        Vector3 v3 = instance.position + rightScaled + upScaled; // Top-right

        BillboardQuad quad;
        quad.textureId = instance.textureId;
        
        // 0: Bottom-left
        quad.vertices[0] = {v0, {0.0f, 1.0f}};
        // 1: Top-left
        quad.vertices[1] = {v2, {0.0f, 0.0f}};
        // 2: Top-right
        quad.vertices[2] = {v3, {1.0f, 0.0f}};
        // 3: Bottom-right
        quad.vertices[3] = {v1, {1.0f, 1.0f}};

        quads.push_back(quad);
    }

    return quads;
}

} // namespace Client::Vegetation
