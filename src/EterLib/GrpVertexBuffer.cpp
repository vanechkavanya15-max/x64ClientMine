#include "StdAfx.h"
#include "EterBase/Stl.h"
#include "GrpVertexBuffer.h"
#include "StateManager.h"

int	CGraphicVertexBuffer::GetVertexStride() const
{
	int retSize = D3DXGetFVFVertexSize(m_dwFVF);
	return retSize;
}

DWORD CGraphicVertexBuffer::GetFlexibleVertexFormat() const
{
	return m_dwFVF;
}

int CGraphicVertexBuffer::GetVertexCount() const
{
	return m_vtxCount;
}

void CGraphicVertexBuffer::SetStream(int stride, int layer) const
{
	assert(ms_lpd3dDevice != NULL);
	STATEMANAGER.SetStreamSource(layer, m_lpd3dVB, stride);	
}

bool CGraphicVertexBuffer::LockRange(unsigned count, void** pretVertices) const
{
	if (!m_lpd3dVB)
		return false;

	DWORD dwLockSize = GetVertexStride() * count;
	assert(dwLockSize <= m_dwBufferSize);
	if (dwLockSize > m_dwBufferSize)
		return false;

	DWORD dwFlags = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
	{
		dwFlags = (dwLockSize == m_dwBufferSize) ? D3DLOCK_DISCARD : D3DLOCK_NOOVERWRITE;
	}

	if (FAILED(m_lpd3dVB->Lock(0, dwLockSize, (void**)pretVertices, dwFlags)))
		return false;

	return true;
}

bool CGraphicVertexBuffer::Lock(void** pretVertices) const
{
	if (!m_lpd3dVB)
		return false;

	DWORD dwLockSize = GetVertexStride() * GetVertexCount();
	assert(dwLockSize <= m_dwBufferSize);
	if (dwLockSize > m_dwBufferSize)
		return false;

	DWORD dwFlags = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
		dwFlags = D3DLOCK_DISCARD;

	if (FAILED(m_lpd3dVB->Lock(0, dwLockSize, (void**)pretVertices, dwFlags)))
		return false;

	return true;
}

bool CGraphicVertexBuffer::Unlock() const
{
	if (!m_lpd3dVB)
		return false;

	if (FAILED(m_lpd3dVB->Unlock()))
		return false;
	return true;
}

bool CGraphicVertexBuffer::IsEmpty() const
{
	return m_lpd3dVB == nullptr;
}

bool CGraphicVertexBuffer::LockDynamic(void** pretVertices)
{
	if (!m_lpd3dVB)
		return false;

	DWORD dwFlags = (m_dwUsage & D3DUSAGE_DYNAMIC) ? D3DLOCK_DISCARD : 0;
	if (FAILED(m_lpd3dVB->Lock(0, 0, (void**)pretVertices, dwFlags)))
		return false;

	return true;
}

bool CGraphicVertexBuffer::Lock(void** pretVertices)
{
	if (!m_lpd3dVB)
		return false;

	DWORD dwFlags = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
		dwFlags = D3DLOCK_DISCARD;

	if (FAILED(m_lpd3dVB->Lock(0, 0, (void**)pretVertices, dwFlags)))
		return false;

	return true;
}

bool CGraphicVertexBuffer::Lock(DWORD offset, DWORD size, void** pretVertices, DWORD dwFlags)
{
	if (!m_lpd3dVB)
		return false;

	assert(offset + size <= m_dwBufferSize);
	if (offset + size > m_dwBufferSize)
		return false;

	if (m_dwUsage & D3DUSAGE_DYNAMIC)
	{
		if (dwFlags == 0)
			dwFlags = (size == 0 || size == m_dwBufferSize) ? D3DLOCK_DISCARD : D3DLOCK_NOOVERWRITE;
	}

	if (FAILED(m_lpd3dVB->Lock(offset, size, pretVertices, dwFlags)))
		return false;

	return true;
}

bool CGraphicVertexBuffer::Unlock()
{
	if (!m_lpd3dVB)
		return false;

	if (FAILED(m_lpd3dVB->Unlock()))
		return false;
	return true;
}

bool CGraphicVertexBuffer::Copy(int bufSize, const void* srcVertices)
{
	assert(m_lpd3dVB != NULL);
	assert(srcVertices != NULL);
	assert(bufSize > 0);
	assert((DWORD)bufSize <= m_dwBufferSize);

	if (!m_lpd3dVB || !srcVertices || bufSize <= 0 || (DWORD)bufSize > m_dwBufferSize)
		return false;

	void* dstVertices = nullptr;
	DWORD dwFlags = m_dwLockFlag;
	if (m_dwUsage & D3DUSAGE_DYNAMIC)
		dwFlags = ((DWORD)bufSize == m_dwBufferSize) ? D3DLOCK_DISCARD : D3DLOCK_NOOVERWRITE;

	if (FAILED(m_lpd3dVB->Lock(0, bufSize, &dstVertices, dwFlags)))
		return false;

	memcpy(dstVertices, srcVertices, bufSize);
	
	Unlock();
	return true;
}

bool CGraphicVertexBuffer::CreateDeviceObjects()
{
	if (!ms_lpd3dDevice)
		return true;

	assert(m_lpd3dVB == NULL);

	HRESULT hr = ms_lpd3dDevice->CreateVertexBuffer(
		m_dwBufferSize, 
		m_dwUsage, 
		m_dwFVF, 
		m_d3dPool, 
		&m_lpd3dVB,
		nullptr);

	if (FAILED(hr))
	{
		TraceError("CGraphicVertexBuffer::CreateDeviceObjects: CreateVertexBuffer failed (hr=0x%08X, size=%u, fvf=0x%08X, usage=0x%08X, pool=%d)",
			hr, m_dwBufferSize, m_dwFVF, m_dwUsage, m_d3dPool);
		return false;
	}

	return true;
}

void CGraphicVertexBuffer::DestroyDeviceObjects()
{
	safe_release(m_lpd3dVB);
}

bool CGraphicVertexBuffer::Create(int vtxCount, DWORD fvf, DWORD usage, D3DPOOL d3dPool)
{
	assert(ms_lpd3dDevice != NULL);
	assert(vtxCount > 0);

	Destroy();

	m_vtxCount = vtxCount;
	m_dwBufferSize = D3DXGetFVFVertexSize(fvf) * m_vtxCount;
	m_d3dPool = d3dPool;
	m_dwUsage = usage;
	m_dwFVF = fvf;

	if (usage & D3DUSAGE_DYNAMIC)
		m_dwLockFlag = D3DLOCK_DISCARD;
	else if (usage & D3DUSAGE_WRITEONLY)
		m_dwLockFlag = 0;
	else
		m_dwLockFlag = D3DLOCK_READONLY;

	return CreateDeviceObjects();
}

void CGraphicVertexBuffer::Destroy()
{
	DestroyDeviceObjects();
}

void CGraphicVertexBuffer::Initialize()
{
	m_lpd3dVB = NULL;
	m_vtxCount = 0;
	m_dwBufferSize = 0;
}

CGraphicVertexBuffer::CGraphicVertexBuffer()
{
	Initialize();
}

CGraphicVertexBuffer::~CGraphicVertexBuffer()
{
	Destroy();
}
