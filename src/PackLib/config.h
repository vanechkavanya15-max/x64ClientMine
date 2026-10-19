#pragma once
#include <cstdint>
#include <array>
#include <vector>
#include <string>
#include <memory>
#include <unordered_map>
#include <span>
#include <stdexcept>
#include <cstring>
#include <algorithm>

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#endif

#include <sodium.h>

constexpr size_t PACK_KEY_SIZE = crypto_stream_xchacha20_KEYBYTES;      // 32 bytes
constexpr size_t PACK_NONCE_SIZE = crypto_stream_xchacha20_NONCEBYTES;  // 24 bytes

constexpr std::array<uint8_t, PACK_KEY_SIZE> PACK_KEY = {
	0x00,0x11,0x22,0x33, 0x44,0x55,0x66,0x77,
	0x88,0x99,0xAA,0xBB, 0xCC,0xDD,0xEE,0xFF,
	0x01,0x23,0x45,0x67, 0x89,0xAB,0xCD,0xEF,
	0xFE,0xDC,0xBA,0x98, 0x76,0x54,0x32,0x10
};

#pragma pack(push, 1)
struct TPackFileHeader
{
	uint64_t	entry_num;
	uint64_t	data_begin;
	uint8_t     nonce[PACK_NONCE_SIZE];
};
struct TPackFileEntry
{
	char		file_name[FILENAME_MAX+1];
	uint64_t	offset;
	uint64_t	file_size;
	uint64_t	compressed_size;
	uint8_t		encryption;
	uint8_t     nonce[PACK_NONCE_SIZE];
};
#pragma pack(pop)

class CPack;

enum class EPackFileStorageType : uint8_t
{
	Empty = 0,
	ZeroCopyView,       // Bezposredni widok std::span bez alokacji pamieci (mmap / external)
	VirtualMemory,      // Pamiec zaalokowana przez VirtualAlloc (bez fragmentacji sterty CRT)
	PooledVector        // Bufor zapasowy std::vector<uint8_t>
};

/**
 * @brief Nowoczesna abstrakcja pliku VFS wspierajaca zarowno widoki zero-copy (std::span)
 * jak i pamiec mapowana wirtualnie (VirtualAlloc/VirtualFree) bez alokacji sterty CRT.
 */
class TPackFile
{
public:
	TPackFile() noexcept
		: m_pData(nullptr)
		, m_size(0)
		, m_storageType(EPackFileStorageType::Empty)
		, m_virtAllocSize(0)
	{}

	// Konstruktor widoku zero-copy ze std::span
	explicit TPackFile(std::span<const uint8_t> view) noexcept
		: m_pData(const_cast<uint8_t*>(view.data()))
		, m_size(view.size())
		, m_storageType(EPackFileStorageType::ZeroCopyView)
		, m_virtAllocSize(0)
	{}

	TPackFile(const uint8_t* pData, size_t size) noexcept
		: m_pData(const_cast<uint8_t*>(pData))
		, m_size(size)
		, m_storageType(EPackFileStorageType::ZeroCopyView)
		, m_virtAllocSize(0)
	{}

	~TPackFile()
	{
		Release();
	}

	TPackFile(TPackFile&& other) noexcept
		: m_pData(other.m_pData)
		, m_size(other.m_size)
		, m_storageType(other.m_storageType)
		, m_virtAllocSize(other.m_virtAllocSize)
		, m_fallbackBuffer(std::move(other.m_fallbackBuffer))
	{
		other.m_pData = nullptr;
		other.m_size = 0;
		other.m_storageType = EPackFileStorageType::Empty;
		other.m_virtAllocSize = 0;
	}

	TPackFile& operator=(TPackFile&& other) noexcept
	{
		if (this != &other)
		{
			Release();
			m_pData = other.m_pData;
			m_size = other.m_size;
			m_storageType = other.m_storageType;
			m_virtAllocSize = other.m_virtAllocSize;
			m_fallbackBuffer = std::move(other.m_fallbackBuffer);

			other.m_pData = nullptr;
			other.m_size = 0;
			other.m_storageType = EPackFileStorageType::Empty;
			other.m_virtAllocSize = 0;
		}
		return *this;
	}

	TPackFile(const TPackFile& other)
	{
		CopyFrom(other);
	}

	TPackFile& operator=(const TPackFile& other)
	{
		if (this != &other)
		{
			Release();
			CopyFrom(other);
		}
		return *this;
	}

	void SetView(std::span<const uint8_t> view) noexcept
	{
		Release();
		m_pData = const_cast<uint8_t*>(view.data());
		m_size = view.size();
		m_storageType = EPackFileStorageType::ZeroCopyView;
		m_virtAllocSize = 0;
	}

	void SetView(const uint8_t* pData, size_t size) noexcept
	{
		Release();
		m_pData = const_cast<uint8_t*>(pData);
		m_size = size;
		m_storageType = EPackFileStorageType::ZeroCopyView;
		m_virtAllocSize = 0;
	}

