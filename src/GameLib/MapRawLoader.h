#pragma once

#include <cstdint>
#include <span>
#include <string_view>
#include <vector>
#include <expected>

/**
 * @brief Error codes for MapRawLoader operations.
 */
enum class MapRawLoadError : uint8_t
{
	Success = 0,
	FileOpenFailed,
	InvalidSize,
};

/**
 * @brief Holds the loaded data for a Raw Tile Map file.
 */
struct MapRawTileData
{
	std::vector<uint8_t> tiles; ///< Flattened 2D array of tile data.
};

/**
 * @brief Holds the loaded data for a Raw Height Map file.
 */
struct MapRawHeightData
{
	std::vector<uint16_t> heights; ///< Flattened 2D array of height data.
};

/**
 * @brief A standalone reader for RAW terrain files, fully decoupled from the GUI.
 */
class MapRawLoader
{
public:
	/**
	 * @brief Expected dimensions for tilemaps and heightmaps.
	 */
	static constexpr size_t TILEMAP_RAW_XSIZE = 258;
	static constexpr size_t TILEMAP_RAW_YSIZE = 258;
	static constexpr size_t EXPECTED_TILEMAP_SIZE = TILEMAP_RAW_XSIZE * TILEMAP_RAW_YSIZE * sizeof(uint8_t);

	static constexpr size_t HEIGHTMAP_RAW_XSIZE = 131;
	static constexpr size_t HEIGHTMAP_RAW_YSIZE = 131;
	static constexpr size_t EXPECTED_HEIGHTMAP_SIZE = HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE * sizeof(uint16_t);

	/**
	 * @brief Loads tilemap data from a raw binary buffer.
	 * 
	 * @param buffer A span of constant bytes containing the raw file data.
	 * @return Expected MapRawTileData on successful load, or an error code on failure.
	 */
	static std::expected<MapRawTileData, MapRawLoadError> LoadTileMapFromBuffer(std::span<const uint8_t> buffer);

	/**
	 * @brief Loads tilemap data directly from a file.
	 * 
	 * @param filePath The path to the tilemap file.
	 * @return Expected MapRawTileData on successful load, or an error code on failure.
	 */
	static std::expected<MapRawTileData, MapRawLoadError> LoadTileMapFromFile(std::string_view filePath);

	/**
	 * @brief Loads heightmap data from a raw binary buffer.
	 * 
	 * @param buffer A span of constant bytes containing the raw file data.
	 * @return Expected MapRawHeightData on successful load, or an error code on failure.
	 */
	static std::expected<MapRawHeightData, MapRawLoadError> LoadHeightMapFromBuffer(std::span<const uint8_t> buffer);

	/**
	 * @brief Loads heightmap data directly from a file.
	 * 
	 * @param filePath The path to the heightmap file.
	 * @return Expected MapRawHeightData on successful load, or an error code on failure.
	 */
	static std::expected<MapRawHeightData, MapRawLoadError> LoadHeightMapFromFile(std::string_view filePath);
};
