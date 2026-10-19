#pragma once

#include <windows.h>
#include <chrono>
#include <cstdint>
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
		uint64_t GetCurrentMillisecond64() const noexcept;
		uint64_t GetCurrentMicrosecond64() const noexcept;

		float	GetElapsedSecond() const noexcept;
		DWORD	GetElapsedMilliecond() const noexcept;
		uint64_t GetElapsedMicrosecond64() const noexcept;

		void	UseCustomTime();

	protected:
		bool		m_bUseRealTime;
		uint64_t	m_u64BaseTime;
		uint64_t	m_u64CurrentTime;
		DWORD		m_dwCurrentTime;
		float		m_fCurrentTime;
		uint64_t	m_u64ElapsedTime;
		DWORD		m_dwElapsedTime;
		float		m_fElapsedTime;
		double		m_dHighResCurrentTimeSec;
		int			m_index;
};

BOOL	ELTimer_Init();

DWORD	ELTimer_GetMSec();
uint64_t ELTimer_GetMSec64();
uint64_t ELTimer_GetUSec64();
double	ELTimer_GetHighResSec();
std::chrono::steady_clock::time_point ELTimer_GetNow();

VOID	ELTimer_SetServerMSec(DWORD dwServerTime);
DWORD	ELTimer_GetServerMSec();
uint64_t ELTimer_GetServerMSec64();

VOID	ELTimer_SetFrameMSec();
DWORD	ELTimer_GetFrameMSec();
uint64_t ELTimer_GetFrameMSec64();
DWORD	ELTimer_GetServerFrameMSec();
