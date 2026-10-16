#include "MiniMapTextureRenderer.h"
#include <cmath>

namespace Client::MiniMap {

std::expected<std::vector<MiniMapRenderBatch>, std::string> MiniMapTextureRenderer::GenerateQuads(const MiniMapRenderContext& ctx) const {
    // Walidacja poprawnosci skali i rozmiaru komorki
    if (ctx.scale <= 0.0f) {
        return std::unexpected("Invalid map scale");
    }
    if (ctx.cellScale <= 0.0f) {
        return std::unexpected("Invalid cell scale");
    }

    // Obliczanie rozmiarow wyswietlanej srodkowej sekcji minimapy
    float scaledCellSize = ctx.scale * ctx.cellScale;
    float halfWidth = (3.0f * scaledCellSize) / 2.0f;

    // Przesuniecie kafelkow wzgledem pozycji srodka mapy
    float offsetX = std::fmod(ctx.mapCenterX, ctx.cellScale) * ctx.scale;
    float offsetY = std::fmod(ctx.mapCenterY, ctx.cellScale) * ctx.scale;

    float startX = ctx.screenX - halfWidth - offsetX;
    float startY = ctx.screenY - halfWidth - offsetY;

    std::vector<MiniMapRenderBatch> batches;
    batches.reserve(9);

    // Iteracja po siatce tekstur 3x3 i dodanie poprawnych wierzcholkow
    for (int row = 0; row < 3; ++row) {
        for (int col = 0; col < 3; ++col) {
            TextureHandle tex = ctx.textures[row * 3 + col];
            if (!tex) {
                continue; // Pomin brakujace tekstury z paczek wyjsciowych
            }

            float x0 = startX + col * scaledCellSize;
            float y0 = startY + row * scaledCellSize;
            float x1 = x0 + scaledCellSize;
            float y1 = y0 + scaledCellSize;

            MiniMapRenderBatch batch;
            batch.texture = tex;
            // Generowanie trasy (dwa trojkaty) dla danego quada
            batch.vertices = {{
                { x0, y0, 0.0f, 0.0f }, // Top-Left
                { x1, y0, 1.0f, 0.0f }, // Top-Right
                { x0, y1, 0.0f, 1.0f }, // Bottom-Left
                { x1, y0, 1.0f, 0.0f }, // Top-Right
                { x1, y1, 1.0f, 1.0f }, // Bottom-Right
                { x0, y1, 0.0f, 1.0f }  // Bottom-Left
            }};
            batches.push_back(batch);
        }
    }

    return batches;
}

} // namespace Client::MiniMap
