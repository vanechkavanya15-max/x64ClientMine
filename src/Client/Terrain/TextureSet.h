#pragma once

#include <string>
#include <vector>
#include <expected>
#include "TerrainTypes.h"

namespace Client::Terrain {

enum class TextureSetError {
    InvalidIndex,
    EmptyTextureName,
    TextureNotFound,
    LoadFailed,
    SaveFailed
};

struct TerrainTexture {
    std::string filename;
    TextureId textureId;
    ImageId imageId;
    float uScale = 4.0f;
    float vScale = 4.0f;
    float uOffset = 0.0f;
    float vOffset = 0.0f;
    bool bSplat = true;
    uint16_t begin = 0;
    uint16_t end = 0;
    Matrix4x4 transform = Matrix4x4::Identity();
};

class TextureSet {
public:
    TextureSet();
    ~TextureSet() = default;

    void Initialize();
    void Clear();

    std::expected<void, TextureSetError> Load(std::string_view fileName, float terrainTexCoordBase);
    std::expected<void, TextureSetError> Save(std::string_view fileName);

    size_t GetTextureCount() const;

    std::expected<std::reference_wrapper<TerrainTexture>, TextureSetError> GetTexture(size_t index);
    std::expected<std::reference_wrapper<const TerrainTexture>, TextureSetError> GetTexture(size_t index) const;

    std::expected<void, TextureSetError> RemoveTexture(size_t index);

    std::expected<void, TextureSetError> SetTexture(size_t index,
                               std::string_view fileName,
                               float uScale, float vScale,
                               float uOffset, float vOffset,
                               bool bSplat,
                               uint16_t begin, uint16_t end,
                               float terrainTexCoordBase);

    void Reload(float terrainTexCoordBase);

    std::expected<void, TextureSetError> AddTexture(std::string_view fileName,
                               float uScale, float vScale,
                               float uOffset, float vOffset,
                               bool bSplat,
                               uint16_t begin, uint16_t end,
                               float terrainTexCoordBase);

    std::string_view GetFileName() const { return m_stFileName; }

protected:
    void AddEmptyTexture();
    void UpdateTransform(TerrainTexture& tex, float terrainTexCoordBase);

private:
    std::vector<TerrainTexture> m_Textures;
    std::string m_stFileName;
};

} // namespace Client::Terrain
