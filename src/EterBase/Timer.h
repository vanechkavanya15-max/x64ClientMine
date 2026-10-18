#pragma once

#include <windows.h>
#include "Singleton.h"

class CTimer : public CSingleton<CTimer>
{
	public:
		CTimer();
		virtual ~CTimer();

		void	Advance();
		void	AdvanceFixed(double fixedStepSeconds);
		void	Adjust(int iTimeGap);
		void	SetBaseTime();

		float	GetCurrentSecond() const noexcept;
		DWORD	GetCurrentMillisecond() const noexcept;

		float	GetElapsedSecond() const noexcept;
		DWORD	GetElapsedMilliecond() const noexcept;

		void	UseCustomTime();

	protected:
		bool	m_bUseRealTime;
		DWORD	m_dwBaseTime;
		DWORD	m_dwCurrentTime;
		float	m_fCurrentTime;
		DWORD	m_dwElapsedTime;
		float	m_fElapsedTime;
		double	m_dHighResCurrentTimeSec;
		int		m_index;
};

BOOL	ELTimer_Init();

DWORD	ELTimer_GetMSec();
double	ELTimer_GetHighResSec();

VOID	ELTimer_SetServerMSec(DWORD dwServerTime);
DWORD	ELTimer_GetServerMSec();

VOID	ELTimer_SetFrameMSec();
DWORD	ELTimer_GetFrameMSec();
DWORD	ELTimer_GetServerFrameMSec();
