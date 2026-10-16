#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TerrainCoordinateMapper.h"

TEST_CASE("TerrainCoordinateMapper: Sector indexing (GetTerrainNum)") {
    using namespace Client::Terrain;
    CHECK(TerrainCoordinateMapper::GetTerrainNum(100.0f, 100.0f, 3) == 0);
    CHECK(TerrainCoordinateMapper::GetTerrainNum(25700.0f, 100.0f, 3) == 1);
    CHECK(TerrainCoordinateMapper::GetTerrainNum(100.0f, 25700.0f, 3) == 3);
    CHECK(TerrainCoordinateMapper::GetTerrainNum(-100.0f, 100.0f, 3) == 0); // Out of bounds handles as 0
}

TEST_CASE("TerrainCoordinateMapper: ConvertToMapCoords") {
    using namespace Client::Terrain;
    
    SUBCASE("Positive valid coordinates") {
        auto result = TerrainCoordinateMapper::ConvertToMapCoords(100.0f, 150.0f);
        CHECK(result.terrainNumX == 0);
        CHECK(result.terrainNumY == 0);
        
        // local = 100.0, fooscale = 1.0f / 100 = 0.01
        // cellX = 100 * 0.01 = 1
        CHECK(result.cellX == 1);
        CHECK(result.cellY == 1); // 150 * 0.01 = 1.5 -> int is 1
        
        // subCell: ratioScale = 2 * 0.01 = 0.02
        // subCellX = 100 * 0.02 = 2. 2 % 2 = 0
        // subCellY = 150 * 0.02 = 3. 3 % 2 = 1
        CHECK(result.subCellX == 0);
        CHECK(result.subCellY == 1);
    }
    
    SUBCASE("High coordinates (wrapping)") {
        auto result = TerrainCoordinateMapper::ConvertToMapCoords(25700.0f, 51300.0f);
        // 25700 / 25600 = 1
        // 51300 / 25600 = 2
        CHECK(result.terrainNumX == 1);
        CHECK(result.terrainNumY == 2);
        
        // wrapped = 100.0f and 100.0f
        CHECK(result.cellX == 1);
        CHECK(result.cellY == 1);
        CHECK(result.subCellX == 0);
        CHECK(result.subCellY == 0);
    }
    
    SUBCASE("Negative coordinates (x negative)") {
        auto result = TerrainCoordinateMapper::ConvertToMapCoords(-100.0f, 100.0f);
        CHECK(result.terrainNumX == 0); // -100 / 25600 = 0 in integer math towards 0
        CHECK(result.terrainNumY == 0);
        
        // wrappedFx = 25500.0f
        // cellX = 25500 * 0.01 = 255
        CHECK(result.cellX == 255);
        CHECK(result.cellY == 1);
    }
    
    SUBCASE("Negative coordinates (y negative)") {
        auto result = TerrainCoordinateMapper::ConvertToMapCoords(100.0f, -100.0f);
        // fy = -fy => 100.0f
        CHECK(result.terrainNumX == 0);
        CHECK(result.terrainNumY == 0);
        
        CHECK(result.cellX == 1);
        CHECK(result.cellY == 1);
    }
}

TEST_CASE("TerrainCoordinateMapper: GetPatchCoords") {
    using namespace Client::Terrain;
    
    SUBCASE("First patch") {
        auto result = TerrainCoordinateMapper::GetPatchCoords(100.0f, 100.0f);
        CHECK(result.patchNumX == 0);
        CHECK(result.patchNumY == 0);
    }
    
    SUBCASE("Second patch X") {
        // PATCH_XSIZE = 16, CELLSCALE = 200
        // To be in patch 1, x0 >= 16 => fx >= 3200
        auto result = TerrainCoordinateMapper::GetPatchCoords(3300.0f, 100.0f);
        CHECK(result.patchNumX == 1);
        CHECK(result.patchNumY == 0);
    }
}
