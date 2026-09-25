#include "StdAfx.h"
#include "Timer.h"
#include "m2_client_bridge.h"

#pragma comment(lib, "winmm.lib")

BOOL ELTimer_Init()
{	
	return m2_timer_init();
}

DWORD ELTimer_GetMSec()
{
	return m2_timer_get_msec();
}

VOID	ELTimer_SetServerMSec(DWORD dwServerTime)
{
	m2_timer_set_server_msec(dwServerTime);
}

DWORD	ELTimer_GetServerMSec()
{
	return m2_timer_get_server_msec();
}

DWORD	ELTimer_GetFrameMSec()
{
	return m2_timer_get_frame_msec();
}

DWORD	ELTimer_GetServerFrameMSec()
{
	return m2_timer_get_server_msec();
}

VOID	ELTimer_SetFrameMSec()
{
	m2_timer_set_frame_msec();
}

CTimer::CTimer()
{
	ELTimer_Init();

	NANOBEGIN
	if (this)
	{
		m_dwCurrentTime = 0;
		m_bUseRealTime = true;
		m_index = 0;
	
		m_dwElapsedTime = 0;

		m_fCurrentTime = 0.0f;
	}
	NANOEND
}

CTimer::~CTimer()
{
}

void CTimer::SetBaseTime()
{
	m_dwCurrentTime = 0;
}

void CTimer::Advance()
{
	if (!m_bUseRealTime)
	{
		++m_index;

		if (m_index == 1)
			m_index = -1;

		m_dwCurrentTime += 16 + (m_index & 1);
		m_fCurrentTime = m_dwCurrentTime / 1000.0f;
	}
	else
	{
		DWORD currentTime = ELTimer_GetMSec();

		if (m_dwCurrentTime == 0)
			m_dwCurrentTime = currentTime;

		m_dwElapsedTime = currentTime - m_dwCurrentTime;
		m_dwCurrentTime = currentTime;
	}
}

void CTimer::Adjust(int iTimeGap)
{
	m_dwCurrentTime += iTimeGap;
}

float CTimer::GetCurrentSecond()
{
	if (m_bUseRealTime)
		return ELTimer_GetMSec() / 1000.0f;

	return m_fCurrentTime;
}

DWORD CTimer::GetCurrentMillisecond()
{
	if (m_bUseRealTime)
		return ELTimer_GetMSec();

	return m_dwCurrentTime;
}

float CTimer::GetElapsedSecond()
{
	return GetElapsedMilliecond() / 1000.0f;
}

DWORD CTimer::GetElapsedMilliecond()
{
	if (!m_bUseRealTime)
		return 16 + (m_index & 1);

	return m_dwElapsedTime;
}

void CTimer::UseCustomTime()
{
	m_bUseRealTime = false;
}
