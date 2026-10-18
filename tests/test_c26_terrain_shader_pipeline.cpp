#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include "EterLib/Render/TerrainShaderPipeline.h"

using namespace EterLib::Render;

TEST_CASE("TerrainShaderPipeline: Lifecycle and Initial State") {
    TerrainShaderPipeline pipeline;
    CHECK_FALSE(pipeline.IsReady());

    // Shutdown without Initialize should not crash (RAII / safety)
    pipeline.Shutdown();
    CHECK_FALSE(pipeline.IsReady());
}

TEST_CASE("TerrainShaderPipeline: TerrainSectorBindDesc Defaults") {
    TerrainSectorBindDesc desc;
    CHECK(desc.sectorOriginX == 0.0f);
    CHECK(desc.sectorOriginY == 0.0f);
    CHECK(desc.invSectorSize == doctest::Approx(1.0f / 25600.0f));
    CHECK(desc.detailTileScale == 64.0f);
    CHECK(desc.splatTexture == nullptr);
    for (int i = 0; i < 4; ++i) {
        CHECK(desc.layerTextures[i] == nullptr);
    }
}
