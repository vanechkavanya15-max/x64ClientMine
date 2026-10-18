#include "Pack.h"
#include "EterLib/BufferPool.h"
#include <zstd.h>
#include <filesystem>
#include <cstring>

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
}

static ZSTD_DCtx* GetThreadLocalZSTDContext()
{
	return s_zstdDCtx.ctx;
}

void CPack::DecryptData(uint8_t* data, size_t len, const uint8_t* nonce)
{
	crypto_stream_xchacha20_xor(data, data, len, nonce, PACK_KEY.data());
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
	result.resize(entry.file_size);

	size_t offset = m_header.data_begin + entry.offset;
	ZSTD_DCtx* dctx = GetThreadLocalZSTDContext();

	switch (entry.encryption)
	{
		case 0: {
			size_t decompressed_size = ZSTD_decompressDCtx(dctx, result.data(), result.size(), m_file.data() + offset, entry.compressed_size);
			if (ZSTD_isError(decompressed_size) || decompressed_size != entry.file_size) {
				return false;
			}
		} break;

		case 1: {
			std::vector<uint8_t> compressed_data;
			if (pPool) {
				compressed_data = pPool->Acquire(entry.compressed_size);
			}
			compressed_data.resize(entry.compressed_size);

			memcpy(compressed_data.data(), m_file.data() + offset, entry.compressed_size);

			DecryptData(compressed_data.data(), entry.compressed_size, entry.nonce);

			size_t decompressed_size = ZSTD_decompressDCtx(dctx, result.data(), result.size(), compressed_data.data(), compressed_data.size());

			if (pPool) {
				pPool->Release(std::move(compressed_data));
			}

			if (ZSTD_isError(decompressed_size) || decompressed_size != entry.file_size) {
				return false;
			}
		} break;

		default: return false;
	}

	return true;
}
