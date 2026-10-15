#include "MapAttrLoader.h"
#include <fstream>
#include <cstring>

/**
 * @brief Loads map attribute data from a raw binary buffer.
 * 
 * @param buffer A span of constant bytes containing the raw file data.
 * @param outData The structured MapAttrData to populate on success.
 * @return MapAttrLoadError::Success on successful load, or an error code on failure.
 */
std::expected<MapAttrData, MapAttrLoadError> MapAttrLoader::LoadFromBuffer(std::span<const uint8_t> buffer)
{
	if (buffer.size() < sizeof(MapAttrHeader))
	{
		return std::unexpected(MapAttrLoadError::InvalidSize);
	}

	MapAttrHeader header;
	std::memcpy(&header, buffer.data(), sizeof(MapAttrHeader));

	if (header.magic != MAGIC_NUMBER)
	{
		return std::unexpected(MapAttrLoadError::InvalidMagicNumber);
	}

	const std::size_t expectedDataSize = static_cast<std::size_t>(header.width) * header.height;
	const std::size_t requiredBufferSize = sizeof(MapAttrHeader) + expectedDataSize;

	if (buffer.size() < requiredBufferSize)
	{
		return std::unexpected(MapAttrLoadError::InvalidDimensions);
	}

	MapAttrData outData;
	outData.header = header;
	outData.attributes.resize(expectedDataSize);

	std::memcpy(outData.attributes.data(), buffer.data() + sizeof(MapAttrHeader), expectedDataSize);

	return outData;
}

/**
 * @brief Loads map attribute data directly from a file.
 * 
 * @param filePath The path to the attribute map file.
 * @param outData The structured MapAttrData to populate on success.
 * @return MapAttrLoadError::Success on successful load, or an error code on failure.
 */
std::expected<MapAttrData, MapAttrLoadError> MapAttrLoader::LoadFromFile(std::string_view filePath)
{
	std::ifstream file(std::string(filePath), std::ios::binary | std::ios::ate);
	if (!file.is_open())
	{
		return std::unexpected(MapAttrLoadError::FileOpenFailed);
	}

	const std::streamsize size = file.tellg();
	if (size < static_cast<std::streamsize>(sizeof(MapAttrHeader)))
	{
		return std::unexpected(MapAttrLoadError::InvalidSize);
	}

	file.seekg(0, std::ios::beg);
	std::vector<uint8_t> buffer(size);
	if (!file.read(reinterpret_cast<char*>(buffer.data()), size))
	{
		return std::unexpected(MapAttrLoadError::InvalidSize); // File read error resulting in mismatched size
	}

	return LoadFromBuffer(buffer);
}
