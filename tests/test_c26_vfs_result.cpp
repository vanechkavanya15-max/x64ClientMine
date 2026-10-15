#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "../src/Client/Core/VfsTypes.h"
#include "../src/Client/Core/IVfsProvider.h"
#include "../src/Client/Core/VfsLoader.h"
#include "../src/Client/Core/VfsLoader.cpp"

#include <unordered_map>
#include <string>

using namespace Client::Core;

class MockVfsProvider : public IVfsProvider {
public:
    std::unordered_map<std::string, std::vector<uint8_t>> files;
    std::unordered_map<std::string, VfsError> errors;

    VfsResult<std::vector<uint8_t>> ReadFile(std::string_view path) override {
        std::string pathStr(path);
        if (errors.contains(pathStr)) {
            return std::unexpected(errors[pathStr]);
        }
        if (files.contains(pathStr)) {
            return files[pathStr];
        }
        return std::unexpected(VfsError::FileNotFound);
    }

    VfsResult<std::span<const uint8_t>> GetCachedFile(std::string_view path) override {
        std::string pathStr(path);
        if (errors.contains(pathStr)) {
            return std::unexpected(errors[pathStr]);
        }
        if (files.contains(pathStr)) {
            return std::span<const uint8_t>(files[pathStr]);
        }
        return std::unexpected(VfsError::FileNotFound);
    }

    bool Exists(std::string_view path) const override {
        return files.contains(std::string(path));
    }
};

TEST_CASE("VfsLoader - Load valid file") {
    auto provider = std::make_shared<MockVfsProvider>();
    provider->files["data/config.ini"] = { 'a', 'b', 'c' };

    VfsLoader loader(provider);
    auto result = loader.LoadFile("data/config.ini");

    REQUIRE(result.has_value());
    REQUIRE(result->size() == 3);
    REQUIRE(result.value()[0] == 'a');
}

TEST_CASE("VfsLoader - Empty file returns EmptyFile error") {
    auto provider = std::make_shared<MockVfsProvider>();
    provider->files["data/empty.txt"] = {};

    VfsLoader loader(provider);
    auto result = loader.LoadFile("data/empty.txt");

    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == VfsError::EmptyFile);
}

TEST_CASE("VfsLoader - Invalid path returns InvalidPath error") {
    auto provider = std::make_shared<MockVfsProvider>();

    VfsLoader loader(provider);
    auto result = loader.LoadFile("");

    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == VfsError::InvalidPath);
}

TEST_CASE("VfsLoader - File not found") {
    auto provider = std::make_shared<MockVfsProvider>();

    VfsLoader loader(provider);
    auto result = loader.LoadFile("data/missing.txt");

    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == VfsError::FileNotFound);
}

TEST_CASE("VfsLoader - Provider error propagates (AccessDenied)") {
    auto provider = std::make_shared<MockVfsProvider>();
    provider->errors["data/secret.bin"] = VfsError::AccessDenied;

    VfsLoader loader(provider);
    auto result = loader.LoadFile("data/secret.bin");

    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == VfsError::AccessDenied);
}

TEST_CASE("VfsLoader - GetFileView valid file") {
    auto provider = std::make_shared<MockVfsProvider>();
    provider->files["data/image.png"] = { 0x89, 0x50, 0x4E, 0x47 };

    VfsLoader loader(provider);
    auto result = loader.GetFileView("data/image.png");

    REQUIRE(result.has_value());
    REQUIRE(result->size() == 4);
}

TEST_CASE("VfsLoader - Null provider handles gracefully") {
    VfsLoader loader(nullptr);
    auto result = loader.LoadFile("test.txt");

    REQUIRE_FALSE(result.has_value());
    REQUIRE(result.error() == VfsError::AccessDenied);
}

TEST_CASE("VfsLoader - CheckExists") {
    auto provider = std::make_shared<MockVfsProvider>();
    provider->files["data/config.ini"] = { '1' };

    VfsLoader loader(provider);
    REQUIRE(loader.CheckExists("data/config.ini") == true);
    REQUIRE(loader.CheckExists("data/missing.ini") == false);
    REQUIRE(loader.CheckExists("") == false);
}
