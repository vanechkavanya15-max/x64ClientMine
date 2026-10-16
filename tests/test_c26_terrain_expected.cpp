#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/GameLib/MapAttrLoader.h"
#include "../src/GameLib/MapRawLoader.h"

// Note: In isolated tests, we define dummy stdafx.h equivalent if needed or include cpp file directly
#include "../src/GameLib/MapAttrLoader.cpp"
#include "../src/GameLib/MapRawLoader.cpp"

#include <vector>
#include <cstdint>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <expected>

TEST_SUITE("C26 MapAttrLoader std::expected Migration") {

    TEST_CASE("LoadFromBuffer - Success") {
        std::vector<uint8_t> buffer(sizeof(MapAttrHeader) + 4);
        MapAttrHeader header;
        header.magic = MapAttrLoader::MAGIC_NUMBER;
        header.width = 2;
        header.height = 2;
        
        std::memcpy(buffer.data(), &header, sizeof(MapAttrHeader));
        buffer[sizeof(MapAttrHeader)] = 1;
        buffer[sizeof(MapAttrHeader) + 1] = 2;
        buffer[sizeof(MapAttrHeader) + 2] = 3;
        buffer[sizeof(MapAttrHeader) + 3] = 4;

        auto result = MapAttrLoader::LoadFromBuffer(buffer);
        
        REQUIRE(result.has_value());
        CHECK(result->header.width == 2);
        CHECK(result->header.height == 2);
        CHECK(result->attributes.size() == 4);
        CHECK(result->attributes[0] == 1);
        CHECK(result->attributes[3] == 4);
    }

    TEST_CASE("LoadFromBuffer - Invalid Size (too small for header)") {
        std::vector<uint8_t> buffer(sizeof(MapAttrHeader) - 1);
        auto result = MapAttrLoader::LoadFromBuffer(buffer);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapAttrLoadError::InvalidSize);
    }

    TEST_CASE("LoadFromBuffer - Invalid Magic Number") {
        std::vector<uint8_t> buffer(sizeof(MapAttrHeader) + 4);
        MapAttrHeader header;
        header.magic = 1234; // Invalid magic
        header.width = 2;
        header.height = 2;
        std::memcpy(buffer.data(), &header, sizeof(MapAttrHeader));
        
        auto result = MapAttrLoader::LoadFromBuffer(buffer);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapAttrLoadError::InvalidMagicNumber);
    }

    TEST_CASE("LoadFromBuffer - Invalid Dimensions (not enough data)") {
        std::vector<uint8_t> buffer(sizeof(MapAttrHeader) + 2); // Only 2 bytes data
        MapAttrHeader header;
        header.magic = MapAttrLoader::MAGIC_NUMBER;
        header.width = 2;
        header.height = 2; // Needs 4 bytes
        std::memcpy(buffer.data(), &header, sizeof(MapAttrHeader));
        
        auto result = MapAttrLoader::LoadFromBuffer(buffer);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapAttrLoadError::InvalidDimensions);
    }

    TEST_CASE("LoadFromFile - Success") {
        const std::string filename = "test_attr.atr";
        std::vector<uint8_t> buffer(sizeof(MapAttrHeader) + 4);
        MapAttrHeader header;
        header.magic = MapAttrLoader::MAGIC_NUMBER;
        header.width = 2;
        header.height = 2;
        std::memcpy(buffer.data(), &header, sizeof(MapAttrHeader));
        buffer[sizeof(MapAttrHeader)] = 10;
        
        std::ofstream out(filename, std::ios::binary);
        out.write(reinterpret_cast<const char*>(buffer.data()), buffer.size());
        out.close();

        auto result = MapAttrLoader::LoadFromFile(filename);
        
        REQUIRE(result.has_value());
        CHECK(result->header.width == 2);
        CHECK(result->attributes[0] == 10);

        std::filesystem::remove(filename);
    }

    TEST_CASE("LoadFromFile - File Open Failed") {
        auto result = MapAttrLoader::LoadFromFile("non_existent_file.atr");
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapAttrLoadError::FileOpenFailed);
    }
}

