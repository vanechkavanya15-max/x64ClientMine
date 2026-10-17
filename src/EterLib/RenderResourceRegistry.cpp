#include "StdAfx.h"
#include "RenderResourceRegistry.h"
#include "EterBase/CRC32.h"
#include <cstring>
#include <algorithm>

CRenderResourceRegistry::CRenderResourceRegistry()
{
	Reset();
}

CRenderResourceRegistry::~CRenderResourceRegistry()
{
	Reset();
}

void CRenderResourceRegistry::Reset()
{
	std::lock_guard<std::mutex> lock(m_mutex);

	// Tekstury
	m_textureToHandle.clear();
	m_freeSlots.clear();
	m_slots.clear();
	m_slots.resize(1);
	m_slots[0].generation = 0;
	m_slots[0].occupied = false;
	m_slots[0].pTexture = nullptr;
	m_slots[0].width = 0;
	m_slots[0].height = 0;
	m_slots[0].format = D3DFMT_UNKNOWN;
	m_slots[0].debugName.clear();
	m_slots[0].mipCopies.clear();
	m_slots[0].mip0Crc32 = 0;
	m_slots[0].hasCpuCopy = false;

	// Bufory wierzcholkow
	m_vbToHandle.clear();
	m_vbFreeSlots.clear();
	m_vbSlots.clear();
	m_vbSlots.resize(1);
	m_vbSlots[0].generation = 0;
	m_vbSlots[0].occupied = false;
	m_vbSlots[0].pVB = nullptr;
	m_vbSlots[0].size = 0;
	m_vbSlots[0].fvf = 0;
	m_vbSlots[0].stride = 0;
	m_vbSlots[0].vertexCount = 0;
	m_vbSlots[0].usage = 0;
	m_vbSlots[0].pool = D3DPOOL_DEFAULT;
	m_vbSlots[0].debugName.clear();
	m_vbSlots[0].cpuCopy.clear();
	m_vbSlots[0].crc32 = 0;
	m_vbSlots[0].isStatic = true;

	// Bufory indeksow
	m_ibToHandle.clear();
	m_ibFreeSlots.clear();
	m_ibSlots.clear();
	m_ibSlots.resize(1);
	m_ibSlots[0].generation = 0;
	m_ibSlots[0].occupied = false;
	m_ibSlots[0].pIB = nullptr;
	m_ibSlots[0].size = 0;
	m_ibSlots[0].format = D3DFMT_INDEX16;
	m_ibSlots[0].indexCount = 0;
	m_ibSlots[0].usage = 0;
	m_ibSlots[0].pool = D3DPOOL_DEFAULT;
	m_ibSlots[0].debugName.clear();
	m_ibSlots[0].cpuCopy.clear();
	m_ibSlots[0].crc32 = 0;
	m_ibSlots[0].isStatic = true;
}

// =========================================================================
// IMPLEMENTACJA DLA TEKSTUR
// =========================================================================

TRenderTextureHandle CRenderResourceRegistry::RegisterTexture(IDirect3DTexture9* pTexture, const char* debugName)
{
	if (!pTexture)
		return kInvalidTextureHandle;

	std::lock_guard<std::mutex> lock(m_mutex);

	auto it = m_textureToHandle.find(pTexture);
	if (it != m_textureToHandle.end())
	{
		const TRenderTextureHandle existingHandle = it->second;
		const uint32_t slotIndex = RenderResourceGetSlotIndex(existingHandle);
		if (slotIndex > 0 && slotIndex < m_slots.size() && m_slots[slotIndex].occupied)
		{
			if (debugName && *debugName && m_slots[slotIndex].debugName.empty())
			{
				m_slots[slotIndex].debugName = debugName;
			}
			return existingHandle;
		}
	}

	uint32_t slotIndex = 0;
	if (!m_freeSlots.empty())
	{
		slotIndex = m_freeSlots.back();
		m_freeSlots.pop_back();
	}
	else
	{
		if (m_slots.size() > kTextureIndexMask)
		{
			TraceError("CRenderResourceRegistry::RegisterTexture: Limit slotow tekstur przekroczony (max %u)", kTextureIndexMask);
			return kInvalidTextureHandle;
		}

		slotIndex = static_cast<uint32_t>(m_slots.size());
		m_slots.emplace_back();
		m_slots.back().generation = 1;
	}

	SSlot& slot = m_slots[slotIndex];
	slot.occupied = true;
	slot.pTexture = pTexture;
	slot.debugName = debugName ? debugName : "";
	slot.width = 0;
	slot.height = 0;
	slot.format = D3DFMT_UNKNOWN;
	slot.mipCopies.clear();
	slot.mip0Crc32 = 0;
	slot.hasCpuCopy = false;

	D3DSURFACE_DESC desc;
	std::memset(&desc, 0, sizeof(desc));
	if (SUCCEEDED(pTexture->GetLevelDesc(0, &desc)))
	{
		slot.width = desc.Width;
		slot.height = desc.Height;
		slot.format = desc.Format;
	}

	const TRenderTextureHandle handle = RenderResourceMakeHandle(slotIndex, slot.generation);
	m_textureToHandle[pTexture] = handle;
	return handle;
}

