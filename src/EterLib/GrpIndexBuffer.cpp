#include "StdAfx.h"
#include "EterBase/Stl.h"
#include "GrpIndexBuffer.h"
#include "StateManager.h"
#include "RenderResourceRegistry.h"
#include "EterBase/CRC32.h"

LPDIRECT3DINDEXBUFFER9 CGraphicIndexBuffer::GetD3DIndexBuffer() const
{
	assert(m_lpd3dIdxBuf != NULL);
	return m_lpd3dIdxBuf;
}

void CGraphicIndexBuffer::SetIndices(int startIndex) const
{
	assert(ms_lpd3dDevice != NULL);
	STATEMANAGER.SetIndices(m_lpd3dIdxBuf, startIndex);
}


bool CGraphicIndexBuffer::Lock(void** pretIndices) const
{
	if (!m_lpd3dIdxBuf)
	{
		if (!m_cpuCopy.empty() && pretIndices)
		{
			*pretIndices = const_cast<uint8_t*>(m_cpuCopy.data());
			return true;
		}
		return false;
	}

	DWORD dwLockFlag = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
		dwLockFlag = D3DLOCK_DISCARD;

	if (FAILED(m_lpd3dIdxBuf->Lock(0, m_dwBufferSize, pretIndices, dwLockFlag)))
		return false;

	return true;
}

void CGraphicIndexBuffer::Unlock() const
{
	if (!m_lpd3dIdxBuf)
		return;

	m_lpd3dIdxBuf->Unlock();
}

bool CGraphicIndexBuffer::Lock(void** pretIndices)
{
	if (!m_lpd3dIdxBuf)
	{
		if (m_dwBufferSize > 0 && pretIndices)
		{
			if (m_cpuCopy.size() < m_dwBufferSize)
				m_cpuCopy.resize(m_dwBufferSize, 0);
			*pretIndices = m_cpuCopy.data();
			return true;
		}
		return false;
	}

	DWORD dwLockFlag = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
		dwLockFlag = D3DLOCK_DISCARD;

	if (FAILED(m_lpd3dIdxBuf->Lock(0, m_dwBufferSize, pretIndices, dwLockFlag)))
		return false;

	return true;
}

bool CGraphicIndexBuffer::Lock(DWORD offset, DWORD size, void** pretIndices, DWORD dwFlags)
{
	assert(m_lpd3dIdxBuf != NULL);
	if (!m_lpd3dIdxBuf)
		return false;

	assert(offset + size <= m_dwBufferSize);
	if (offset + size > m_dwBufferSize)
		return false;

	if (m_dwUsage & D3DUSAGE_DYNAMIC)
	{
		if (dwFlags == 0)
			dwFlags = (size == 0 || size == m_dwBufferSize) ? D3DLOCK_DISCARD : D3DLOCK_NOOVERWRITE;
	}

	if (FAILED(m_lpd3dIdxBuf->Lock(offset, size, pretIndices, dwFlags)))
		return false;

	return true;
}

void CGraphicIndexBuffer::Unlock()
{
	assert(m_lpd3dIdxBuf != NULL);

	if (!m_lpd3dIdxBuf)
		return;

	m_lpd3dIdxBuf->Unlock();
}

bool CGraphicIndexBuffer::Copy(int bufSize, const void* srcIndices)
{
	assert(m_lpd3dIdxBuf != NULL);
	assert(srcIndices != NULL);
	assert(bufSize > 0);
	assert((DWORD)bufSize <= m_dwBufferSize);

	if (!m_lpd3dIdxBuf || !srcIndices || bufSize <= 0 || (DWORD)bufSize > m_dwBufferSize)
		return false;

	BYTE* dstIndices = nullptr;
	DWORD dwLockFlag = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
		dwLockFlag = ((DWORD)bufSize == m_dwBufferSize) ? D3DLOCK_DISCARD : D3DLOCK_NOOVERWRITE;

	if (FAILED(m_lpd3dIdxBuf->Lock(0, bufSize, (void**)&dstIndices, dwLockFlag)))
		return false;

	memcpy(dstIndices, srcIndices, bufSize);

	m_lpd3dIdxBuf->Unlock();

	m_cpuCopy.assign(static_cast<const uint8_t*>(srcIndices), static_cast<const uint8_t*>(srcIndices) + bufSize);
	CRenderResourceRegistry::SafeSetIndexBufferCpuCopy(m_lpd3dIdxBuf, srcIndices, bufSize);

	return true;
}

bool CGraphicIndexBuffer::Create(int faceCount, TFace* faces)
{
	int idxCount = faceCount * 3;
	m_iidxCount = idxCount;
	if (!Create(idxCount, D3DFMT_INDEX16))
		return false;

	assert(m_lpd3dIdxBuf != NULL);
	assert(faces != NULL);
	DWORD requiredBytes = sizeof(WORD) * idxCount;
	assert(requiredBytes <= m_dwBufferSize);
	if (!m_lpd3dIdxBuf || !faces || requiredBytes > m_dwBufferSize)
		return false;

	WORD* dstIndices = nullptr;
	DWORD dwLockFlag = (m_dwUsage & D3DUSAGE_DYNAMIC) ? D3DLOCK_DISCARD : 0;
	if (FAILED(m_lpd3dIdxBuf->Lock(0, requiredBytes, (void**)&dstIndices, dwLockFlag)))
		return false;

	WORD* dstIndicesStart = dstIndices;
	for (int i = 0; i < faceCount; ++i, dstIndices += 3)
	{
		TFace* curFace = faces + i;
		dstIndices[0] = curFace->indices[0];
		dstIndices[1] = curFace->indices[1];
		dstIndices[2] = curFace->indices[2];
	}

	m_lpd3dIdxBuf->Unlock();

	m_cpuCopy.assign(reinterpret_cast<const uint8_t*>(dstIndicesStart), reinterpret_cast<const uint8_t*>(dstIndicesStart) + requiredBytes);
	CRenderResourceRegistry::SafeSetIndexBufferCpuCopy(m_lpd3dIdxBuf, m_cpuCopy.data(), m_cpuCopy.size());

	return true;
}

