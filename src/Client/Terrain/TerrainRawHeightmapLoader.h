#pragma once

#include "Client/Core/IVfsProvider.h"
#include <expected>
#include <vector>
#include <cstdint>
#include <string_view>
#include <memory>

namespace Client::Terrain {

// Kody bledow dla ladowania mapy wysokosci
enum class TerrainHeightmapError {
    Success,
    VfsError,
    InvalidDimensions,
    InvalidFileSize
};

// Struktura przechowujaca dane mapy wysokosci
struct TerrainHeightmapData {
    uint32_t width;
    uint32_t height;
    std::vector<uint16_t> heights;
};

// Klasa odpowiedzialna za ladowanie surowych danych mapy wysokosci
class TerrainRawHeightmapLoader {
public:
    TerrainRawHeightmapLoader(std::shared_ptr<Client::Core::IVfsProvider> vfsProvider);

    // Laduje dane z podanej sciezki z oczekiwanymi wymiarami
    std::expected<TerrainHeightmapData, TerrainHeightmapError> Load(std::string_view path, uint32_t expectedWidth, uint32_t expectedHeight);

private:
    std::shared_ptr<Client::Core::IVfsProvider> m_vfsProvider;
};

} // namespace Client::Terrain