TRenderTextureHandle CRenderResourceRegistry::RegisterTextureWithInfo(
	IDirect3DTexture9* pTexture,
	uint32_t width,
	uint32_t height,
	D3DFORMAT format,
	const char* debugName)
{
	if (!pTexture)
		return kInvalidTextureHandle;

	std::lock_guard<std::mutex> lock(m_mutex);

	auto it = m_textureToHandle.find(pTexture);
	if (it != m_textureToHandle.end())
	{
		const TRenderTextureHandle existingHandle = it->second;
		const uint32_t slotIndex = RenderResourceGetSlotIndex(existingHandle);
		if (slotIndex > 0 && slotIndex < m_slots.size() && m_slots[slotIndex].occupied)
		{
			SSlot& slot = m_slots[slotIndex];
			slot.width = width;
			slot.height = height;
			slot.format = format;
			if (debugName && *debugName)
				slot.debugName = debugName;
			return existingHandle;
		}
	}

	uint32_t slotIndex = 0;
	if (!m_freeSlots.empty())
	{
		slotIndex = m_freeSlots.back();
		m_freeSlots.pop_back();
	}
	else
	{
		if (m_slots.size() > kTextureIndexMask)
		{
			TraceError("CRenderResourceRegistry::RegisterTextureWithInfo: Limit slotow przekroczony");
			return kInvalidTextureHandle;
		}

		slotIndex = static_cast<uint32_t>(m_slots.size());
		m_slots.emplace_back();
		m_slots.back().generation = 1;
	}

	SSlot& slot = m_slots[slotIndex];
	slot.occupied = true;
	slot.pTexture = pTexture;
	slot.debugName = debugName ? debugName : "";
	slot.width = width;
	slot.height = height;
	slot.format = format;
	slot.mipCopies.clear();
	slot.mip0Crc32 = 0;
	slot.hasCpuCopy = false;

	const TRenderTextureHandle handle = RenderResourceMakeHandle(slotIndex, slot.generation);
	m_textureToHandle[pTexture] = handle;
	return handle;
}

TRenderTextureHandle CRenderResourceRegistry::RegisterTextureWithPixels(
	IDirect3DTexture9* pTexture,
	uint32_t width,
	uint32_t height,
	D3DFORMAT format,
	const void* pMip0Pixels,
	size_t mip0Size,
	uint32_t pitch,
	const char* debugName)
{
	TRenderTextureHandle handle = RegisterTextureWithInfo(pTexture, width, height, format, debugName);
	if (handle != kInvalidTextureHandle && pMip0Pixels && mip0Size > 0)
	{
		AttachTexturePixelCopy(handle, 0, width, height, pitch, pMip0Pixels, mip0Size);
	}
	return handle;
}

void CRenderResourceRegistry::UnregisterTexture(TRenderTextureHandle handle)
{
	if (handle == kInvalidTextureHandle)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	UnregisterTextureLocked(handle);
}

void CRenderResourceRegistry::UnregisterTexture(IDirect3DTexture9* pTexture)
{
	if (!pTexture)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	auto it = m_textureToHandle.find(pTexture);
	if (it != m_textureToHandle.end())
	{
		const TRenderTextureHandle handle = it->second;
		UnregisterTextureLocked(handle);
	}
}

void CRenderResourceRegistry::UnregisterTextureLocked(TRenderTextureHandle handle)
{
	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return;

	SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return;

	if (slot.pTexture)
	{
		m_textureToHandle.erase(slot.pTexture);
		slot.pTexture = nullptr;
	}

	slot.occupied = false;
	slot.width = 0;
	slot.height = 0;
	slot.format = D3DFMT_UNKNOWN;
	slot.debugName.clear();
	slot.mipCopies.clear();
	slot.mip0Crc32 = 0;
	slot.hasCpuCopy = false;

	slot.generation = (slot.generation + 1) & kTextureGenerationMask;
	if (slot.generation == 0)
		slot.generation = 1;

	m_freeSlots.push_back(slotIndex);
}

IDirect3DTexture9* CRenderResourceRegistry::ResolveTexture(TRenderTextureHandle handle) const
{
	if (handle == kInvalidTextureHandle)
		return nullptr;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return nullptr;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return nullptr;

	return slot.pTexture;
}