TEST_SUITE("C26 MapRawLoader std::expected Migration") {

    TEST_CASE("LoadTileMapFromBuffer - Success") {
        std::vector<uint8_t> buffer(MapRawLoader::EXPECTED_TILEMAP_SIZE, 0);
        buffer[0] = 5;
        buffer[MapRawLoader::EXPECTED_TILEMAP_SIZE - 1] = 10;

        auto result = MapRawLoader::LoadTileMapFromBuffer(buffer);
        
        REQUIRE(result.has_value());
        CHECK(result->tiles.size() == MapRawLoader::EXPECTED_TILEMAP_SIZE);
        CHECK(result->tiles[0] == 5);
        CHECK(result->tiles[MapRawLoader::EXPECTED_TILEMAP_SIZE - 1] == 10);
    }

    TEST_CASE("LoadTileMapFromBuffer - Invalid Size") {
        std::vector<uint8_t> buffer(MapRawLoader::EXPECTED_TILEMAP_SIZE - 1, 0);
        auto result = MapRawLoader::LoadTileMapFromBuffer(buffer);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapRawLoadError::InvalidSize);
    }

    TEST_CASE("LoadTileMapFromFile - Success") {
        const std::string filename = "test_tilemap.raw";
        std::vector<uint8_t> buffer(MapRawLoader::EXPECTED_TILEMAP_SIZE, 0);
        buffer[0] = 15;
        
        std::ofstream out(filename, std::ios::binary);
        out.write(reinterpret_cast<const char*>(buffer.data()), buffer.size());
        out.close();

        auto result = MapRawLoader::LoadTileMapFromFile(filename);
        
        REQUIRE(result.has_value());
        CHECK(result->tiles.size() == MapRawLoader::EXPECTED_TILEMAP_SIZE);
        CHECK(result->tiles[0] == 15);

        std::filesystem::remove(filename);
    }

    TEST_CASE("LoadTileMapFromFile - File Open Failed") {
        auto result = MapRawLoader::LoadTileMapFromFile("non_existent_file.raw");
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapRawLoadError::FileOpenFailed);
    }

    TEST_CASE("LoadHeightMapFromBuffer - Success") {
        std::vector<uint8_t> buffer(MapRawLoader::EXPECTED_HEIGHTMAP_SIZE, 0);
        
        // 16-bit value: 0x1234
        buffer[0] = 0x34;
        buffer[1] = 0x12;

        auto result = MapRawLoader::LoadHeightMapFromBuffer(buffer);
        
        REQUIRE(result.has_value());
        CHECK(result->heights.size() == MapRawLoader::EXPECTED_HEIGHTMAP_SIZE / 2);
        CHECK(result->heights[0] == 0x1234);
    }

    TEST_CASE("LoadHeightMapFromBuffer - Invalid Size") {
        std::vector<uint8_t> buffer(MapRawLoader::EXPECTED_HEIGHTMAP_SIZE + 1, 0);
        auto result = MapRawLoader::LoadHeightMapFromBuffer(buffer);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapRawLoadError::InvalidSize);
    }

    TEST_CASE("LoadHeightMapFromFile - Success") {
        const std::string filename = "test_heightmap.raw";
        std::vector<uint8_t> buffer(MapRawLoader::EXPECTED_HEIGHTMAP_SIZE, 0);
        
        // 16-bit value: 0xABCD
        buffer[2] = 0xCD;
        buffer[3] = 0xAB;
        
        std::ofstream out(filename, std::ios::binary);
        out.write(reinterpret_cast<const char*>(buffer.data()), buffer.size());
        out.close();

        auto result = MapRawLoader::LoadHeightMapFromFile(filename);
        
        REQUIRE(result.has_value());
        CHECK(result->heights.size() == MapRawLoader::EXPECTED_HEIGHTMAP_SIZE / 2);
        CHECK(result->heights[1] == 0xABCD);

        std::filesystem::remove(filename);
    }

    TEST_CASE("LoadHeightMapFromFile - File Open Failed") {
        auto result = MapRawLoader::LoadHeightMapFromFile("non_existent_file_h.raw");
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == MapRawLoadError::FileOpenFailed);
    }
}
