#include "StdAfx.h"
#include "Timer.h"
#include <chrono>
#include <algorithm>

namespace
{
	using EngineClock = std::chrono::steady_clock;
	static const EngineClock::time_point gs_baseTimePoint = EngineClock::now();
	static bool gs_bTimerInitialized = true;

	static DWORD gs_dwServerTime = 0;
	static DWORD gs_dwClientTime = 0;
	static DWORD gs_dwFrameTime = 0;
}

BOOL ELTimer_Init()
{
	// steady_clock is statically initialized at process start
	gs_bTimerInitialized = true;
	return TRUE;
}

DWORD ELTimer_GetMSec()
{
	const auto now = EngineClock::now();
	const auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(now - gs_baseTimePoint).count();
	return static_cast<DWORD>(ms);
}

double ELTimer_GetHighResSec()
{
	const auto now = EngineClock::now();
	const std::chrono::duration<double> sec = now - gs_baseTimePoint;
	return sec.count();
}

VOID ELTimer_SetServerMSec(DWORD dwServerTime)
{
	if (0 != dwServerTime)
	{
		gs_dwServerTime = dwServerTime;
		gs_dwClientTime = CTimer::instance().GetCurrentMillisecond();
	}
}

DWORD ELTimer_GetServerMSec()
{
	return CTimer::instance().GetCurrentMillisecond() - gs_dwClientTime + gs_dwServerTime;
}

DWORD ELTimer_GetFrameMSec()
{
	return gs_dwFrameTime;
}

DWORD ELTimer_GetServerFrameMSec()
{
	return ELTimer_GetFrameMSec() - gs_dwClientTime + gs_dwServerTime;
}

VOID ELTimer_SetFrameMSec()
{
	gs_dwFrameTime = ELTimer_GetMSec();
}

CTimer::CTimer()
	: m_bUseRealTime(true)
	, m_dwBaseTime(0)
	, m_dwCurrentTime(0)
	, m_fCurrentTime(0.0f)
	, m_dwElapsedTime(0)
	, m_fElapsedTime(0.0166667f)
	, m_dHighResCurrentTimeSec(0.0)
	, m_index(0)
{
	ELTimer_Init();
}

CTimer::~CTimer()
{
}

void CTimer::SetBaseTime()
{
	m_dwCurrentTime = 0;
	m_dHighResCurrentTimeSec = 0.0;
}

void CTimer::Advance()
{
	if (!m_bUseRealTime)
	{
		constexpr double FIXED_STEP = 1.0 / 60.0;
		m_dHighResCurrentTimeSec += FIXED_STEP;
		m_dwElapsedTime = 16;
		m_fElapsedTime = static_cast<float>(FIXED_STEP);
		m_dwCurrentTime = static_cast<DWORD>(m_dHighResCurrentTimeSec * 1000.0);
		m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
	}
	else
	{
		const double curHighResSec = ELTimer_GetHighResSec();
		const DWORD currentTime = ELTimer_GetMSec();

		if (m_dHighResCurrentTimeSec <= 0.0)
		{
			m_dHighResCurrentTimeSec = curHighResSec;
			m_dwCurrentTime = currentTime;
		}

		double deltaSec = curHighResSec - m_dHighResCurrentTimeSec;
		// Clamping protection against spiral of death / huge jumps on freeze or breakpoint
		if (deltaSec < 0.0)
			deltaSec = 0.0;
		else if (deltaSec > 0.25)
			deltaSec = 0.25;

		m_dHighResCurrentTimeSec = curHighResSec;
		m_fElapsedTime = static_cast<float>(deltaSec);
		m_dwElapsedTime = static_cast<DWORD>(deltaSec * 1000.0);
		m_dwCurrentTime = currentTime;
		m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
	}
}

void CTimer::AdvanceFixed(double fixedStepSeconds)
{
	m_dHighResCurrentTimeSec += fixedStepSeconds;
	m_fElapsedTime = static_cast<float>(fixedStepSeconds);
	m_dwElapsedTime = static_cast<DWORD>(fixedStepSeconds * 1000.0);
	m_dwCurrentTime = static_cast<DWORD>(m_dHighResCurrentTimeSec * 1000.0);
	m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
}

void CTimer::Adjust(int iTimeGap)
{
	m_dwCurrentTime += iTimeGap;
	m_dHighResCurrentTimeSec += static_cast<double>(iTimeGap) / 1000.0;
	m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
}

float CTimer::GetCurrentSecond() const noexcept
{
	return m_fCurrentTime;
}

DWORD CTimer::GetCurrentMillisecond() const noexcept
{
	return m_dwCurrentTime;
}

float CTimer::GetElapsedSecond() const noexcept
{
	return m_fElapsedTime;
}

DWORD CTimer::GetElapsedMilliecond() const noexcept
{
	return m_dwElapsedTime;
}

void CTimer::UseCustomTime()
{
	m_bUseRealTime = false;
}
