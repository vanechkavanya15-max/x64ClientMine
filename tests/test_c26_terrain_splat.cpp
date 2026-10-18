#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#define TEST_MODE_DISABLE_STDAFX 1
#include "Client/Terrain/TerrainTextureSplatting.h"

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

TEST_CASE("TerrainTextureSplatting UpdateLayerScale") {
    TerrainTextureSplatting splatting;
    
    splatting.AddLayer(1, "terrain/grass.png", 5.0f, 5.0f);
    
    auto res = splatting.UpdateLayerScale(1, 15.0f, 20.0f);
    CHECK(res.has_value());
    
    auto layer = splatting.GetLayer(1);
    REQUIRE(layer.has_value());
    CHECK(layer->scaleU == 15.0f);
    CHECK(layer->scaleV == 20.0f);
    
    SUBCASE("Invalid index") {
        auto failRes = splatting.UpdateLayerScale(TerrainTextureSplatting::MAX_LAYERS, 10.0f, 10.0f);
        CHECK_FALSE(failRes.has_value());
        CHECK(failRes.error() == SplatError::InvalidLayerIndex);
    }
    
    SUBCASE("Layer not set") {
        auto failRes = splatting.UpdateLayerScale(2, 10.0f, 10.0f);
        CHECK_FALSE(failRes.has_value());
        CHECK(failRes.error() == SplatError::LayerNotSet);
    }
}

TEST_CASE("TerrainTextureSplatting ReplaceLayerTexture") {
    TerrainTextureSplatting splatting;
    
    splatting.AddLayer(2, "terrain/dirt.png");
    
    auto res = splatting.ReplaceLayerTexture(2, "terrain/mud.png");
    CHECK(res.has_value());
    
    auto layer = splatting.GetLayer(2);
    REQUIRE(layer.has_value());
    CHECK(layer->texturePath == "terrain/mud.png");
    
    SUBCASE("Invalid index") {
        auto failRes = splatting.ReplaceLayerTexture(TerrainTextureSplatting::MAX_LAYERS, "terrain/mud.png");
        CHECK_FALSE(failRes.has_value());
        CHECK(failRes.error() == SplatError::InvalidLayerIndex);
    }
    
    SUBCASE("Layer not set") {
        auto failRes = splatting.ReplaceLayerTexture(1, "terrain/mud.png");
        CHECK_FALSE(failRes.has_value());
        CHECK(failRes.error() == SplatError::LayerNotSet);
    }
}

TEST_CASE("TerrainTextureSplatting SwapLayers") {
    TerrainTextureSplatting splatting;
    
    splatting.AddLayer(0, "terrain/grass.png", 5.0f, 5.0f);
    splatting.AddLayer(2, "terrain/rock.png", 10.0f, 10.0f);
    
    auto res = splatting.SwapLayers(0, 2);
    CHECK(res.has_value());
    
    auto layer0 = splatting.GetLayer(0);
    REQUIRE(layer0.has_value());
    CHECK(layer0->layerIndex == 0);
    CHECK(layer0->texturePath == "terrain/rock.png");
    CHECK(layer0->scaleU == 10.0f);
    CHECK(layer0->scaleV == 10.0f);
    
    auto layer2 = splatting.GetLayer(2);
    REQUIRE(layer2.has_value());
    CHECK(layer2->layerIndex == 2);
    CHECK(layer2->texturePath == "terrain/grass.png");
    CHECK(layer2->scaleU == 5.0f);
    CHECK(layer2->scaleV == 5.0f);
    
    SUBCASE("Invalid indices") {
        auto failRes1 = splatting.SwapLayers(TerrainTextureSplatting::MAX_LAYERS, 2);
        CHECK_FALSE(failRes1.has_value());
        CHECK(failRes1.error() == SplatError::InvalidLayerIndex);
        
        auto failRes2 = splatting.SwapLayers(0, TerrainTextureSplatting::MAX_LAYERS);
        CHECK_FALSE(failRes2.has_value());
        CHECK(failRes2.error() == SplatError::InvalidLayerIndex);
    }
    
    SUBCASE("Layers not set") {
        auto failRes1 = splatting.SwapLayers(0, 1); // 1 is not set
        CHECK_FALSE(failRes1.has_value());
        CHECK(failRes1.error() == SplatError::LayerNotSet);
        
        auto failRes2 = splatting.SwapLayers(1, 2); // 1 is not set
        CHECK_FALSE(failRes2.has_value());
        CHECK(failRes2.error() == SplatError::LayerNotSet);
    }
}