bool CRenderResourceRegistry::GetTextureInfo(TRenderTextureHandle handle, STextureResourceInfo* pOutInfo) const
{
	if (!pOutInfo || handle == kInvalidTextureHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	pOutInfo->width = slot.width;
	pOutInfo->height = slot.height;
	pOutInfo->format = slot.format;
	pOutInfo->debugName = slot.debugName;
	pOutInfo->pTexture = slot.pTexture;
	pOutInfo->mipCopies = slot.mipCopies;
	pOutInfo->mip0Crc32 = slot.mip0Crc32;
	pOutInfo->hasCpuCopy = slot.hasCpuCopy;
	return true;
}

bool CRenderResourceRegistry::GetTextureInfo(TRenderTextureHandle handle, STextureResourceInfo& outInfo) const
{
	return GetTextureInfo(handle, &outInfo);
}

uint32_t CRenderResourceRegistry::GetWidth(TRenderTextureHandle handle) const
{
	STextureResourceInfo info;
	return GetTextureInfo(handle, &info) ? info.width : 0;
}

uint32_t CRenderResourceRegistry::GetHeight(TRenderTextureHandle handle) const
{
	STextureResourceInfo info;
	return GetTextureInfo(handle, &info) ? info.height : 0;
}

D3DFORMAT CRenderResourceRegistry::GetFormat(TRenderTextureHandle handle) const
{
	STextureResourceInfo info;
	return GetTextureInfo(handle, &info) ? info.format : D3DFMT_UNKNOWN;
}

std::string CRenderResourceRegistry::GetDebugName(TRenderTextureHandle handle) const
{
	STextureResourceInfo info;
	return GetTextureInfo(handle, &info) ? info.debugName : "";
}

TRenderTextureHandle CRenderResourceRegistry::FindTextureHandle(IDirect3DTexture9* pTexture) const
{
	if (!pTexture)
		return kInvalidTextureHandle;

	std::lock_guard<std::mutex> lock(m_mutex);
	auto it = m_textureToHandle.find(pTexture);
	if (it != m_textureToHandle.end())
		return it->second;

	return kInvalidTextureHandle;
}

bool CRenderResourceRegistry::IsValidHandle(TRenderTextureHandle handle) const
{
	return ResolveTexture(handle) != nullptr;
}

size_t CRenderResourceRegistry::GetActiveTextureCount() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_textureToHandle.size();
}

size_t CRenderResourceRegistry::GetTotalSlotCapacity() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_slots.size();
}

bool CRenderResourceRegistry::AttachTexturePixelCopy(
	TRenderTextureHandle handle,
	uint32_t mipLevel,
	uint32_t width,
	uint32_t height,
	uint32_t pitch,
	const void* pPixels,
	size_t size)
{
	if (handle == kInvalidTextureHandle || !pPixels || size == 0)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	const uint32_t crc = GetCRC32(reinterpret_cast<const char*>(pPixels), size);

	// Sprawdz czy mip juz istnieje w liscie (zastap) czy dopisz nowy
	bool found = false;
	for (auto& mip : slot.mipCopies)
	{
		if (mip.level == mipLevel)
		{
			mip.width = width;
			mip.height = height;
			mip.pitch = pitch;
			mip.dataSize = static_cast<uint32_t>(size);
			mip.crc32 = crc;
			mip.pixels.assign(static_cast<const uint8_t*>(pPixels), static_cast<const uint8_t*>(pPixels) + size);
			found = true;
			break;
		}
	}

	if (!found)
	{
		STextureMipCpuCopy mip;
		mip.level = mipLevel;
		mip.width = width;
		mip.height = height;
		mip.pitch = pitch;
		mip.dataSize = static_cast<uint32_t>(size);
		mip.crc32 = crc;
		mip.pixels.assign(static_cast<const uint8_t*>(pPixels), static_cast<const uint8_t*>(pPixels) + size);
		slot.mipCopies.push_back(std::move(mip));
	}

	if (mipLevel == 0)
	{
		slot.mip0Crc32 = crc;
		slot.hasCpuCopy = true;
	}

	return true;
}

bool CRenderResourceRegistry::AttachTexturePixelCopy(
	IDirect3DTexture9* pTexture,
	uint32_t mipLevel,
	uint32_t width,
	uint32_t height,
	uint32_t pitch,
	const void* pPixels,
	size_t size)
{
	if (!pTexture)
		return false;

	TRenderTextureHandle handle = FindTextureHandle(pTexture);
	if (handle == kInvalidTextureHandle)
	{
		handle = RegisterTexture(pTexture, "AutoStagedTex");
	}

	return AttachTexturePixelCopy(handle, mipLevel, width, height, pitch, pPixels, size);
}

