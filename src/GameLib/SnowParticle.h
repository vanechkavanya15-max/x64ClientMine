#pragma once

#include <d3dx9.h>
#include <vector>

struct SParticleVertex
{
	D3DXVECTOR3 v3Pos;
	float u, v;
};

struct BlurVertex
{
	D3DXVECTOR3 pos;
	FLOAT       rhw;
	DWORD       color;
	FLOAT       tu, tv;

	static const DWORD FVF;

	BlurVertex(D3DXVECTOR3 p, float w, DWORD c, float u, float v)
		: pos(p), rhw(w), color(c), tu(u), tv(v) {}
	~BlurVertex() = default;
};

class CSnowParticle
{
	public:
		CSnowParticle();
		~CSnowParticle();

		static void InitializePool(size_t poolSize);
		static CSnowParticle * New();
		static void Delete(CSnowParticle * pSnowParticle);
		static void DestroyPool();

		void Init(const D3DXVECTOR3 & c_rv3Pos);

		void SetCameraVertex(const D3DXVECTOR3 & rv3Up, const D3DXVECTOR3 & rv3Cross);
		bool IsActivate() const;

		void Update(float fElapsedTime, const D3DXVECTOR3 & c_rv3Pos);
		void GetVerticies(SParticleVertex & rv3Vertex1, SParticleVertex & rv3Vertex2,
						  SParticleVertex & rv3Vertex3, SParticleVertex & rv3Vertex4);

		const D3DXVECTOR3 & GetPosition() const noexcept { return m_v3Position; }
		float GetHalfWidth() const noexcept { return m_fHalfWidth; }
		float GetHalfHeight() const noexcept { return m_fHalfHeight; }
		float GetCurRadian() const noexcept { return m_fcurRadian; }
		float GetRotation() const noexcept { return D3DXToDegree(m_fcurRadian); }

	protected:
		bool m_bActivate;
		bool m_bChangedSize;
		float m_fHalfWidth;
		float m_fHalfHeight;

		D3DXVECTOR3 m_v3Velocity;
		D3DXVECTOR3 m_v3Position;

		D3DXVECTOR3 m_v3Up;
		D3DXVECTOR3 m_v3Cross;

		float m_fPeriod;
		float m_fcurRadian;
		float m_fAmplitude;

	public:
		static std::vector<CSnowParticle*> ms_kVct_SnowParticlePool;
};
