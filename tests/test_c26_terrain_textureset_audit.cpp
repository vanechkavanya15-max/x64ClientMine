#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <limits>
#include "PRTerrainLib/Terrain.h"
#include "PRTerrainLib/TextureSet.h"

// Pomocnicza klasa do testow jednostkowych dajaca dostep do chronionych pol
class TestableTerrain : public CTerrainImpl {
public:
    void SetRawShadowValue(size_t index, WORD val) {
        if (index < sizeof(m_awShadowMap) / sizeof(m_awShadowMap[0]))
            m_awShadowMap[index] = val;
    }
};

TEST_CASE("PRTerrainLib: GetShadowMapColor calculations and bounds") {
    TestableTerrain terrain;

    SUBCASE("Out of bounds and NaN protection") {
        CHECK(terrain.GetShadowMapColor(-1.0f, 100.0f) == 0xFFFFFFFFu);
        CHECK(terrain.GetShadowMapColor(100.0f, -1.0f) == 0xFFFFFFFFu);
        CHECK(terrain.GetShadowMapColor(static_cast<float>(CTerrainImpl::TERRAIN_XSIZE) + 10.0f, 0.0f) == 0xFFFFFFFFu);
        
        const float nanValue = std::numeric_limits<float>::quiet_NaN();
        CHECK(terrain.GetShadowMapColor(nanValue, 100.0f) == 0xFFFFFFFFu);
        CHECK(terrain.GetShadowMapColor(100.0f, nanValue) == 0xFFFFFFFFu);
    }

    SUBCASE("Default initialized shadowmap is white (0xFFFFFFFF)") {
        // Initialize() wypelnia 0xFFFF -> powinna byc pelna biel 0xFFFFFFFF
        DWORD col = terrain.GetShadowMapColor(0.0f, 0.0f);
        CHECK(col == 0xFFFFFFFFu);
    }

    SUBCASE("Black shadowmap texel (0x0000)") {
        terrain.SetRawShadowValue(0, 0x0000);
        DWORD col = terrain.GetShadowMapColor(0.0f, 0.0f);
        CHECK(col == 0xFF000000u); // Alpha 0xFF, R 0, G 0, B 0
    }

    SUBCASE("Component decoding with 5 to 8 bit expansion") {
        // 5-bit R = 31 (b11111 << 10), G = 0, B = 0 -> 0x7C00
        terrain.SetRawShadowValue(0, static_cast<WORD>(0x1F << 10));
        DWORD colRed = terrain.GetShadowMapColor(0.0f, 0.0f);
        CHECK(((colRed >> 16) & 0xFF) == 255); // (31 << 3) | (31 >> 2) = 248 | 7 = 255
        CHECK(((colRed >> 8) & 0xFF) == 0);
        CHECK((colRed & 0xFF) == 0);

        // 5-bit G = 31 (b11111 << 5) -> 0x03E0
        terrain.SetRawShadowValue(0, static_cast<WORD>(0x1F << 5));
        DWORD colGreen = terrain.GetShadowMapColor(0.0f, 0.0f);
        CHECK(((colGreen >> 16) & 0xFF) == 0);
        CHECK(((colGreen >> 8) & 0xFF) == 255);
        CHECK((colGreen & 0xFF) == 0);

        // 5-bit B = 31 -> 0x001F
        terrain.SetRawShadowValue(0, static_cast<WORD>(0x1F));
        DWORD colBlue = terrain.GetShadowMapColor(0.0f, 0.0f);
        CHECK(((colBlue >> 16) & 0xFF) == 0);
        CHECK(((colBlue >> 8) & 0xFF) == 0);
        CHECK((colBlue & 0xFF) == 255);
    }
}

TEST_CASE("PRTerrainLib: TextureSet eraser protection and safe indexing") {
    CTextureSet textureSet;
    textureSet.Create(); // Inicjalizuje error texture i dodaje slot 0 (gumke)

    CHECK(textureSet.GetTextureCount() == 1);

    SUBCASE("RemoveTexture on slot 0 is forbidden") {
        bool removed = textureSet.RemoveTexture(0);
        CHECK_FALSE(removed);
        CHECK(textureSet.GetTextureCount() == 1);
    }

    SUBCASE("Out of bounds texture returns error texture") {
        TTerrainTexture& tex = textureSet.GetTexture(999);
        CHECK(tex.stFilename.empty());
    }

    SUBCASE("TerrainImpl defaults without explicit SetTextureSet") {
        CTerrainImpl terrain;
        // Bez SetTextureSet, GetTextureSet() musi zwrocic bezpieczna atrape
        CHECK(terrain.GetNumTextures() >= 0);
        TTerrainTexture& t = terrain.GetTexture(0);
        CHECK(t.stFilename.empty());
    }
}