bool CGraphicIndexBuffer::CreateDeviceObjects()
{
	if (!ms_lpd3dDevice)
	{
		if (m_dwBufferSize > 0)
			m_cpuCopy.resize(m_dwBufferSize, 0);
		return true;
	}

	assert(m_lpd3dIdxBuf == NULL);

	HRESULT hr = ms_lpd3dDevice->CreateIndexBuffer(
		m_dwBufferSize,
		m_dwUsage,
		m_d3dFmt,
		m_d3dPool,
		&m_lpd3dIdxBuf,
		NULL);

	if (FAILED(hr))
	{
		TraceError("CGraphicIndexBuffer::CreateDeviceObjects: CreateIndexBuffer failed (hr=0x%08X, size=%u, fmt=%d, usage=0x%08X, pool=%d)",
			hr, m_dwBufferSize, m_d3dFmt, m_dwUsage, m_d3dPool);
		return false;
	}

	CRenderResourceRegistry::SafeRegisterIndexBuffer(
		m_lpd3dIdxBuf,
		m_dwBufferSize,
		m_d3dFmt,
		m_cpuCopy.empty() ? nullptr : m_cpuCopy.data(),
		(m_dwUsage & D3DUSAGE_DYNAMIC) ? "DynamicIB" : "StaticIB"
	);

	return true;
}

void CGraphicIndexBuffer::DestroyDeviceObjects()
{
	if (m_lpd3dIdxBuf)
	{
		CRenderResourceRegistry::SafeUnregisterIndexBuffer(m_lpd3dIdxBuf);
		safe_release(m_lpd3dIdxBuf);
	}
	m_cpuCopy.clear();
}

bool CGraphicIndexBuffer::Create(int idxCount, D3DFORMAT d3dFmt)
{
	return Create(idxCount, d3dFmt, D3DUSAGE_WRITEONLY, D3DPOOL_DEFAULT);
}

bool CGraphicIndexBuffer::Create(int idxCount, D3DFORMAT d3dFmt, DWORD usage, D3DPOOL d3dPool)
{
	Destroy();

	m_iidxCount = idxCount;
	UINT bytesPerIndex = (d3dFmt == D3DFMT_INDEX32) ? 4u : 2u;
	m_dwBufferSize = bytesPerIndex * idxCount;
	m_d3dFmt = d3dFmt;
	m_dwUsage = usage;
	m_d3dPool = d3dPool;

	if (usage & D3DUSAGE_DYNAMIC)
		m_dwLockFlag = D3DLOCK_DISCARD;
	else if (usage & D3DUSAGE_WRITEONLY)
		m_dwLockFlag = 0;
	else
		m_dwLockFlag = D3DLOCK_READONLY;

	return CreateDeviceObjects();
}

void CGraphicIndexBuffer::Destroy()
{
	DestroyDeviceObjects();
}

void CGraphicIndexBuffer::Initialize()
{
	m_lpd3dIdxBuf = NULL;
	m_dwBufferSize = 0;
	m_d3dFmt = D3DFMT_UNKNOWN;
	m_iidxCount = 0;
	m_dwUsage = D3DUSAGE_WRITEONLY;
	m_d3dPool = D3DPOOL_DEFAULT;
	m_dwLockFlag = 0;
	m_cpuCopy.clear();
}

bool CGraphicIndexBuffer::GetCpuCopy(std::vector<uint8_t>& outData) const
{
	if (!m_cpuCopy.empty())
	{
		outData = m_cpuCopy;
		return true;
	}
	if (m_lpd3dIdxBuf)
	{
		return CRenderResourceRegistry::SafeGetIndexBufferCpuCopy(m_lpd3dIdxBuf, outData);
	}
	return false;
}

bool CGraphicIndexBuffer::HasCpuCopy() const
{
	if (!m_cpuCopy.empty())
		return true;
	if (m_lpd3dIdxBuf && CRenderResourceRegistry::InstancePtr())
	{
		std::vector<uint8_t> tmp;
		return CRenderResourceRegistry::SafeGetIndexBufferCpuCopy(m_lpd3dIdxBuf, tmp);
	}
	return false;
}

uint32_t CGraphicIndexBuffer::GetCpuCopyCrc32() const
{
	if (!m_cpuCopy.empty())
	{
		return GetCRC32(reinterpret_cast<const char*>(m_cpuCopy.data()), m_cpuCopy.size());
	}
	if (m_lpd3dIdxBuf && CRenderResourceRegistry::InstancePtr())
	{
		TRenderIndexBufferHandle h = CRenderResourceRegistry::Instance().FindIndexBufferHandle(m_lpd3dIdxBuf);
		if (h != kInvalidIndexBufferHandle)
			return CRenderResourceRegistry::Instance().GetIndexBufferCrc32(h);
	}
	return 0;
}

CGraphicIndexBuffer::CGraphicIndexBuffer()
{
	Initialize();
}

CGraphicIndexBuffer::~CGraphicIndexBuffer()
{
	Destroy();
}
