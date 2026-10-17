#include "StdAfx.h"
#include "RenderResourceRegistry.h"
#include <cstring>

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
	m_textureToHandle.clear();
	m_freeSlots.clear();
	m_slots.clear();

	// Rezerwacja slotu 0 jako nieprawidlowego (kInvalidTextureHandle = 0)
	m_slots.resize(1);
	m_slots[0].generation = 0;
	m_slots[0].occupied = false;
	m_slots[0].pTexture = nullptr;
	m_slots[0].width = 0;
	m_slots[0].height = 0;
	m_slots[0].format = D3DFMT_UNKNOWN;
	m_slots[0].debugName.clear();
}

TRenderTextureHandle CRenderResourceRegistry::RegisterTexture(IDirect3DTexture9* pTexture, const char* debugName)
{
	if (!pTexture)
		return kInvalidTextureHandle;

	std::lock_guard<std::mutex> lock(m_mutex);

	// Jesli tekstura jest juz zarejestrowana, zwroc istniejacy uchwyt (idempotentnosc)
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

	// Przydzial indeksu slotu: z listy wolnych lub rozszerzenie wektora
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
		m_slots.back().generation = 1; // Poczatkowa generacja dla nowo utworzonego slotu
	}

	SSlot& slot = m_slots[slotIndex];
	slot.occupied = true;
	slot.pTexture = pTexture;
	slot.debugName = debugName ? debugName : "";
	slot.width = 0;
	slot.height = 0;
	slot.format = D3DFMT_UNKNOWN;

	// Odczyt metadanych tekstury z Direct3D 9
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

	const TRenderTextureHandle handle = RenderResourceMakeHandle(slotIndex, slot.generation);
	m_textureToHandle[pTexture] = handle;
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
		return; // Nieprawidlowy lub przestarzaly uchwyt

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

	// Inkrementacja generacji przy zwolnieniu (12 bitow). Generacja nigdy nie przyjmuje 0.
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
