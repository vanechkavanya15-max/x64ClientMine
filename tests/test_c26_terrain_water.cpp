#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Terrain/TerrainWaterSurfaceRenderer.h"

using namespace Client::Terrain;
using namespace Client::Core;

class MockWaterRenderBackend : public IWaterRenderBackend
{
public:
    bool shouldFail = false;
    bool wasCalled = false;
    size_t lastVertexCount = 0;
    size_t lastIndexCount = 0;

    std::expected<void, WaterRendererError> DrawWaterPatch(
        const std::vector<WaterVertex>& vertices,
        const std::vector<uint16_t>& indices) override
    {
        wasCalled = true;
        if (shouldFail)
        {
            return std::unexpected(WaterRendererError::BackendFailure);
        }

        lastVertexCount = vertices.size();
        lastIndexCount = indices.size();

        return {};
    }
};

TEST_SUITE("TerrainWaterSurfaceRendererTests")
{
    TEST_CASE("Successful Generation")
    {
        MockWaterRenderBackend backend;
        TerrainWaterSurfaceRenderer renderer(&backend);

        MapCoords pos{ 0.0f, 0.0f, 0.0f };
        float size = 100.0f;
        float height = 10.0f;
        uint32_t resolution = 4; // 4x4 grid

        auto result = renderer.RenderWaterPatch(pos, size, height, resolution);

        CHECK(result.has_value());
        CHECK(backend.wasCalled);
        
        // (resolution + 1)^2 vertices
        CHECK(backend.lastVertexCount == 25);
        // resolution^2 * 6 indices
        CHECK(backend.lastIndexCount == 96);
    }

    TEST_CASE("Invalid Size")
    {
        MockWaterRenderBackend backend;
        TerrainWaterSurfaceRenderer renderer(&backend);

        MapCoords pos{ 0.0f, 0.0f, 0.0f };
        float size = -10.0f;
        float height = 10.0f;
        uint32_t resolution = 4;

        auto result = renderer.RenderWaterPatch(pos, size, height, resolution);

        CHECK(!result.has_value());
        CHECK(result.error() == WaterRendererError::InvalidSize);
        CHECK(!backend.wasCalled);
    }

    TEST_CASE("Invalid Resolution - Zero")
    {
        MockWaterRenderBackend backend;
        TerrainWaterSurfaceRenderer renderer(&backend);

        MapCoords pos{ 0.0f, 0.0f, 0.0f };
        float size = 100.0f;
        float height = 10.0f;
        uint32_t resolution = 0;

        auto result = renderer.RenderWaterPatch(pos, size, height, resolution);

        CHECK(!result.has_value());
        CHECK(result.error() == WaterRendererError::InvalidGridResolution);
        CHECK(!backend.wasCalled);
    }

    TEST_CASE("Invalid Resolution - Too High")
    {
        MockWaterRenderBackend backend;
        TerrainWaterSurfaceRenderer renderer(&backend);

        MapCoords pos{ 0.0f, 0.0f, 0.0f };
        float size = 100.0f;
        float height = 10.0f;
        uint32_t resolution = 256; // Max is 128

        auto result = renderer.RenderWaterPatch(pos, size, height, resolution);

        CHECK(!result.has_value());
        CHECK(result.error() == WaterRendererError::InvalidGridResolution);
        CHECK(!backend.wasCalled);
    }

    TEST_CASE("Null Backend")
    {
        TerrainWaterSurfaceRenderer renderer(nullptr);

        MapCoords pos{ 0.0f, 0.0f, 0.0f };
        float size = 100.0f;
        float height = 10.0f;
        uint32_t resolution = 4;

        auto result = renderer.RenderWaterPatch(pos, size, height, resolution);

        CHECK(!result.has_value());
        CHECK(result.error() == WaterRendererError::InvalidState);
    }

    TEST_CASE("Backend Failure")
    {
        MockWaterRenderBackend backend;
        backend.shouldFail = true;
        TerrainWaterSurfaceRenderer renderer(&backend);

        MapCoords pos{ 0.0f, 0.0f, 0.0f };
        float size = 100.0f;
        float height = 10.0f;
        uint32_t resolution = 4;

        auto result = renderer.RenderWaterPatch(pos, size, height, resolution);

        CHECK(!result.has_value());
        CHECK(result.error() == WaterRendererError::BackendFailure);
        CHECK(backend.wasCalled);
    }
    
    TEST_CASE("Wave Offset Update")
    {
        MockWaterRenderBackend backend;
        TerrainWaterSurfaceRenderer renderer(&backend);
        
        // Call it just to make sure it doesn't crash, we can't easily test internal state
        // without exposing it, but we can verify it's noexcept and compiles.
        renderer.UpdateWaveOffset(5.0f);
        CHECK(true);
    }
}
