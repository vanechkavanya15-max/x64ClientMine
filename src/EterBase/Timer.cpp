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

	static uint64_t gs_u64ServerTime = 0;
	static uint64_t gs_u64ClientTime = 0;
	static uint64_t gs_u64FrameTime = 0;
}

BOOL ELTimer_Init()
{
	gs_bTimerInitialized = true;
	return TRUE;
}

std::chrono::steady_clock::time_point ELTimer_GetNow()
{
	return EngineClock::now();
}

uint64_t ELTimer_GetMSec64()
{
	const auto now = EngineClock::now();
	return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::milliseconds>(now - gs_baseTimePoint).count());
}

uint64_t ELTimer_GetUSec64()
{
	const auto now = EngineClock::now();
	return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::microseconds>(now - gs_baseTimePoint).count());
}

DWORD ELTimer_GetMSec()
{
	return static_cast<DWORD>(ELTimer_GetMSec64() & 0xFFFFFFFFULL);
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
		gs_u64ServerTime = static_cast<uint64_t>(dwServerTime);
		gs_u64ClientTime = ELTimer_GetMSec64();
	}
}

uint64_t ELTimer_GetServerMSec64()
{
	return ELTimer_GetMSec64() - gs_u64ClientTime + gs_u64ServerTime;
}

DWORD ELTimer_GetServerMSec()
{
	return static_cast<DWORD>(ELTimer_GetServerMSec64() & 0xFFFFFFFFULL);
}

uint64_t ELTimer_GetFrameMSec64()
{
	return gs_u64FrameTime;
}

DWORD ELTimer_GetFrameMSec()
{
	return static_cast<DWORD>(gs_u64FrameTime & 0xFFFFFFFFULL);
}

DWORD ELTimer_GetServerFrameMSec()
{
	return static_cast<DWORD>((gs_u64FrameTime - gs_u64ClientTime + gs_u64ServerTime) & 0xFFFFFFFFULL);
}

VOID ELTimer_SetFrameMSec()
{
	gs_u64FrameTime = ELTimer_GetMSec64();
}

CTimer::CTimer()
	: m_bUseRealTime(true)
	, m_u64BaseTime(0)
	, m_u64CurrentTime(0)
	, m_dwCurrentTime(0)
	, m_fCurrentTime(0.0f)
	, m_u64ElapsedTime(0)
	, m_dwElapsedTime(0)
	, m_fElapsedTime(0.0166667f)
	, m_dHighResCurrentTimeSec(0.0)
	, m_index(0)
{
	ELTimer_Init();
	m_dHighResCurrentTimeSec = ELTimer_GetHighResSec();
	m_u64CurrentTime = static_cast<uint64_t>(m_dHighResCurrentTimeSec * 1000.0);
	m_dwCurrentTime = static_cast<DWORD>(m_u64CurrentTime & 0xFFFFFFFFULL);
	m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
}

CTimer::~CTimer()
{
}

void CTimer::SetBaseTime()
{
	m_u64CurrentTime = 0;
	m_dwCurrentTime = 0;
	m_fCurrentTime = 0.0f;
	m_dHighResCurrentTimeSec = 0.0;
	m_u64ElapsedTime = 0;
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
			m_u64CurrentTime = static_cast<uint64_t>(curHighResSec * 1000.0);
			m_dwCurrentTime = static_cast<DWORD>(m_u64CurrentTime & 0xFFFFFFFFULL);
		}

		double deltaSec = curHighResSec - m_dHighResCurrentTimeSec;
		// Zabezpieczenie przed ujemnym krokiem lub przeskokami spowodowanymi obciazeniem/NTP
		deltaSec = std::clamp(deltaSec, 0.0, 0.1);

		m_dHighResCurrentTimeSec = curHighResSec;
		const uint64_t prevMS64 = m_u64CurrentTime;
		m_u64CurrentTime = static_cast<uint64_t>(m_dHighResCurrentTimeSec * 1000.0);
		m_dwCurrentTime = static_cast<DWORD>(m_u64CurrentTime & 0xFFFFFFFFULL);
		m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);

		m_u64ElapsedTime = (m_u64CurrentTime >= prevMS64) ? (m_u64CurrentTime - prevMS64) : 0;
		if (m_u64ElapsedTime > 100)
			m_u64ElapsedTime = 100;

		m_dwElapsedTime = static_cast<DWORD>(m_u64ElapsedTime);
		m_fElapsedTime = static_cast<float>(deltaSec);
		gs_u64FrameTime = m_u64CurrentTime;
	}
}

void CTimer::AdvanceFixed(double fixedStepSeconds)
{
	const uint64_t prevMS64 = m_u64CurrentTime;
	m_dHighResCurrentTimeSec += fixedStepSeconds;
	m_u64CurrentTime = static_cast<uint64_t>(m_dHighResCurrentTimeSec * 1000.0);
	m_dwCurrentTime = static_cast<DWORD>(m_u64CurrentTime & 0xFFFFFFFFULL);
	m_fCurrentTime = static_cast<float>(m_dHighResCurrentTimeSec);
	m_u64ElapsedTime = (m_u64CurrentTime >= prevMS64) ? (m_u64CurrentTime - prevMS64) : 0;
	m_dwElapsedTime = static_cast<DWORD>(m_u64ElapsedTime);
	m_fElapsedTime = static_cast<float>(fixedStepSeconds);
	gs_u64FrameTime = m_u64CurrentTime;
}

void CTimer::Adjust(int iTimeGap)
{
	m_u64CurrentTime += iTimeGap;
	m_dwCurrentTime = static_cast<DWORD>(m_u64CurrentTime & 0xFFFFFFFFULL);
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

uint64_t CTimer::GetCurrentMillisecond64() const noexcept
{
	return m_u64CurrentTime;
}

uint64_t CTimer::GetCurrentMicrosecond64() const noexcept
{
	return static_cast<uint64_t>(m_dHighResCurrentTimeSec * 1000000.0);
}

float CTimer::GetElapsedSecond() const noexcept
{
	return m_fElapsedTime;
}

DWORD CTimer::GetElapsedMilliecond() const noexcept
{
	return m_dwElapsedTime;
}

uint64_t CTimer::GetElapsedMicrosecond64() const noexcept
{
	return static_cast<uint64_t>(m_fElapsedTime * 1000000.0f);
}

void CTimer::UseCustomTime()
{
	m_bUseRealTime = false;
}
