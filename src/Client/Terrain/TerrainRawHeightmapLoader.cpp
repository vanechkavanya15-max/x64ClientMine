#include "TerrainRawHeightmapLoader.h"
#include <cstring>

namespace Client::Terrain {

TerrainRawHeightmapLoader::TerrainRawHeightmapLoader(std::shared_ptr<Client::Core::IVfsProvider> vfsProvider)
    : m_vfsProvider(std::move(vfsProvider)) {
}

std::expected<TerrainHeightmapData, TerrainHeightmapError> TerrainRawHeightmapLoader::Load(std::string_view path, uint32_t expectedWidth, uint32_t expectedHeight) {
    if (expectedWidth == 0 || expectedHeight == 0) {
        return std::unexpected(TerrainHeightmapError::InvalidDimensions);
    }

    auto vfsResult = m_vfsProvider->ReadFile(path);
    if (!vfsResult) {
        return std::unexpected(TerrainHeightmapError::VfsError);
    }

    const auto& fileData = vfsResult.value();
    size_t expectedSize = static_cast<size_t>(expectedWidth) * expectedHeight * sizeof(uint16_t);

    if (fileData.size() != expectedSize) {
        return std::unexpected(TerrainHeightmapError::InvalidFileSize);
    }

    TerrainHeightmapData data;
    data.width = expectedWidth;
    data.height = expectedHeight;
    data.heights.resize(expectedWidth * expectedHeight);

    std::memcpy(data.heights.data(), fileData.data(), expectedSize);

    return data;
}

} // namespace Client::Terrain