bool CRenderResourceRegistry::GetTextureMipPixelCopy(TRenderTextureHandle handle, uint32_t mipLevel, STextureMipCpuCopy* pOutMip) const
{
	if (!pOutMip || handle == kInvalidTextureHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	for (const auto& mip : slot.mipCopies)
	{
		if (mip.level == mipLevel)
		{
			*pOutMip = mip;
			return true;
		}
	}

	return false;
}

bool CRenderResourceRegistry::GetTextureMip0Pixels(TRenderTextureHandle handle, std::vector<uint8_t>& outPixels, uint32_t* pOutCrc32) const
{
	STextureMipCpuCopy mip0;
	if (!GetTextureMipPixelCopy(handle, 0, &mip0))
		return false;

	outPixels = std::move(mip0.pixels);
	if (pOutCrc32)
		*pOutCrc32 = mip0.crc32;

	return true;
}

uint32_t CRenderResourceRegistry::GetTextureMip0Crc32(TRenderTextureHandle handle) const
{
	if (handle == kInvalidTextureHandle)
		return 0;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return 0;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return 0;

	return slot.mip0Crc32;
}

bool CRenderResourceRegistry::VerifyTextureMip0Crc(TRenderTextureHandle handle, uint32_t expectedCrc) const
{
	const uint32_t actualCrc = GetTextureMip0Crc32(handle);
	return (actualCrc != 0) && (actualCrc == expectedCrc);
}

size_t CRenderResourceRegistry::GetTextureMipCount(TRenderTextureHandle handle) const
{
	if (handle == kInvalidTextureHandle)
		return 0;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return 0;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return 0;

	return slot.mipCopies.size();
}

bool CRenderResourceRegistry::HasTextureCpuCopy(TRenderTextureHandle handle) const
{
	if (handle == kInvalidTextureHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_slots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SSlot& slot = m_slots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	return slot.hasCpuCopy;
}

// =========================================================================
// IMPLEMENTACJA DLA BUFOROW WIERZCHOLKOW (VB)
// =========================================================================

TRenderVertexBufferHandle CRenderResourceRegistry::RegisterVertexBuffer(
	IDirect3DVertexBuffer9* pVB,
	uint32_t size,
	uint32_t fvf,
	const void* pCpuData,
	const char* debugName)
{
	return RegisterVertexBufferWithInfo(pVB, size, fvf, 0, 0, 0, D3DPOOL_DEFAULT, pCpuData, debugName);
}

TRenderVertexBufferHandle CRenderResourceRegistry::RegisterVertexBufferWithInfo(
	IDirect3DVertexBuffer9* pVB,
	uint32_t size,
	uint32_t fvf,
	uint32_t stride,
	uint32_t vertexCount,
	DWORD usage,
	D3DPOOL pool,
	const void* pCpuData,
	const char* debugName)
{
	if (!pVB)
		return kInvalidVertexBufferHandle;

	std::lock_guard<std::mutex> lock(m_mutex);

	auto it = m_vbToHandle.find(pVB);
	if (it != m_vbToHandle.end())
	{
		const TRenderVertexBufferHandle existingHandle = it->second;
		const uint32_t slotIndex = RenderResourceGetSlotIndex(existingHandle);
		if (slotIndex > 0 && slotIndex < m_vbSlots.size() && m_vbSlots[slotIndex].occupied)
		{
			SVBSlot& slot = m_vbSlots[slotIndex];
			slot.size = size;
			slot.fvf = fvf;
			slot.stride = stride;
			slot.vertexCount = vertexCount;
			slot.usage = usage;
			slot.pool = pool;
			if (debugName && *debugName)
				slot.debugName = debugName;
			if (pCpuData && size > 0)
			{
				slot.cpuCopy.assign(static_cast<const uint8_t*>(pCpuData), static_cast<const uint8_t*>(pCpuData) + size);
				slot.crc32 = GetCRC32(reinterpret_cast<const char*>(pCpuData), size);
			}
			return existingHandle;
		}
	}

	uint32_t slotIndex = 0;
	if (!m_vbFreeSlots.empty())
	{
		slotIndex = m_vbFreeSlots.back();
		m_vbFreeSlots.pop_back();
	}
	else
	{
		if (m_vbSlots.size() > kTextureIndexMask)
		{
			TraceError("CRenderResourceRegistry::RegisterVertexBuffer: Limit slotow VB przekroczony");
			return kInvalidVertexBufferHandle;
		}

		slotIndex = static_cast<uint32_t>(m_vbSlots.size());
		m_vbSlots.emplace_back();
		m_vbSlots.back().generation = 1;
	}

	SVBSlot& slot = m_vbSlots[slotIndex];
	slot.occupied = true;
	slot.pVB = pVB;
	slot.size = size;
	slot.fvf = fvf;
	slot.stride = stride;
	slot.vertexCount = vertexCount;
	slot.usage = usage;
	slot.pool = pool;
	slot.debugName = debugName ? debugName : "";
	slot.isStatic = (usage & D3DUSAGE_DYNAMIC) == 0;
	slot.cpuCopy.clear();
	slot.crc32 = 0;

	if (pCpuData && size > 0)
	{
		slot.cpuCopy.assign(static_cast<const uint8_t*>(pCpuData), static_cast<const uint8_t*>(pCpuData) + size);
		slot.crc32 = GetCRC32(reinterpret_cast<const char*>(pCpuData), size);
	}

	const TRenderVertexBufferHandle handle = RenderResourceMakeHandle(slotIndex, slot.generation);
	m_vbToHandle[pVB] = handle;
	return handle;
}

void CRenderResourceRegistry::UnregisterVertexBuffer(TRenderVertexBufferHandle handle)
{
	if (handle == kInvalidVertexBufferHandle)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	UnregisterVertexBufferLocked(handle);
}

void CRenderResourceRegistry::UnregisterVertexBuffer(IDirect3DVertexBuffer9* pVB)
{
	if (!pVB)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	auto it = m_vbToHandle.find(pVB);
	if (it != m_vbToHandle.end())
	{
		UnregisterVertexBufferLocked(it->second);
	}
}

void CRenderResourceRegistry::UnregisterVertexBufferLocked(TRenderVertexBufferHandle handle)
{
	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_vbSlots.size())
		return;

	SVBSlot& slot = m_vbSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return;

	if (slot.pVB)
	{
		m_vbToHandle.erase(slot.pVB);
		slot.pVB = nullptr;
	}

	slot.occupied = false;
	slot.size = 0;
	slot.fvf = 0;
	slot.stride = 0;
	slot.vertexCount = 0;
	slot.usage = 0;
	slot.debugName.clear();
	slot.cpuCopy.clear();
	slot.crc32 = 0;

	slot.generation = (slot.generation + 1) & kTextureGenerationMask;
	if (slot.generation == 0)
		slot.generation = 1;

	m_vbFreeSlots.push_back(slotIndex);
}

IDirect3DVertexBuffer9* CRenderResourceRegistry::ResolveVertexBuffer(TRenderVertexBufferHandle handle) const
{
	if (handle == kInvalidVertexBufferHandle)
		return nullptr;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_vbSlots.size())
		return nullptr;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SVBSlot& slot = m_vbSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return nullptr;

	return slot.pVB;
}

