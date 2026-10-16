#pragma once

#include "GrpBase.h"

class CGraphicIndexBuffer : public CGraphicBase
{
public:
	CGraphicIndexBuffer();
	virtual ~CGraphicIndexBuffer();

	void Destroy();
	bool Create(int idxCount, D3DFORMAT d3dFmt);
	bool Create(int idxCount, D3DFORMAT d3dFmt, DWORD usage, D3DPOOL d3dPool);
	bool Create(int faceCount, TFace* faces);

	bool CreateDeviceObjects();
	void DestroyDeviceObjects();

	bool Copy(int bufSize, const void* srcIndices);

	bool Lock(void** pretIndices) const;
	void Unlock() const;

	bool Lock(void** pretIndices);
	bool Lock(DWORD offset, DWORD size, void** pretIndices, DWORD dwFlags = 0);
	void Unlock();

	void SetIndices(int startIndex = 0) const;

	LPDIRECT3DINDEXBUFFER9 GetD3DIndexBuffer() const;

	int GetIndexCount() const { return m_iidxCount; }
	DWORD GetBufferSize() const { return m_dwBufferSize; }

protected:
	void Initialize();

protected:
	LPDIRECT3DINDEXBUFFER9	m_lpd3dIdxBuf;
	DWORD					m_dwBufferSize;
	D3DFORMAT				m_d3dFmt;
	int						m_iidxCount;
	DWORD					m_dwUsage;
	D3DPOOL					m_d3dPool;
	DWORD					m_dwLockFlag;
};
