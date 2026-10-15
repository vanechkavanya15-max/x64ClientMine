#pragma once

#include <vector>
#include <optional>
#include <span>
#include <expected>
#include "EterBase/Result.h"

namespace Client::MiniMap {

using TextureHandle = void*;

struct MiniMapRenderContext {
    float screenX;
    float screenY;
    float mapCenterX;
    float mapCenterY;
    float scale;
    float cellScale;
    
    // 9 obszarow, tak samo jak w klasycznym Metin2 (AROUND_AREA_NUM = 9)
    TextureHandle textures[9];
};

struct MiniMapVertex {
    float x, y, z;
    float u, v;
    
    constexpr bool operator==(const MiniMapVertex&) const noexcept = default;
};

struct MiniMapDrawBatch {
    TextureHandle texture;
    std::vector<MiniMapVertex> vertices;
};

class MiniMapTextureRenderer {
public:
    MiniMapTextureRenderer() = default;
    ~MiniMapTextureRenderer() = default;

    // Zero-copy span jako input kafelkow.
    // Metoda kalkuluje geometrię dla widocznych kafelkow mapy bez zaleznosci od bezposredniego DirectX.
    [[nodiscard]] EterBase::Result<std::vector<MiniMapDrawBatch>> GenerateQuads(const MiniMapRenderContext& context) const;
};

} // namespace Client::MiniMap