bool CRenderResourceRegistry::SetVertexBufferCpuCopy(TRenderVertexBufferHandle handle, const void* pData, size_t size)
{
	if (handle == kInvalidVertexBufferHandle || !pData || size == 0)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_vbSlots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	SVBSlot& slot = m_vbSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	slot.cpuCopy.assign(static_cast<const uint8_t*>(pData), static_cast<const uint8_t*>(pData) + size);
	slot.size = static_cast<uint32_t>(size);
	slot.crc32 = GetCRC32(reinterpret_cast<const char*>(pData), size);
	return true;
}

bool CRenderResourceRegistry::SetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, const void* pData, size_t size)
{
	if (!pVB)
		return false;

	TRenderVertexBufferHandle handle = FindVertexBufferHandle(pVB);
	if (handle == kInvalidVertexBufferHandle)
	{
		handle = RegisterVertexBuffer(pVB, static_cast<uint32_t>(size), 0, pData, "AutoVB");
		return handle != kInvalidVertexBufferHandle;
	}

	return SetVertexBufferCpuCopy(handle, pData, size);
}

bool CRenderResourceRegistry::GetVertexBufferCpuCopy(TRenderVertexBufferHandle handle, std::vector<uint8_t>& outData) const
{
	if (handle == kInvalidVertexBufferHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_vbSlots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SVBSlot& slot = m_vbSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation || slot.cpuCopy.empty())
		return false;

	outData = slot.cpuCopy;
	return true;
}

bool CRenderResourceRegistry::GetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, std::vector<uint8_t>& outData) const
{
	if (!pVB)
		return false;

	TRenderVertexBufferHandle handle = FindVertexBufferHandle(pVB);
	if (handle == kInvalidVertexBufferHandle)
		return false;

	return GetVertexBufferCpuCopy(handle, outData);
}

bool CRenderResourceRegistry::GetVertexBufferInfo(TRenderVertexBufferHandle handle, SVertexBufferResourceInfo* pOutInfo) const
{
	if (!pOutInfo || handle == kInvalidVertexBufferHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_vbSlots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SVBSlot& slot = m_vbSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	pOutInfo->pVB = slot.pVB;
	pOutInfo->size = slot.size;
	pOutInfo->fvf = slot.fvf;
	pOutInfo->stride = slot.stride;
	pOutInfo->vertexCount = slot.vertexCount;
	pOutInfo->usage = slot.usage;
	pOutInfo->pool = slot.pool;
	pOutInfo->debugName = slot.debugName;
	pOutInfo->cpuCopy = slot.cpuCopy;
	pOutInfo->crc32 = slot.crc32;
	pOutInfo->isStatic = slot.isStatic;
	return true;
}

TRenderVertexBufferHandle CRenderResourceRegistry::FindVertexBufferHandle(IDirect3DVertexBuffer9* pVB) const
{
	if (!pVB)
		return kInvalidVertexBufferHandle;

	std::lock_guard<std::mutex> lock(m_mutex);
	auto it = m_vbToHandle.find(pVB);
	if (it != m_vbToHandle.end())
		return it->second;

	return kInvalidVertexBufferHandle;
}

uint32_t CRenderResourceRegistry::GetVertexBufferCrc32(TRenderVertexBufferHandle handle) const
{
	if (handle == kInvalidVertexBufferHandle)
		return 0;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_vbSlots.size())
		return 0;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SVBSlot& slot = m_vbSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return 0;

	return slot.crc32;
}

bool CRenderResourceRegistry::VerifyVertexBufferCrc(TRenderVertexBufferHandle handle, uint32_t expectedCrc) const
{
	const uint32_t actualCrc = GetVertexBufferCrc32(handle);
	return (actualCrc != 0) && (actualCrc == expectedCrc);
}

