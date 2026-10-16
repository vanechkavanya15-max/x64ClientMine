#include "MapRawLoader.h"
#include <fstream>
#include <cstring>

/**
 * @brief Loads tilemap data from a raw binary buffer.
 */
std::expected<MapRawTileData, MapRawLoadError> MapRawLoader::LoadTileMapFromBuffer(std::span<const uint8_t> buffer)
{
	if (buffer.size() != EXPECTED_TILEMAP_SIZE)
	{
		return std::unexpected(MapRawLoadError::InvalidSize);
	}

	MapRawTileData outData;
	outData.tiles.resize(EXPECTED_TILEMAP_SIZE);
	std::memcpy(outData.tiles.data(), buffer.data(), EXPECTED_TILEMAP_SIZE);

	return outData;
}

/**
 * @brief Loads tilemap data directly from a file.
 */
std::expected<MapRawTileData, MapRawLoadError> MapRawLoader::LoadTileMapFromFile(std::string_view filePath)
{
	std::ifstream file(std::string(filePath), std::ios::binary | std::ios::ate);
	if (!file.is_open())
	{
		return std::unexpected(MapRawLoadError::FileOpenFailed);
	}

	const std::streamsize size = file.tellg();
	if (size != static_cast<std::streamsize>(EXPECTED_TILEMAP_SIZE))
	{
		return std::unexpected(MapRawLoadError::InvalidSize);
	}

	file.seekg(0, std::ios::beg);
	std::vector<uint8_t> buffer(size);
	if (!file.read(reinterpret_cast<char*>(buffer.data()), size))
	{
		return std::unexpected(MapRawLoadError::InvalidSize);
	}

	return LoadTileMapFromBuffer(buffer);
}

/**
 * @brief Loads heightmap data from a raw binary buffer.
 */
std::expected<MapRawHeightData, MapRawLoadError> MapRawLoader::LoadHeightMapFromBuffer(std::span<const uint8_t> buffer)
{
	if (buffer.size() != EXPECTED_HEIGHTMAP_SIZE)
	{
		return std::unexpected(MapRawLoadError::InvalidSize);
	}

	MapRawHeightData outData;
	outData.heights.resize(HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE);
	std::memcpy(outData.heights.data(), buffer.data(), EXPECTED_HEIGHTMAP_SIZE);

	return outData;
}

/**
 * @brief Loads heightmap data directly from a file.
 */
std::expected<MapRawHeightData, MapRawLoadError> MapRawLoader::LoadHeightMapFromFile(std::string_view filePath)
{
	std::ifstream file(std::string(filePath), std::ios::binary | std::ios::ate);
	if (!file.is_open())
	{
		return std::unexpected(MapRawLoadError::FileOpenFailed);
	}

	const std::streamsize size = file.tellg();
	if (size != static_cast<std::streamsize>(EXPECTED_HEIGHTMAP_SIZE))
	{
		return std::unexpected(MapRawLoadError::InvalidSize);
	}

	file.seekg(0, std::ios::beg);
	std::vector<uint8_t> buffer(size);
	if (!file.read(reinterpret_cast<char*>(buffer.data()), size))
	{
		return std::unexpected(MapRawLoadError::InvalidSize);
	}

	return LoadHeightMapFromBuffer(buffer);
}
