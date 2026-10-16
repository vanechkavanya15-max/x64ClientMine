#pragma once

#include <vector>
#include <array>
#include <string>
#include <expected>

namespace Client::MiniMap {

// Uchwyt tekstury, bez zaleznosci od Windows/DirectX
using TextureHandle = void*;

// Struktura wierzcholka renderowania UI dla minimapy (brak wspolrzednej z, poniewaz to 2D UI)
struct MiniMapVertex {
    float x;
    float y;
    float u;
    float v;
};

// Paczka z wierzcholkami quada dla danej tekstury
struct MiniMapRenderBatch {
    TextureHandle texture;
    std::array<MiniMapVertex, 6> vertices;
};

// Kontekst renderowania minimapy (z parametrami skali i offsetu)
struct MiniMapRenderContext {
    float scale = 1.0f;
    float cellScale = 256.0f;
    float screenX = 0.0f;
    float screenY = 0.0f;
    float mapCenterX = 0.0f;
    float mapCenterY = 0.0f;
    std::array<TextureHandle, 9> textures{};
};

// Klasa izolowanego modulu C++23 do obliczania geometrii (quadow) minimapy
class MiniMapTextureRenderer {
public:
    // Zwraca wektor paczek renderowania (batchy) lub string z bledem w przypadku zlych parametrow wejsciowych
    std::expected<std::vector<MiniMapRenderBatch>, std::string> GenerateQuads(const MiniMapRenderContext& ctx) const;
};

} // namespace Client::MiniMap