size_t CRenderResourceRegistry::GetActiveVertexBufferCount() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_vbToHandle.size();
}

// =========================================================================
// IMPLEMENTACJA DLA BUFOROW INDEKSOW (IB)
// =========================================================================

TRenderIndexBufferHandle CRenderResourceRegistry::RegisterIndexBuffer(
	IDirect3DIndexBuffer9* pIB,
	uint32_t size,
	D3DFORMAT format,
	const void* pCpuData,
	const char* debugName)
{
	return RegisterIndexBufferWithInfo(pIB, size, format, 0, 0, D3DPOOL_DEFAULT, pCpuData, debugName);
}

TRenderIndexBufferHandle CRenderResourceRegistry::RegisterIndexBufferWithInfo(
	IDirect3DIndexBuffer9* pIB,
	uint32_t size,
	D3DFORMAT format,
	uint32_t indexCount,
	DWORD usage,
	D3DPOOL pool,
	const void* pCpuData,
	const char* debugName)
{
	if (!pIB)
		return kInvalidIndexBufferHandle;

	std::lock_guard<std::mutex> lock(m_mutex);

	auto it = m_ibToHandle.find(pIB);
	if (it != m_ibToHandle.end())
	{
		const TRenderIndexBufferHandle existingHandle = it->second;
		const uint32_t slotIndex = RenderResourceGetSlotIndex(existingHandle);
		if (slotIndex > 0 && slotIndex < m_ibSlots.size() && m_ibSlots[slotIndex].occupied)
		{
			SIBSlot& slot = m_ibSlots[slotIndex];
			slot.size = size;
			slot.format = format;
			slot.indexCount = indexCount;
			slot.usage = usage;
			slot.pool = pool;
			if (debugName && *debugName)
				slot.debugName = debugName;
			if (pCpuData && size > 0)
			{
				slot.cpuCopy.assign(static_cast<const uint8_t*>(pCpuData), static_cast<const uint8_t*>(pCpuData) + size);
				slot.crc32 = GetCRC32(reinterpret_cast<const char*>(pCpuData), size);
			}
			return existingHandle;
		}
	}

	uint32_t slotIndex = 0;
	if (!m_ibFreeSlots.empty())
	{
		slotIndex = m_ibFreeSlots.back();
		m_ibFreeSlots.pop_back();
	}
	else
	{
		if (m_ibSlots.size() > kTextureIndexMask)
		{
			TraceError("CRenderResourceRegistry::RegisterIndexBuffer: Limit slotow IB przekroczony");
			return kInvalidIndexBufferHandle;
		}

		slotIndex = static_cast<uint32_t>(m_ibSlots.size());
		m_ibSlots.emplace_back();
		m_ibSlots.back().generation = 1;
	}

	SIBSlot& slot = m_ibSlots[slotIndex];
	slot.occupied = true;
	slot.pIB = pIB;
	slot.size = size;
	slot.format = format;
	slot.indexCount = indexCount;
	slot.usage = usage;
	slot.pool = pool;
	slot.debugName = debugName ? debugName : "";
	slot.isStatic = (usage & D3DUSAGE_DYNAMIC) == 0;
	slot.cpuCopy.clear();
	slot.crc32 = 0;

	if (pCpuData && size > 0)
	{
		slot.cpuCopy.assign(static_cast<const uint8_t*>(pCpuData), static_cast<const uint8_t*>(pCpuData) + size);
		slot.crc32 = GetCRC32(reinterpret_cast<const char*>(pCpuData), size);
	}

	const TRenderIndexBufferHandle handle = RenderResourceMakeHandle(slotIndex, slot.generation);
	m_ibToHandle[pIB] = handle;
	return handle;
}

void CRenderResourceRegistry::UnregisterIndexBuffer(TRenderIndexBufferHandle handle)
{
	if (handle == kInvalidIndexBufferHandle)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	UnregisterIndexBufferLocked(handle);
}

void CRenderResourceRegistry::UnregisterIndexBuffer(IDirect3DIndexBuffer9* pIB)
{
	if (!pIB)
		return;

	std::lock_guard<std::mutex> lock(m_mutex);
	auto it = m_ibToHandle.find(pIB);
	if (it != m_ibToHandle.end())
	{
		UnregisterIndexBufferLocked(it->second);
	}
}

void CRenderResourceRegistry::UnregisterIndexBufferLocked(TRenderIndexBufferHandle handle)
{
	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_ibSlots.size())
		return;

	SIBSlot& slot = m_ibSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return;

	if (slot.pIB)
	{
		m_ibToHandle.erase(slot.pIB);
		slot.pIB = nullptr;
	}

	slot.occupied = false;
	slot.size = 0;
	slot.format = D3DFMT_INDEX16;
	slot.indexCount = 0;
	slot.usage = 0;
	slot.debugName.clear();
	slot.cpuCopy.clear();
	slot.crc32 = 0;

	slot.generation = (slot.generation + 1) & kTextureGenerationMask;
	if (slot.generation == 0)
		slot.generation = 1;

	m_ibFreeSlots.push_back(slotIndex);
}

