#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Terrain/TerrainRawHeightmapLoader.h"
#include "Client/Core/IVfsProvider.h"
#include <vector>
#include <string>
#include <unordered_map>
#include <cstring>

using namespace Client::Terrain;
using namespace Client::Core;

class MockVfsProvider : public IVfsProvider {
public:
    std::unordered_map<std::string, std::vector<uint8_t>> files;

    VfsResult<std::vector<uint8_t>> ReadFile(std::string_view path) override {
        auto it = files.find(std::string(path));
        if (it != files.end()) {
            return it->second;
        }
        return std::unexpected(VfsError::FileNotFound);
    }

    VfsResult<std::span<const uint8_t>> GetCachedFile(std::string_view path) override {
        return std::unexpected(VfsError::FileNotFound);
    }

    bool Exists(std::string_view path) const override {
        return files.find(std::string(path)) != files.end();
    }
};

TEST_CASE("TerrainRawHeightmapLoader - Poprawne wczytanie pliku") {
    auto mockVfs = std::make_shared<MockVfsProvider>();
    
    const uint32_t width = 2;
    const uint32_t height = 2;
    std::vector<uint16_t> expectedHeights = { 10, 20, 30, 40 };
    
    std::vector<uint8_t> fileData(width * height * sizeof(uint16_t));
    std::memcpy(fileData.data(), expectedHeights.data(), fileData.size());
    
    mockVfs->files["test.raw"] = fileData;

    TerrainRawHeightmapLoader loader(mockVfs);
    auto result = loader.Load("test.raw", width, height);

    CHECK(result.has_value() == true);
    if (result) {
        CHECK(result->width == width);
        CHECK(result->height == height);
        CHECK(result->heights == expectedHeights);
    }
}

TEST_CASE("TerrainRawHeightmapLoader - Blad VFS (plik nie istnieje)") {
    auto mockVfs = std::make_shared<MockVfsProvider>();
    TerrainRawHeightmapLoader loader(mockVfs);
    
    auto result = loader.Load("nonexistent.raw", 2, 2);
    
    CHECK(result.has_value() == false);
    CHECK(result.error() == TerrainHeightmapError::VfsError);
}

TEST_CASE("TerrainRawHeightmapLoader - Blad wymiarow (0x0)") {
    auto mockVfs = std::make_shared<MockVfsProvider>();
    TerrainRawHeightmapLoader loader(mockVfs);
    
    auto result = loader.Load("test.raw", 0, 0);
    
    CHECK(result.has_value() == false);
    CHECK(result.error() == TerrainHeightmapError::InvalidDimensions);
}

TEST_CASE("TerrainRawHeightmapLoader - Niezgodny rozmiar pliku") {
    auto mockVfs = std::make_shared<MockVfsProvider>();
    
    const uint32_t width = 2;
    const uint32_t height = 2;
    
    // Plik jest za maly (tylko 2 wartosci uint16_t zamiast 4)
    std::vector<uint16_t> smallHeights = { 10, 20 };
    std::vector<uint8_t> fileData(smallHeights.size() * sizeof(uint16_t));
    std::memcpy(fileData.data(), smallHeights.data(), fileData.size());
    
    mockVfs->files["test.raw"] = fileData;

    TerrainRawHeightmapLoader loader(mockVfs);
    auto result = loader.Load("test.raw", width, height);

    CHECK(result.has_value() == false);
    CHECK(result.error() == TerrainHeightmapError::InvalidFileSize);
}
