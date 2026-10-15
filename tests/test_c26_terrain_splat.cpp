#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#define TEST_MODE_DISABLE_STDAFX 1
#include "src/Client/Terrain/TerrainTextureSplatting.h"

using namespace Client::Terrain;

TEST_CASE("TerrainTextureSplatting Initialization and Defaults") {
    TerrainTextureSplatting splatting;
    
    CHECK(splatting.GetActiveLayerCount() == 0);
    
    for (uint8_t i = 0; i < TerrainTextureSplatting::MAX_LAYERS; ++i) {
        CHECK_FALSE(splatting.HasLayer(i));
        auto result = splatting.GetLayer(i);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SplatError::LayerNotSet);
    }
}

TEST_CASE("TerrainTextureSplatting Add and Retrieve Layers") {
    TerrainTextureSplatting splatting;
    
    auto res1 = splatting.AddLayer(0, "terrain/grass.png", 5.0f, 5.0f);
    CHECK(res1.has_value());
    CHECK(splatting.HasLayer(0));
    CHECK(splatting.GetActiveLayerCount() == 1);
    
    auto res2 = splatting.AddLayer(2, "terrain/rock.png", 10.0f, 10.0f);
    CHECK(res2.has_value());
    CHECK(splatting.HasLayer(2));
    CHECK(splatting.GetActiveLayerCount() == 2);
    
    auto layer0 = splatting.GetLayer(0);
    REQUIRE(layer0.has_value());
    CHECK(layer0->layerIndex == 0);
    CHECK(layer0->texturePath == "terrain/grass.png");
    CHECK(layer0->scaleU == 5.0f);
    CHECK(layer0->scaleV == 5.0f);
    
    auto layer2 = splatting.GetLayer(2);
    REQUIRE(layer2.has_value());
    CHECK(layer2->layerIndex == 2);
    CHECK(layer2->texturePath == "terrain/rock.png");
    CHECK(layer2->scaleU == 10.0f);
    CHECK(layer2->scaleV == 10.0f);
}

TEST_CASE("TerrainTextureSplatting Error Handling") {
    TerrainTextureSplatting splatting;
    
    SUBCASE("Out of bounds index") {
        auto res = splatting.AddLayer(TerrainTextureSplatting::MAX_LAYERS, "terrain/dirt.png");
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == SplatError::InvalidLayerIndex);
        
        auto getRes = splatting.GetLayer(TerrainTextureSplatting::MAX_LAYERS);
        CHECK_FALSE(getRes.has_value());
        CHECK(getRes.error() == SplatError::InvalidLayerIndex);
        
        auto remRes = splatting.RemoveLayer(TerrainTextureSplatting::MAX_LAYERS);
        CHECK_FALSE(remRes.has_value());
        CHECK(remRes.error() == SplatError::InvalidLayerIndex);
    }
    
    SUBCASE("Layer already exists") {
        splatting.AddLayer(1, "terrain/sand.png");
        auto res = splatting.AddLayer(1, "terrain/snow.png");
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == SplatError::LayerAlreadyExists);
    }
    
    SUBCASE("Remove non-existent layer") {
        auto res = splatting.RemoveLayer(0);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == SplatError::LayerNotSet);
    }
}

TEST_CASE("TerrainTextureSplatting Remove and Clear") {
    TerrainTextureSplatting splatting;
    
    splatting.AddLayer(0, "terrain/grass.png");
    splatting.AddLayer(1, "terrain/dirt.png");
    splatting.AddLayer(2, "terrain/rock.png");
    splatting.AddLayer(3, "terrain/snow.png");
    
    CHECK(splatting.GetActiveLayerCount() == 4);
    
    auto remRes = splatting.RemoveLayer(1);
    CHECK(remRes.has_value());
    CHECK(splatting.GetActiveLayerCount() == 3);
    CHECK_FALSE(splatting.HasLayer(1));
    
    splatting.Clear();
    CHECK(splatting.GetActiveLayerCount() == 0);
    for (uint8_t i = 0; i < TerrainTextureSplatting::MAX_LAYERS; ++i) {
        CHECK_FALSE(splatting.HasLayer(i));
    }
}

TEST_CASE("TerrainTextureSplatting GetAllActiveLayers") {
    TerrainTextureSplatting splatting;
    
    splatting.AddLayer(0, "terrain/grass.png");
    splatting.AddLayer(3, "terrain/snow.png");
    
    auto activeLayers = splatting.GetAllActiveLayers();
    REQUIRE(activeLayers.size() == 2);
    
    CHECK(activeLayers[0].layerIndex == 0);
    CHECK(activeLayers[0].texturePath == "terrain/grass.png");
    
    CHECK(activeLayers[1].layerIndex == 3);
    CHECK(activeLayers[1].texturePath == "terrain/snow.png");
}