IDirect3DIndexBuffer9* CRenderResourceRegistry::ResolveIndexBuffer(TRenderIndexBufferHandle handle) const
{
	if (handle == kInvalidIndexBufferHandle)
		return nullptr;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_ibSlots.size())
		return nullptr;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SIBSlot& slot = m_ibSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return nullptr;

	return slot.pIB;
}

bool CRenderResourceRegistry::SetIndexBufferCpuCopy(TRenderIndexBufferHandle handle, const void* pData, size_t size)
{
	if (handle == kInvalidIndexBufferHandle || !pData || size == 0)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_ibSlots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	SIBSlot& slot = m_ibSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	slot.cpuCopy.assign(static_cast<const uint8_t*>(pData), static_cast<const uint8_t*>(pData) + size);
	slot.size = static_cast<uint32_t>(size);
	slot.crc32 = GetCRC32(reinterpret_cast<const char*>(pData), size);
	return true;
}

bool CRenderResourceRegistry::SetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, const void* pData, size_t size)
{
	if (!pIB)
		return false;

	TRenderIndexBufferHandle handle = FindIndexBufferHandle(pIB);
	if (handle == kInvalidIndexBufferHandle)
	{
		handle = RegisterIndexBuffer(pIB, static_cast<uint32_t>(size), D3DFMT_INDEX16, pData, "AutoIB");
		return handle != kInvalidIndexBufferHandle;
	}

	return SetIndexBufferCpuCopy(handle, pData, size);
}

bool CRenderResourceRegistry::GetIndexBufferCpuCopy(TRenderIndexBufferHandle handle, std::vector<uint8_t>& outData) const
{
	if (handle == kInvalidIndexBufferHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_ibSlots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SIBSlot& slot = m_ibSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation || slot.cpuCopy.empty())
		return false;

	outData = slot.cpuCopy;
	return true;
}

bool CRenderResourceRegistry::GetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, std::vector<uint8_t>& outData) const
{
	if (!pIB)
		return false;

	TRenderIndexBufferHandle handle = FindIndexBufferHandle(pIB);
	if (handle == kInvalidIndexBufferHandle)
		return false;

	return GetIndexBufferCpuCopy(handle, outData);
}

bool CRenderResourceRegistry::GetIndexBufferInfo(TRenderIndexBufferHandle handle, SIndexBufferResourceInfo* pOutInfo) const
{
	if (!pOutInfo || handle == kInvalidIndexBufferHandle)
		return false;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_ibSlots.size())
		return false;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SIBSlot& slot = m_ibSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return false;

	pOutInfo->pIB = slot.pIB;
	pOutInfo->size = slot.size;
	pOutInfo->format = slot.format;
	pOutInfo->indexCount = slot.indexCount;
	pOutInfo->usage = slot.usage;
	pOutInfo->pool = slot.pool;
	pOutInfo->debugName = slot.debugName;
	pOutInfo->cpuCopy = slot.cpuCopy;
	pOutInfo->crc32 = slot.crc32;
	pOutInfo->isStatic = slot.isStatic;
	return true;
}

TRenderIndexBufferHandle CRenderResourceRegistry::FindIndexBufferHandle(IDirect3DIndexBuffer9* pIB) const
{
	if (!pIB)
		return kInvalidIndexBufferHandle;

	std::lock_guard<std::mutex> lock(m_mutex);
	auto it = m_ibToHandle.find(pIB);
	if (it != m_ibToHandle.end())
		return it->second;

	return kInvalidIndexBufferHandle;
}

uint32_t CRenderResourceRegistry::GetIndexBufferCrc32(TRenderIndexBufferHandle handle) const
{
	if (handle == kInvalidIndexBufferHandle)
		return 0;

	const uint32_t slotIndex = RenderResourceGetSlotIndex(handle);
	const uint32_t generation = RenderResourceGetGeneration(handle);

	if (slotIndex == 0 || slotIndex >= m_ibSlots.size())
		return 0;

	std::lock_guard<std::mutex> lock(m_mutex);
	const SIBSlot& slot = m_ibSlots[slotIndex];
	if (!slot.occupied || slot.generation != generation)
		return 0;

	return slot.crc32;
}

bool CRenderResourceRegistry::VerifyIndexBufferCrc(TRenderIndexBufferHandle handle, uint32_t expectedCrc) const
{
	const uint32_t actualCrc = GetIndexBufferCrc32(handle);
	return (actualCrc != 0) && (actualCrc == expectedCrc);
}

size_t CRenderResourceRegistry::GetActiveIndexBufferCount() const
{
	std::lock_guard<std::mutex> lock(m_mutex);
	return m_ibToHandle.size();
}

// =========================================================================
// STATYCZNE METODY BEZPIECZNE (SAFE WRAPPERS)
// =========================================================================

TRenderTextureHandle CRenderResourceRegistry::SafeRegisterTexture(IDirect3DTexture9* pTexture, const char* debugName)
{
	if (InstancePtr())
		return Instance().RegisterTexture(pTexture, debugName);
	return kInvalidTextureHandle;
}

