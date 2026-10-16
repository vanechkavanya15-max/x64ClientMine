#include "StdAfx.h"
#include "MiniMap/MiniMapWaypointTracker.h"
#include "EterBase/Timer.h"
#include <cmath>
#include <algorithm>

namespace Client::UI::MiniMap {

MiniMapWaypointTracker::MiniMapWaypointTracker()
	: m_uNextSignalId(255)
	, m_fnGlobalToAtlas(nullptr)
{
	Initialize();
}

MiniMapWaypointTracker::~MiniMapWaypointTracker()
{
	Clear();
}

void MiniMapWaypointTracker::Initialize()
{
	m_kMap_dwVID_kObserver.clear();
	m_AtlasWayPointInfoVector.clear();
	m_SignalPointVector.clear();
	m_uNextSignalId = 255;
	m_fnGlobalToAtlas = nullptr;
}

void MiniMapWaypointTracker::Clear()
{
	ClearSignalPoint();
	m_kMap_dwVID_kObserver.clear();
	m_AtlasWayPointInfoVector.clear();
	m_SignalPointVector.clear();
}

void MiniMapWaypointTracker::AddObserver(DWORD dwVID, float fSrcX, float fSrcY, DWORD dwCurTime)
{
	if (dwCurTime == 0)
		dwCurTime = ELTimer_GetMSec();

	auto f = m_kMap_dwVID_kObserver.find(dwVID);
	if (m_kMap_dwVID_kObserver.end() == f)
	{
		SObserver kObserver;
		kObserver.dwSrcTime = dwCurTime;
		kObserver.dwDstTime = dwCurTime + 1000;
		kObserver.fSrcX = fSrcX;
		kObserver.fSrcY = fSrcY;
		kObserver.fDstX = fSrcX;
		kObserver.fDstY = fSrcY;
		kObserver.fCurX = fSrcX;
		kObserver.fCurY = fSrcY;
		m_kMap_dwVID_kObserver.emplace(dwVID, kObserver);
	}
	else
	{
		SObserver& rkObserver = f->second;
		rkObserver.dwSrcTime = dwCurTime;
		rkObserver.dwDstTime = dwCurTime + 1000;
		rkObserver.fSrcX = fSrcX;
		rkObserver.fSrcY = fSrcY;
		rkObserver.fDstX = fSrcX;
		rkObserver.fDstY = fSrcY;
		rkObserver.fCurX = fSrcX;
		rkObserver.fCurY = fSrcY;
	}
}

void MiniMapWaypointTracker::MoveObserver(DWORD dwVID, float fDstX, float fDstY, DWORD dwCurTime)
{
	auto f = m_kMap_dwVID_kObserver.find(dwVID);
	if (m_kMap_dwVID_kObserver.end() == f)
		return;

	if (dwCurTime == 0)
		dwCurTime = ELTimer_GetMSec();

	SObserver& rkObserver = f->second;
	rkObserver.dwSrcTime = dwCurTime;
	rkObserver.dwDstTime = dwCurTime + 1000;
	rkObserver.fSrcX = rkObserver.fCurX;
	rkObserver.fSrcY = rkObserver.fCurY;
	rkObserver.fDstX = fDstX;
	rkObserver.fDstY = fDstY;
}

void MiniMapWaypointTracker::RemoveObserver(DWORD dwVID)
{
	m_kMap_dwVID_kObserver.erase(dwVID);
}

void MiniMapWaypointTracker::ClearObservers()
{
	m_kMap_dwVID_kObserver.clear();
}

void MiniMapWaypointTracker::UpdateObservers(DWORD dwCurTime)
{
	if (dwCurTime == 0)
		dwCurTime = ELTimer_GetMSec();

	for (auto& pair : m_kMap_dwVID_kObserver)
	{
		SObserver& rkObserver = pair.second;

		float fDenominator = static_cast<float>(rkObserver.dwDstTime - rkObserver.dwSrcTime);
		float fPos = 1.0f;
		if (fDenominator > 0.0001f)
		{
			fPos = static_cast<float>(dwCurTime - rkObserver.dwSrcTime) / fDenominator;
		}

		if (fPos < 0.0f)
			fPos = 0.0f;
		else if (fPos > 1.0f)
			fPos = 1.0f;

		rkObserver.fCurX = (rkObserver.fDstX - rkObserver.fSrcX) * fPos + rkObserver.fSrcX;
		rkObserver.fCurY = (rkObserver.fDstY - rkObserver.fSrcY) * fPos + rkObserver.fSrcY;
	}
}

const std::map<DWORD, SObserver>& MiniMapWaypointTracker::GetObservers() const
{
	return m_kMap_dwVID_kObserver;
}

std::map<DWORD, SObserver>& MiniMapWaypointTracker::GetObservers()
{
	return m_kMap_dwVID_kObserver;
}

bool MiniMapWaypointTracker::HasObserver(DWORD dwVID) const
{
	return m_kMap_dwVID_kObserver.find(dwVID) != m_kMap_dwVID_kObserver.end();
}

const SObserver* MiniMapWaypointTracker::GetObserver(DWORD dwVID) const
{
	auto it = m_kMap_dwVID_kObserver.find(dwVID);
	if (it != m_kMap_dwVID_kObserver.end())
		return &it->second;
	return nullptr;
}

void MiniMapWaypointTracker::AddWayPoint(BYTE byType, DWORD dwID, float fX, float fY, const std::string& strText, DWORD dwChrVID)
{
	for (const auto& mark : m_AtlasWayPointInfoVector)
	{
		if (mark.m_dwID == dwID)
			return;
	}

	SWayPointInfo aAtlasMarkInfo;
	aAtlasMarkInfo.m_byType = byType;
	aAtlasMarkInfo.m_dwID = dwID;
	aAtlasMarkInfo.m_fX = fX;
	aAtlasMarkInfo.m_fY = fY;
	aAtlasMarkInfo.m_fScreenX = 0.0f;
	aAtlasMarkInfo.m_fScreenY = 0.0f;
	aAtlasMarkInfo.m_fMiniMapX = 0.0f;
	aAtlasMarkInfo.m_fMiniMapY = 0.0f;
	aAtlasMarkInfo.m_strText = strText;
	aAtlasMarkInfo.m_dwChrVID = dwChrVID;
	UpdateWayPoint(&aAtlasMarkInfo, static_cast<int>(fX), static_cast<int>(fY));
	m_AtlasWayPointInfoVector.push_back(aAtlasMarkInfo);
}

void MiniMapWaypointTracker::RemoveWayPoint(DWORD dwID)
{
	for (auto it = m_AtlasWayPointInfoVector.begin(); it != m_AtlasWayPointInfoVector.end(); ++it)
	{
		if (it->m_dwID == dwID)
		{
			m_AtlasWayPointInfoVector.erase(it);
			return;
		}
	}
}

bool MiniMapWaypointTracker::GetWayPoint(DWORD dwID, SWayPointInfo** ppkInfo)
{
	if (!ppkInfo)
		return false;

	for (auto& info : m_AtlasWayPointInfoVector)
	{
		if (dwID == info.m_dwID)
		{
			*ppkInfo = &info;
			return true;
		}
	}
	*ppkInfo = nullptr;
	return false;
}

SWayPointInfo* MiniMapWaypointTracker::GetWayPoint(DWORD dwID)
{
	for (auto& info : m_AtlasWayPointInfoVector)
	{
		if (dwID == info.m_dwID)
			return &info;
	}
	return nullptr;
}

const SWayPointInfo* MiniMapWaypointTracker::GetWayPoint(DWORD dwID) const
{
	for (const auto& info : m_AtlasWayPointInfoVector)
	{
		if (dwID == info.m_dwID)
			return &info;
	}
	return nullptr;
}

void MiniMapWaypointTracker::UpdateWayPoint(SWayPointInfo* pkInfo, int ix, int iy)
{
	if (!pkInfo)
		return;

	pkInfo->m_fX = static_cast<float>(ix);
	pkInfo->m_fY = static_cast<float>(iy);
	if (m_fnGlobalToAtlas)
	{
		m_fnGlobalToAtlas(ix, iy, &pkInfo->m_fScreenX, &pkInfo->m_fScreenY);
	}
}

bool MiniMapWaypointTracker::UpdateWayPoint(DWORD dwID, int ix, int iy)
{
	SWayPointInfo* pkInfo = GetWayPoint(dwID);
	if (!pkInfo)
		return false;
	UpdateWayPoint(pkInfo, ix, iy);
	return true;
}

void MiniMapWaypointTracker::ClearWayPoints()
{
	m_AtlasWayPointInfoVector.clear();
}

const std::vector<SWayPointInfo>& MiniMapWaypointTracker::GetWayPoints() const
{
	return m_AtlasWayPointInfoVector;
}

std::vector<SWayPointInfo>& MiniMapWaypointTracker::GetWayPoints()
{
	return m_AtlasWayPointInfoVector;
}

void MiniMapWaypointTracker::AddSignalPoint(float fX, float fY)
{
	TSignalPoint sp;
	sp.id = m_uNextSignalId;
	sp.v2Pos.x = fX;
	sp.v2Pos.y = fY;

	m_SignalPointVector.push_back(sp);

	const BYTE c_byTypeWayPoint = 6; // TYPE_WAYPOINT (CPythonMiniMap::TYPE_WAYPOINT)
	AddWayPoint(c_byTypeWayPoint, m_uNextSignalId, fX, fY, "");

	m_uNextSignalId++;
}

void MiniMapWaypointTracker::ClearAllSignalPoint()
{
	for (const auto& sp : m_SignalPointVector)
	{
		RemoveWayPoint(sp.id);
	}
	m_SignalPointVector.clear();
}

void MiniMapWaypointTracker::ClearSignalPoint()
{
	ClearAllSignalPoint();
}

const std::vector<TSignalPoint>& MiniMapWaypointTracker::GetSignalPoints() const
{
	return m_SignalPointVector;
}

std::vector<TSignalPoint>& MiniMapWaypointTracker::GetSignalPoints()
{
	return m_SignalPointVector;
}

void MiniMapWaypointTracker::SetGlobalToAtlasTransform(PositionTransformer transformer)
{
	m_fnGlobalToAtlas = std::move(transformer);
}

void MiniMapWaypointTracker::UpdateTargetPositions(
	float fCenterX, float fCenterY,
	float fScale, float fCellScale,
	float fMiniMapWidth, float fMiniMapHeight,
	float fScreenX, float fScreenY,
	float fWhiteMarkWidth, float fWhiteMarkHeight,
	ChrPositionGetter fnGetChrPos)
{
	const BYTE c_byTypeTarget = 12; // TYPE_TARGET (CPythonMiniMap::TYPE_TARGET)
	const float c_fMiniMapWindowRadius = (fMiniMapWidth < fMiniMapHeight ? fMiniMapWidth : fMiniMapHeight) / 2.0f - 9.0f;

	for (auto& rAtlasMarkInfo : m_AtlasWayPointInfoVector)
	{
		if (c_byTypeTarget != rAtlasMarkInfo.m_byType)
			continue;

		if (0 != rAtlasMarkInfo.m_dwChrVID && fnGetChrPos)
		{
			float fTargetX = 0.0f, fTargetY = 0.0f;
			if (fnGetChrPos(rAtlasMarkInfo.m_dwChrVID, &fTargetX, &fTargetY))
			{
				UpdateWayPoint(&rAtlasMarkInfo, static_cast<int>(fTargetX), static_cast<int>(fTargetY));
			}
		}

		float fDistanceFromCenterX = (rAtlasMarkInfo.m_fX - fCenterX) * fCellScale * fScale;
		float fDistanceFromCenterY = (rAtlasMarkInfo.m_fY - fCenterY) * fCellScale * fScale;

		float fDistanceFromCenter = sqrtf(fDistanceFromCenterX * fDistanceFromCenterX + fDistanceFromCenterY * fDistanceFromCenterY);

		if (fDistanceFromCenter >= c_fMiniMapWindowRadius && c_fMiniMapWindowRadius > 0.0f)
		{
			float fRadian = atan2f(fDistanceFromCenterY, fDistanceFromCenterX);
			fDistanceFromCenterX = c_fMiniMapWindowRadius * cosf(fRadian);
			fDistanceFromCenterY = c_fMiniMapWindowRadius * sinf(fRadian);
		}

		rAtlasMarkInfo.m_fMiniMapX = (fMiniMapWidth - fWhiteMarkWidth) / 2.0f + fDistanceFromCenterX + fScreenX;
		rAtlasMarkInfo.m_fMiniMapY = (fMiniMapHeight - fWhiteMarkHeight) / 2.0f + fDistanceFromCenterY + fScreenY;
	}
}

} // namespace Client::UI::MiniMap
