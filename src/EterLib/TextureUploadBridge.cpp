#include "StdAfx.h"
#include "TextureUploadBridge.h"
#include "WgpuModule.h"
#include "WgpuRenderBackend.h"
#include "RenderQueueManager.h"
#include "RenderOwnership.h"
#include <mutex>

static std::mutex s_bridgeMutex;

CTextureUploadBridge::CTextureUploadBridge() = default;

CTextureUploadBridge::~CTextureUploadBridge()
{
	Shutdown();
}

bool CTextureUploadBridge::Initialize()
{
	std::lock_guard<std::mutex> lock(s_bridgeMutex);
	return EnsureWgpuLoaded();
}

void CTextureUploadBridge::Shutdown()
{
	std::lock_guard<std::mutex> lock(s_bridgeMutex);
	m_registeredCount = 0;
}

bool CTextureUploadBridge::EnsureWgpuLoaded()
{
	CWgpuModule& module = CWgpuModule::Instance();
	if (module.IsLoaded())
		return true;

	return module.Load();
}

bool CTextureUploadBridge::IsWgpuActive() const
{
	if (RenderOwnership::IsWgpuConfigured())
		return true;

	if (CWgpuModule::Instance().IsLoaded())
		return true;

	IRenderBackend* pActive = CRenderQueueManager::Instance().GetActiveBackend();
	if (pActive && pActive == &CWgpuRenderBackend::Instance())
		return true;

	return false;
}

bool CTextureUploadBridge::RegisterTexture(
	uintptr_t dwTextureID,
	uint32_t width,
	uint32_t height,
	uint32_t format,
	const uint8_t* pData,
	size_t dataSize)
{
	if (!dwTextureID || !pData || dataSize == 0)
		return false;

	if (!IsWgpuActive())
		return false;

	CWgpuModule& module = CWgpuModule::Instance();
	if (!module.IsLoaded())
	{
		std::lock_guard<std::mutex> lock(s_bridgeMutex);
		if (!module.IsLoaded() && !EnsureWgpuLoaded())
			return false;
	}

	if (module.RegisterTexture(dwTextureID, width, height, format, pData, dataSize))
	{
		++m_registeredCount;
		return true;
	}

	return false;
}

bool CTextureUploadBridge::RegisterTexture(
	uintptr_t dwTextureID,
	uint32_t width,
	uint32_t height,
	uint32_t format,
	const void* pData,
	size_t dataSize)
{
	return RegisterTexture(
		dwTextureID,
		width,
		height,
		format,
		reinterpret_cast<const uint8_t*>(pData),
		dataSize
	);
}

bool CTextureUploadBridge::RegisterTextureArray(
	uintptr_t dwTextureID,
	uint32_t width,
	uint32_t height,
	uint32_t layers,
	uint32_t format,
	const uint8_t* pData,
	size_t dataSize)
{
	if (!dwTextureID || layers == 0)
		return false;

	if (!IsWgpuActive())
		return false;

	CWgpuModule& module = CWgpuModule::Instance();
	if (!module.IsLoaded())
	{
		std::lock_guard<std::mutex> lock(s_bridgeMutex);
		if (!module.IsLoaded() && !EnsureWgpuLoaded())
			return false;
	}

	if (module.RegisterTextureArray(dwTextureID, width, height, layers, format, pData, dataSize))
	{
		++m_registeredCount;
		return true;
	}

	return false;
}

bool CTextureUploadBridge::RegisterTextureArray(
	uintptr_t dwTextureID,
	uint32_t width,
	uint32_t height,
	uint32_t layers,
	uint32_t format,
	const void* pData,
	size_t dataSize)
{
	return RegisterTextureArray(
		dwTextureID,
		width,
		height,
		layers,
		format,
		reinterpret_cast<const uint8_t*>(pData),
		dataSize
	);
}

bool CTextureUploadBridge::UpdateTextureArrayLayer(
	uintptr_t dwTextureID,
	uint32_t layerIndex,
	const uint8_t* pData,
	size_t dataSize)
{
	if (!dwTextureID || !pData || dataSize == 0)
		return false;

	if (!IsWgpuActive())
		return false;

	CWgpuModule& module = CWgpuModule::Instance();
	if (!module.IsLoaded())
	{
		std::lock_guard<std::mutex> lock(s_bridgeMutex);
		if (!module.IsLoaded() && !EnsureWgpuLoaded())
			return false;
	}

	return module.UpdateTextureArrayLayer(dwTextureID, layerIndex, pData, dataSize);
}

bool CTextureUploadBridge::UpdateTextureArrayLayer(
	uintptr_t dwTextureID,
	uint32_t layerIndex,
	const void* pData,
	size_t dataSize)
{
	return UpdateTextureArrayLayer(
		dwTextureID,
		layerIndex,
		reinterpret_cast<const uint8_t*>(pData),
		dataSize
	);
}

void CTextureUploadBridge::UnregisterTexture(uintptr_t dwTextureID)
{
	if (!dwTextureID)
		return;

	CWgpuModule::Instance().UnregisterTexture(dwTextureID);
}