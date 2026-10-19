#pragma once

#include <cstdint>
#include <array>
#include <string>
#include "Client/Core/StrongTypes.h"

namespace Client::Terrain {

struct TextureIdTag {};
using TextureId = Client::Core::StrongType<TextureIdTag, uint32_t, 0>;

struct BufferIdTag {};
using BufferId = Client::Core::StrongType<BufferIdTag, uint32_t, 0>;

struct ImageIdTag {};
using ImageId = Client::Core::StrongType<ImageIdTag, uint32_t, 0>;

constexpr size_t TERRAIN_PATCHSIZE = 16;
constexpr size_t TERRAIN_SIZE = 128;
constexpr size_t TERRAIN_PATCHCOUNT = TERRAIN_SIZE / TERRAIN_PATCHSIZE;
constexpr size_t MAXTERRAINTEXTURES = 256;

struct TerrainSplat {
    int32_t active = 0;
    int32_t needsUpdate = 0;
    TextureId textureId;
};

struct TerrainSplatPatch {
    std::array<uint32_t, MAXTERRAINTEXTURES> tileCount{};
    std::array<std::array<uint32_t, MAXTERRAINTEXTURES>, TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT> patchTileCount{};
    std::array<TerrainSplat, MAXTERRAINTEXTURES> splats{};
    bool needsUpdate = false;
};

struct TerrainVBuffer {
    int8_t used = 0;
    int16_t mat = 0;
    
    BufferId vertexBufferId;
    BufferId indexBufferId;
    int32_t vertexSize = 0;
    
    int16_t numIndices = 0;
    
    float minx = 0.0f, maxx = 0.0f;
    float miny = 0.0f, maxy = 0.0f;
    float minz = 0.0f, maxz = 0.0f;
};

struct TerrainMaterial {
    std::array<char, 19> name{};
    float ambi_r = 0.0f, ambi_g = 0.0f, ambi_b = 0.0f, ambi_a = 0.0f;
    float diff_r = 0.0f, diff_g = 0.0f, diff_b = 0.0f, diff_a = 0.0f;
    float spec_r = 0.0f, spec_g = 0.0f, spec_b = 0.0f, spec_a = 0.0f;
    float spec_power = 0.0f;
};

struct TerrainGlobals {
    float pageUVLength = 0.0f;
    int32_t squaresPerTexture = 0;
    int32_t splatTilesX = 0;
    int32_t splatTilesY = 0;
    int32_t disableWrapping = 0;
    int32_t disableShadow = 0;
    int32_t shadowMode = 0;
    int32_t outsideVisible = 0;
    std::array<float, 3> sunLocation = {0.0f, 0.0f, 0.0f};
};

struct Matrix4x4 {
    std::array<float, 16> m{};
    
    static Matrix4x4 Identity() {
        return Matrix4x4{{
            1.0f, 0.0f, 0.0f, 0.0f,
            0.0f, 1.0f, 0.0f, 0.0f,
            0.0f, 0.0f, 1.0f, 0.0f,
            0.0f, 0.0f, 0.0f, 1.0f
        }};
    }
};

} // namespace Client::Terrain