	bool AllocateVirtual(size_t size)
	{
		if (m_storageType == EPackFileStorageType::VirtualMemory && m_pData && size <= m_virtAllocSize)
		{
			m_size = size;
			return true;
		}

		Release();
		if (size == 0)
			return true;

#ifdef _WIN32
		// Zaokraglenie rozmiaru w gore do pelnej strony 4KB
		size_t allocSize = (size + 4095) & ~static_cast<size_t>(4095);
		void* pMem = ::VirtualAlloc(nullptr, allocSize, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
		if (!pMem)
		{
			m_fallbackBuffer.resize(size);
			m_pData = m_fallbackBuffer.data();
			m_size = size;
			m_storageType = EPackFileStorageType::PooledVector;
			return true;
		}

		m_pData = static_cast<uint8_t*>(pMem);
		m_size = size;
		m_virtAllocSize = allocSize;
		m_storageType = EPackFileStorageType::VirtualMemory;
		return true;
#else
		m_fallbackBuffer.resize(size);
		m_pData = m_fallbackBuffer.data();
		m_size = size;
		m_storageType = EPackFileStorageType::PooledVector;
		return true;
#endif
	}

	void resize(size_t size)
	{
		if (m_storageType == EPackFileStorageType::VirtualMemory && m_pData && size <= m_virtAllocSize)
		{
			m_size = size;
			return;
		}
		AllocateVirtual(size);
	}

	void clear() noexcept
	{
		Release();
	}

	[[nodiscard]] const uint8_t* data() const noexcept { return m_pData; }
	[[nodiscard]] uint8_t* data() noexcept { return m_pData; }

	[[nodiscard]] size_t size() const noexcept { return m_size; }
	[[nodiscard]] bool empty() const noexcept { return m_size == 0; }

	[[nodiscard]] std::span<const uint8_t> span() const noexcept
	{
		return { m_pData, m_size };
	}

	[[nodiscard]] std::span<uint8_t> span() noexcept
	{
		return { m_pData, m_size };
	}

	operator std::span<const uint8_t>() const noexcept
	{
		return span();
	}

	[[nodiscard]] bool IsZeroCopy() const noexcept { return m_storageType == EPackFileStorageType::ZeroCopyView; }
	[[nodiscard]] bool IsVirtualMapped() const noexcept { return m_storageType == EPackFileStorageType::VirtualMemory; }
	[[nodiscard]] EPackFileStorageType GetStorageType() const noexcept { return m_storageType; }

	// Kontrola granic (Bounds Checking)
	[[nodiscard]] const uint8_t& at(size_t index) const
	{
		if (index >= m_size)
			throw std::out_of_range("TPackFile::at: index out of bounds");
		return m_pData[index];
	}

	[[nodiscard]] const uint8_t& operator[](size_t index) const noexcept
	{
		return m_pData[index];
	}

	[[nodiscard]] uint8_t& operator[](size_t index) noexcept
	{
		return m_pData[index];
	}

	// Bezpieczne wycinanie podwidokow
	[[nodiscard]] std::span<const uint8_t> subspan(size_t offset, size_t count = std::dynamic_extent) const
	{
		if (offset > m_size)
			return {};
		size_t available = m_size - offset;
		size_t actualCount = (count == std::dynamic_extent || count > available) ? available : count;
		return { m_pData + offset, actualCount };
	}

	const uint8_t* begin() const noexcept { return m_pData; }
	const uint8_t* end() const noexcept { return m_pData + m_size; }
	uint8_t* begin() noexcept { return m_pData; }
	uint8_t* end() noexcept { return m_pData + m_size; }

private:
	void Release() noexcept
	{
		if (m_storageType == EPackFileStorageType::VirtualMemory && m_pData)
		{
#ifdef _WIN32
			::VirtualFree(m_pData, 0, MEM_RELEASE);
#endif
		}
		m_pData = nullptr;
		m_size = 0;
		m_virtAllocSize = 0;
		m_storageType = EPackFileStorageType::Empty;
		m_fallbackBuffer.clear();
	}

	void CopyFrom(const TPackFile& other)
	{
		if (other.m_storageType == EPackFileStorageType::ZeroCopyView)
		{
			m_pData = other.m_pData;
			m_size = other.m_size;
			m_storageType = EPackFileStorageType::ZeroCopyView;
			m_virtAllocSize = 0;
		}
		else if (other.m_size > 0 && other.m_pData)
		{
			AllocateVirtual(other.m_size);
			if (m_pData)
			{
				std::memcpy(m_pData, other.m_pData, other.m_size);
			}
		}
		else
		{
			m_pData = nullptr;
			m_size = 0;
			m_storageType = EPackFileStorageType::Empty;
			m_virtAllocSize = 0;
		}
	}

private:
	uint8_t* m_pData{nullptr};
	size_t m_size{0};
	EPackFileStorageType m_storageType{EPackFileStorageType::Empty};
	size_t m_virtAllocSize{0};
	std::vector<uint8_t> m_fallbackBuffer;
};

using TPackFileMapEntry = std::pair<std::shared_ptr<CPack>, TPackFileEntry>;
using TPackFileMap = std::unordered_map<std::string, TPackFileMapEntry>;