void CRenderResourceRegistry::SafeUnregisterTexture(TRenderTextureHandle handle)
{
	if (InstancePtr())
		Instance().UnregisterTexture(handle);
}

void CRenderResourceRegistry::SafeUnregisterTexture(IDirect3DTexture9* pTexture)
{
	if (InstancePtr())
		Instance().UnregisterTexture(pTexture);
}

IDirect3DTexture9* CRenderResourceRegistry::SafeResolveTexture(TRenderTextureHandle handle)
{
	if (InstancePtr())
		return Instance().ResolveTexture(handle);
	return nullptr;
}

bool CRenderResourceRegistry::SafeAttachTexturePixelCopy(
	IDirect3DTexture9* pTexture,
	uint32_t mipLevel,
	uint32_t width,
	uint32_t height,
	uint32_t pitch,
	const void* pPixels,
	size_t size)
{
	if (InstancePtr())
		return Instance().AttachTexturePixelCopy(pTexture, mipLevel, width, height, pitch, pPixels, size);
	return false;
}

bool CRenderResourceRegistry::SafeAttachTexturePixelCopy(
	TRenderTextureHandle handle,
	uint32_t mipLevel,
	uint32_t width,
	uint32_t height,
	uint32_t pitch,
	const void* pPixels,
	size_t size)
{
	if (InstancePtr())
		return Instance().AttachTexturePixelCopy(handle, mipLevel, width, height, pitch, pPixels, size);
	return false;
}

bool CRenderResourceRegistry::SafeGetTextureMip0Pixels(TRenderTextureHandle handle, std::vector<uint8_t>& outPixels, uint32_t* pOutCrc32)
{
	if (InstancePtr())
		return Instance().GetTextureMip0Pixels(handle, outPixels, pOutCrc32);
	return false;
}

uint32_t CRenderResourceRegistry::SafeGetTextureMip0Crc32(TRenderTextureHandle handle)
{
	if (InstancePtr())
		return Instance().GetTextureMip0Crc32(handle);
	return 0;
}

bool CRenderResourceRegistry::SafeVerifyTextureMip0Crc(TRenderTextureHandle handle, uint32_t expectedCrc)
{
	if (InstancePtr())
		return Instance().VerifyTextureMip0Crc(handle, expectedCrc);
	return false;
}

TRenderVertexBufferHandle CRenderResourceRegistry::SafeRegisterVertexBuffer(
	IDirect3DVertexBuffer9* pVB,
	uint32_t size,
	uint32_t fvf,
	const void* pCpuData,
	const char* debugName)
{
	if (InstancePtr())
		return Instance().RegisterVertexBuffer(pVB, size, fvf, pCpuData, debugName);
	return kInvalidVertexBufferHandle;
}

void CRenderResourceRegistry::SafeUnregisterVertexBuffer(TRenderVertexBufferHandle handle)
{
	if (InstancePtr())
		Instance().UnregisterVertexBuffer(handle);
}

void CRenderResourceRegistry::SafeUnregisterVertexBuffer(IDirect3DVertexBuffer9* pVB)
{
	if (InstancePtr())
		Instance().UnregisterVertexBuffer(pVB);
}

bool CRenderResourceRegistry::SafeSetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, const void* pData, size_t size)
{
	if (InstancePtr())
		return Instance().SetVertexBufferCpuCopy(pVB, pData, size);
	return false;
}

bool CRenderResourceRegistry::SafeGetVertexBufferCpuCopy(IDirect3DVertexBuffer9* pVB, std::vector<uint8_t>& outData)
{
	if (InstancePtr())
		return Instance().GetVertexBufferCpuCopy(pVB, outData);
	return false;
}

TRenderIndexBufferHandle CRenderResourceRegistry::SafeRegisterIndexBuffer(
	IDirect3DIndexBuffer9* pIB,
	uint32_t size,
	D3DFORMAT format,
	const void* pCpuData,
	const char* debugName)
{
	if (InstancePtr())
		return Instance().RegisterIndexBuffer(pIB, size, format, pCpuData, debugName);
	return kInvalidIndexBufferHandle;
}

void CRenderResourceRegistry::SafeUnregisterIndexBuffer(TRenderIndexBufferHandle handle)
{
	if (InstancePtr())
		Instance().UnregisterIndexBuffer(handle);
}

void CRenderResourceRegistry::SafeUnregisterIndexBuffer(IDirect3DIndexBuffer9* pIB)
{
	if (InstancePtr())
		Instance().UnregisterIndexBuffer(pIB);
}

bool CRenderResourceRegistry::SafeSetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, const void* pData, size_t size)
{
	if (InstancePtr())
		return Instance().SetIndexBufferCpuCopy(pIB, pData, size);
	return false;
}

bool CRenderResourceRegistry::SafeGetIndexBufferCpuCopy(IDirect3DIndexBuffer9* pIB, std::vector<uint8_t>& outData)
{
	if (InstancePtr())
		return Instance().GetIndexBufferCpuCopy(pIB, outData);
	return false;
}
