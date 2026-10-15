#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/GameLib/MapAttrLoader.h"

// Note: In isolated tests, we define dummy stdafx.h equivalent if needed or include cpp file directly
#include "../src/GameLib/MapAttrLoader.cpp"

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
