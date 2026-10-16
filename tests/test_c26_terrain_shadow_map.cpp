#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN

#include "doctest.h"
#include "../src/Client/Terrain/TerrainShadowMapRenderer.h"
#include "../src/Client/Terrain/IShadowMapRenderBackend.h"
#include "../src/Client/Core/StrongTypes.h"
#include <memory>

using namespace Client::Terrain;
using namespace Client::Core;

class MockShadowMapRenderBackend : public IShadowMapRenderBackend {
public:
    bool m_initCalled = false;
    bool m_beginCalled = false;
    bool m_endCalled = false;
    uint32_t m_width = 0;
    uint32_t m_height = 0;

    std::expected<void, ShadowMapError> m_initResult = {};
    std::expected<void, ShadowMapError> m_beginResult = {};
    std::expected<void, ShadowMapError> m_endResult = {};
    std::expected<void, ShadowMapError> m_setMatrixResult = {};

    std::expected<void, ShadowMapError> InitializeTarget(uint32_t width, uint32_t height) noexcept override {
        m_initCalled = true;
        m_width = width;
        m_height = height;
        return m_initResult;
    }

    std::expected<void, ShadowMapError> BeginTarget() noexcept override {
        m_beginCalled = true;
        return m_beginResult;
    }

    std::expected<void, ShadowMapError> EndTarget() noexcept override {
        m_endCalled = true;
        return m_endResult;
    }

    std::expected<void, ShadowMapError> SetShadowMatrix(const float* /*matrixData*/) noexcept override {
        return m_setMatrixResult;
    }
};

TEST_CASE("TerrainShadowMapRenderer - Initialization") {
    auto backend = std::make_shared<MockShadowMapRenderBackend>();
    TerrainShadowMapRenderer renderer(backend);

    SUBCASE("Successful initialization") {
        auto result = renderer.Initialize(1024);
        CHECK(result.has_value());
        CHECK(backend->m_initCalled);
        CHECK(backend->m_width == 1024);
        CHECK(backend->m_height == 1024);
    }

    SUBCASE("Double initialization returns error") {
        renderer.Initialize(1024);
        auto result = renderer.Initialize(1024);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ShadowMapError::AlreadyInitialized);
    }

    SUBCASE("Backend init failure propagation") {
        backend->m_initResult = std::unexpected(ShadowMapError::RenderTargetCreationFailure);
        auto result = renderer.Initialize(1024);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ShadowMapError::RenderTargetCreationFailure);
    }
}

TEST_CASE("TerrainShadowMapRenderer - Null Backend") {
    TerrainShadowMapRenderer renderer(nullptr);
    auto result = renderer.Initialize(1024);
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == ShadowMapError::BackendNotInitialized);
}

TEST_CASE("TerrainShadowMapRenderer - Render Pass Lifecycle") {
    auto backend = std::make_shared<MockShadowMapRenderBackend>();
    TerrainShadowMapRenderer renderer(backend);
    
    // Pass bez inicjalizacji
    CHECK_FALSE(renderer.BeginRenderPass().has_value());
    CHECK_FALSE(renderer.EndRenderPass().has_value());

    renderer.Initialize(1024);

    SUBCASE("Successful pass lifecycle") {
        auto beginRes = renderer.BeginRenderPass();
        CHECK(beginRes.has_value());
        CHECK(backend->m_beginCalled);

        auto endRes = renderer.EndRenderPass();
        CHECK(endRes.has_value());
        CHECK(backend->m_endCalled);
    }

    SUBCASE("Nested pass returns error") {
        renderer.BeginRenderPass();
        auto beginRes = renderer.BeginRenderPass();
        CHECK_FALSE(beginRes.has_value());
        CHECK(beginRes.error() == ShadowMapError::RenderPassAlreadyActive);
    }

    SUBCASE("End without begin returns error") {
        auto endRes = renderer.EndRenderPass();
        CHECK_FALSE(endRes.has_value());
        CHECK(endRes.error() == ShadowMapError::NoActiveRenderPass);
    }

    SUBCASE("Backend failure propagation") {
        backend->m_beginResult = std::unexpected(ShadowMapError::TextureBindingFailure);
        auto beginRes = renderer.BeginRenderPass();
        CHECK_FALSE(beginRes.has_value());
        CHECK(beginRes.error() == ShadowMapError::TextureBindingFailure);
    }
}

TEST_CASE("TerrainShadowMapRenderer - Light Direction") {
    auto backend = std::make_shared<MockShadowMapRenderBackend>();
    TerrainShadowMapRenderer renderer(backend);

    SUBCASE("Set before init returns error") {
        auto res = renderer.SetLightDirection(MapCoords{0.0f, -1.0f, 0.0f});
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ShadowMapError::BackendNotInitialized);
    }

    renderer.Initialize(1024);

    SUBCASE("Successful light direction set") {
        auto res = renderer.SetLightDirection(MapCoords{1.0f, -1.0f, 0.0f});
        CHECK(res.has_value());
    }

    SUBCASE("Invalid light direction returns error") {
        auto res = renderer.SetLightDirection(MapCoords{0.0f, 0.0f, 0.0f});
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ShadowMapError::InvalidLightDirection);
    }
}
