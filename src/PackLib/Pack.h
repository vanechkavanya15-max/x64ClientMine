#pragma once
#include <string>
#include <span>
#include <mio/mmap.hpp>

#include "config.h"

class CBufferPool;

class CPack : public std::enable_shared_from_this<CPack>
{
public:
	CPack() = default;
	~CPack() = default;

	bool Load(const std::string& path);
	const std::vector<TPackFileEntry>& GetIndex() const { return m_index; }
	const std::string& GetPackName() const { return m_packName; }
	void SetPackName(const std::string& name) { m_packName = name; }

	// Pobiera plik jako TPackFile (zero-copy view dla plikow nieskompresowanych lub pamiec mapowana wirtualnie)
	bool GetFile(const TPackFileEntry& entry, TPackFile& result);
	bool GetFileWithPool(const TPackFileEntry& entry, TPackFile& result, CBufferPool* pPool);

	// Bezposredni dostep zero-copy std::span (zwraca widok jesli plik jest nieskompresowany i niezaszyfrowany)
	std::span<const uint8_t> GetFileView(const TPackFileEntry& entry) const noexcept;

	// Sprawdza czy dany wpis kwalifikuje sie do odczytu w 100% zero-copy
	bool IsZeroCopyEligible(const TPackFileEntry& entry) const noexcept;

private:
	void DecryptData(uint8_t* data, size_t len, const uint8_t* nonce);

	TPackFileHeader m_header;
	std::vector<TPackFileEntry> m_index;
	mio::mmap_source m_file;
	std::string m_packName;
};
