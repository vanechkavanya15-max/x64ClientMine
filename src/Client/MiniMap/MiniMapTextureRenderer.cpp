#include "MiniMapTextureRenderer.h"
#include <cmath>

namespace Client::MiniMap {

EterBase::Result<std::vector<MiniMapDrawBatch>> MiniMapTextureRenderer::GenerateQuads(const MiniMapRenderContext& context) const
{
    if (context.scale <= 0.0f) {
        return std::unexpected("Invalid map scale");
    }

    if (context.cellScale <= 0.0f) {
        return std::unexpected("Invalid cell scale");
    }

    std::vector<MiniMapDrawBatch> batches;
    batches.reserve(9);

    float scaledCellSize = context.cellScale * context.scale;
    float mapHalfWidth = (3.0f * scaledCellSize) / 2.0f; 
    float mapHalfHeight = (3.0f * scaledCellSize) / 2.0f;
    
    // Rzeczywista implementacja biznesowa uzywajaca mapCenter do offsetowania z renderowaniem
    // srodek renderingu ekranu to screenX, screenY.
    // pozycja swiata to mapCenterX, mapCenterY w skali komorek mapy.
    
    // Obliczamy wzgledny offset od centrum mapy.
    // Srodek minimapy na ekranie:
    float screenCenterX = context.screenX;
    float screenCenterY = context.screenY;

    // Przesuniecie wywolane pozycja na swiecie (mapCenter) modulo wielkosc kafelka.
    // offset wewnatrz kafelka, przeliczony na piksele:
    float offsetX = std::fmod(context.mapCenterX, context.cellScale) * context.scale;
    float offsetY = std::fmod(context.mapCenterY, context.cellScale) * context.scale;

    // Startowy X, Y dla obszaru 3x3 (tak ze srodek znajduje sie na ekranie)
    float startX = screenCenterX - mapHalfWidth - offsetX;
    float startY = screenCenterY - mapHalfHeight - offsetY;

    for (int i = 0; i < 9; ++i) {
        TextureHandle tex = context.textures[i];
        if (!tex) {
            continue;
        }

        int col = i % 3;
        int row = i / 3;

        float quadX = startX + static_cast<float>(col) * scaledCellSize;
        float quadY = startY + static_cast<float>(row) * scaledCellSize;

        MiniMapDrawBatch batch;
        batch.texture = tex;
        batch.vertices.reserve(6);

        // Quad składa się z 2 trójkątów = 6 wierzchołków
        // Trojkat 1 (Top-Left, Top-Right, Bottom-Left)
        batch.vertices.push_back({ quadX, quadY, 0.0f, 0.0f, 0.0f }); // TL
        batch.vertices.push_back({ quadX + scaledCellSize, quadY, 0.0f, 1.0f, 0.0f }); // TR
        batch.vertices.push_back({ quadX, quadY + scaledCellSize, 0.0f, 0.0f, 1.0f }); // BL

        // Trojkat 2 (Bottom-Left, Top-Right, Bottom-Right)
        batch.vertices.push_back({ quadX, quadY + scaledCellSize, 0.0f, 0.0f, 1.0f }); // BL
        batch.vertices.push_back({ quadX + scaledCellSize, quadY, 0.0f, 1.0f, 0.0f }); // TR
        batch.vertices.push_back({ quadX + scaledCellSize, quadY + scaledCellSize, 0.0f, 1.0f, 1.0f }); // BR

        batches.push_back(std::move(batch));
    }

    return batches;
}

} // namespace Client::MiniMap
