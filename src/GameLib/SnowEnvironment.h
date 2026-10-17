#pragma once

#include "EterLib/GrpScreen.h"
#include <vector>

class CSnowParticle;
class CGraphicImageInstance;

class CSnowEnvironment : public CScreen
{
	public:
		CSnowEnvironment();
		virtual ~CSnowEnvironment();

		bool Create();
		void Destroy();

		void Enable();
		void Disable();

		void Update(const D3DXVECTOR3 & c_rv3Pos);
		void Deform();
		void Render();

		DWORD GetLastTime() const noexcept { return m_dwLastTime; }
		void SetLastTime(DWORD dwLastTime) noexcept { m_dwLastTime = dwLastTime; }

	protected:
		void __Initialize();

	protected:
		D3DXVECTOR3 m_v3Center;

		CGraphicImageInstance * m_pImageInstance;
		std::vector<CSnowParticle*> m_kVct_pkParticleSnow;

		DWORD m_dwParticleMaxNum;
		BOOL m_bSnowEnable;
		DWORD m_dwLastTime;
};
