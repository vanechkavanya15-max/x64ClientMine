#include "Pack.h"
#include "EterLib/BufferPool.h"
#include <zstd.h>
#include <filesystem>
#include <cstring>
#include <algorithm>

namespace
{
	struct ThreadLocalZstdDCtx
	{
		ZSTD_DCtx* ctx = nullptr;
		ThreadLocalZstdDCtx() : ctx(ZSTD_createDCtx()) {}
		~ThreadLocalZstdDCtx()
		{
			if (ctx)
			{
				ZSTD_freeDCtx(ctx);
				ctx = nullptr;
			}
		}
	};
	static thread_local ThreadLocalZstdDCtx s_zstdDCtx;

	// Bufor roboczy thread-local do deszyfrowania strumienia XChaCha20 bez alokacji na stercie
	struct ThreadLocalDecryptScratch
	{
		std::vector<uint8_t> buffer;
		uint8_t* EnsureCapacity(size_t needed)
		{
			if (buffer.size() < needed)
			{
				size_t newCap = (std::max)(needed, buffer.size() * 3 / 2);
				buffer.resize(newCap);
			}
			return buffer.data();
		}
	};
	static thread_local ThreadLocalDecryptScratch s_decryptScratch;
}

static ZSTD_DCtx* GetThreadLocalZSTDContext()
{
	return s_zstdDCtx.ctx;
}

void CPack::DecryptData(uint8_t* data, size_t len, const uint8_t* nonce)
{
	crypto_stream_xchacha20_xor(data, data, len, nonce, PACK_KEY.data());
}

bool CPack::IsZeroCopyEligible(const TPackFileEntry& entry) const noexcept
{
	return entry.encryption == 0 && entry.compressed_size == entry.file_size;
}

std::span<const uint8_t> CPack::GetFileView(const TPackFileEntry& entry) const noexcept
{
	if (m_file.size() == 0 || !m_file.is_open())
		return {};

	size_t offset = m_header.data_begin + entry.offset;
	if (offset > m_file.size() || entry.compressed_size > m_file.size() - offset)
		return {};

	if (IsZeroCopyEligible(entry))
	{
		const uint8_t* pRaw = reinterpret_cast<const uint8_t*>(m_file.data()) + offset;
		return { pRaw, static_cast<size_t>(entry.file_size) };
	}

	return {};
}

bool CPack::Load(const std::string& path)
{
	std::error_code ec;
	m_file.map(path, ec);

	if (ec) {
		return false;
	}

	std::filesystem::path p = std::filesystem::u8path(path);
	m_packName = p.stem().string();

	size_t file_size = m_file.size();
	if (file_size < sizeof(TPackFileHeader)) {
		return false;
	}

	memcpy(&m_header, m_file.data(), sizeof(TPackFileHeader));

	if (m_header.entry_num > file_size / sizeof(TPackFileEntry)) {
		return false;
	}

	uint64_t index_bytes = static_cast<uint64_t>(m_header.entry_num) * sizeof(TPackFileEntry);
	if (file_size - sizeof(TPackFileHeader) < index_bytes) {
		return false;
	}

	if (m_header.data_begin > file_size) {
		return false;
	}

	m_index.resize(m_header.entry_num);

	for (size_t i = 0; i < m_header.entry_num; i++) {
		TPackFileEntry& entry = m_index[i];
		memcpy(&entry, m_file.data() + sizeof(TPackFileHeader) + i * sizeof(TPackFileEntry), sizeof(TPackFileEntry));
		DecryptData((uint8_t*)&entry, sizeof(TPackFileEntry), m_header.nonce);

		if (entry.offset > file_size || entry.compressed_size > file_size) {
			return false;
		}

		uint64_t entry_end = static_cast<uint64_t>(m_header.data_begin) + entry.offset;
		if (entry_end < m_header.data_begin) {
			return false;
		}

		if (entry_end + entry.compressed_size < entry_end || entry_end + entry.compressed_size > file_size) {
			return false;
		}
	}

	return true;
}

bool CPack::GetFile(const TPackFileEntry& entry, TPackFile& result)
{
	return GetFileWithPool(entry, result, nullptr);
}

bool CPack::GetFileWithPool(const TPackFileEntry& entry, TPackFile& result, CBufferPool* pPool)
{
	if (m_file.size() == 0 || !m_file.is_open())
		return false;

	size_t offset = m_header.data_begin + entry.offset;

	// Rygorystyczna kontrola granic (bounds check) wirtualnej mapy pliku
	if (offset > m_file.size() || entry.compressed_size > m_file.size() - offset)
	{
		return false;
	}

	// Ochrona przed przepelnieniem i nierealistycznymi rozmiarami pliku (> 1GB)
	if (entry.file_size > 1024ULL * 1024ULL * 1024ULL)
	{
		return false;
	}

	const uint8_t* pRawSource = reinterpret_cast<const uint8_t*>(m_file.data()) + offset;

	// 1. Sciezka 100% Zero-Copy: plik nieszyfrowany i nieskompresowany w archiwum
	if (IsZeroCopyEligible(entry))
	{
		result.SetView(std::span<const uint8_t>(pRawSource, static_cast<size_t>(entry.file_size)));
		return true;
	}

	ZSTD_DCtx* dctx = GetThreadLocalZSTDContext();

	switch (entry.encryption)
	{
		case 0: {
			// Plik skompresowany ZSTD bez szyfrowania
			// Alokacja bezposrednio w pamieci wirtualnej VirtualAlloc (zero fragmentacji sterty CRT)
			if (!result.AllocateVirtual(static_cast<size_t>(entry.file_size))) {
				return false;
			}

			size_t decompressed_size = ZSTD_decompressDCtx(
				dctx,
				result.data(),
				result.size(),
				pRawSource,
				static_cast<size_t>(entry.compressed_size)
			);

			if (ZSTD_isError(decompressed_size) || decompressed_size != entry.file_size) {
				result.clear();
				return false;
			}
		} break;

		case 1: {
			// Plik zaszyfrowany XChaCha20 i skompresowany ZSTD
			if (entry.compressed_size == entry.file_size)
			{
				// Plik zaszyfrowany, ale nieskompresowany: deszyfrowanie bezposrednio w buforze docelowym
				if (!result.AllocateVirtual(static_cast<size_t>(entry.file_size))) {
					return false;
				}
				std::memcpy(result.data(), pRawSource, static_cast<size_t>(entry.file_size));
				DecryptData(result.data(), static_cast<size_t>(entry.file_size), entry.nonce);
				return true;
			}

			// Uzycie thread-local scratch buffera zamiast alokacji vectora na stercie
			uint8_t* pScratch = s_decryptScratch.EnsureCapacity(static_cast<size_t>(entry.compressed_size));
			std::memcpy(pScratch, pRawSource, static_cast<size_t>(entry.compressed_size));
			DecryptData(pScratch, static_cast<size_t>(entry.compressed_size), entry.nonce);

			if (!result.AllocateVirtual(static_cast<size_t>(entry.file_size))) {
				return false;
			}

			size_t decompressed_size = ZSTD_decompressDCtx(
				dctx,
				result.data(),
				result.size(),
				pScratch,
				static_cast<size_t>(entry.compressed_size)
			);

			if (ZSTD_isError(decompressed_size) || decompressed_size != entry.file_size) {
				result.clear();
				return false;
			}
		} break;

		default: return false;
	}

	return true;
}
