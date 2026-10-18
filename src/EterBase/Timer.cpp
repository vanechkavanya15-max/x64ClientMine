#include "StdAfx.h"
#include "Timer.h"
#include <chrono>
#include <algorithm>
#include <cmath>

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
		gs_dwClientTime = ELTimer_GetMSec();
	}
}

DWORD ELTimer_GetServerMSec()
{
	return ELTimer_GetMSec() - gs_dwClientTime + gs_dwServerTime;
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
	m_dHighResCurrentTimeSec = ELTimer_GetHighResSec();
	m_dwCurrentTime = static_cast<DWORD>(m_dHighResCurrentTimeSec * 1000.0);
	m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
}

CTimer::~CTimer()
{
}

void CTimer::SetBaseTime()
{
	m_dwCurrentTime = 0;
	m_fCurrentTime = 0.0f;
	m_dHighResCurrentTimeSec = 0.0;
	m_dwElapsedTime = 0;
	m_fElapsedTime = 0.0f;
}

void CTimer::Advance()
{
	if (!m_bUseRealTime)
	{
		AdvanceFixed(1.0 / 60.0);
		return;
	}
	else
	{
		const double curHighResSec = ELTimer_GetHighResSec();

		if (m_dHighResCurrentTimeSec <= 0.0)
		{
			m_dHighResCurrentTimeSec = curHighResSec;
			m_dwCurrentTime = static_cast<DWORD>(curHighResSec * 1000.0);
		}

		double deltaSec = curHighResSec - m_dHighResCurrentTimeSec;
		deltaSec = std::clamp(deltaSec, 0.0, 0.1);

		m_dHighResCurrentTimeSec = curHighResSec;
		const DWORD prevMS = m_dwCurrentTime;
		m_dwCurrentTime = static_cast<DWORD>(m_dHighResCurrentTimeSec * 1000.0);
		m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);

		m_dwElapsedTime = (m_dwCurrentTime >= prevMS) ? (m_dwCurrentTime - prevMS) : 0;
		if (m_dwElapsedTime > 100)
			m_dwElapsedTime = 100;

		m_fElapsedTime = static_cast<float>(deltaSec);
		gs_dwFrameTime = m_dwCurrentTime;
	}
}

void CTimer::AdvanceFixed(double fixedStepSeconds)
{
	const DWORD prevMS = m_dwCurrentTime;
	m_dHighResCurrentTimeSec += fixedStepSeconds;
	m_dwCurrentTime = static_cast<DWORD>(m_dHighResCurrentTimeSec * 1000.0);
	m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
	m_dwElapsedTime = (m_dwCurrentTime >= prevMS) ? (m_dwCurrentTime - prevMS) : 0;
	m_fElapsedTime = static_cast<float>(fixedStepSeconds);
	gs_dwFrameTime = m_dwCurrentTime;
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
