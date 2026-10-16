#pragma once

#include <string>
#include <vector>
#include <map>
#include <functional>
#include <d3dx9math.h>

namespace Client::UI::MiniMap {

struct SObserver
{
	float fCurX;
	float fCurY;
	float fSrcX;
	float fSrcY;
	float fDstX;
	float fDstY;

	DWORD dwSrcTime;
	DWORD dwDstTime;

	SObserver()
		: fCurX(0.0f), fCurY(0.0f)
		, fSrcX(0.0f), fSrcY(0.0f)
		, fDstX(0.0f), fDstY(0.0f)
		, dwSrcTime(0), dwDstTime(0)
	{
	}
};

struct TSignalPoint
{
	D3DXVECTOR2 v2Pos;
	unsigned int id;

	TSignalPoint()
		: v2Pos(0.0f, 0.0f)
		, id(0)
	{
	}

	TSignalPoint(unsigned int nId, float fX, float fY)
		: v2Pos(fX, fY)
		, id(nId)
	{
	}
};

struct SWayPointInfo
{
	BYTE m_byType;
	DWORD m_dwID;
	float m_fX;
	float m_fY;
	float m_fScreenX;
	float m_fScreenY;
	float m_fMiniMapX;
	float m_fMiniMapY;
	DWORD m_dwChrVID;
	std::string m_strText;

	SWayPointInfo()
		: m_byType(0)
		, m_dwID(0)
		, m_fX(0.0f)
		, m_fY(0.0f)
		, m_fScreenX(0.0f)
		, m_fScreenY(0.0f)
		, m_fMiniMapX(0.0f)
		, m_fMiniMapY(0.0f)
		, m_dwChrVID(0)
	{
	}
};

using TAtlasMarkInfo = SWayPointInfo;
using TWayPointInfo = SWayPointInfo;

class MiniMapWaypointTracker
{
public:
	using PositionTransformer = std::function<void(long lx, long ly, float* pfx, float* pfy)>;
	using ChrPositionGetter = std::function<bool(DWORD dwChrVID, float* pfX, float* pfY)>;

	MiniMapWaypointTracker();
	~MiniMapWaypointTracker();

	void Initialize();
	void Clear();

	// Observerzy (SObserver)
	void AddObserver(DWORD dwVID, float fSrcX, float fSrcY, DWORD dwCurTime = 0);
	void MoveObserver(DWORD dwVID, float fDstX, float fDstY, DWORD dwCurTime = 0);
	void RemoveObserver(DWORD dwVID);
	void ClearObservers();
	void UpdateObservers(DWORD dwCurTime);
	const std::map<DWORD, SObserver>& GetObservers() const;
	std::map<DWORD, SObserver>& GetObservers();
	bool HasObserver(DWORD dwVID) const;
	const SObserver* GetObserver(DWORD dwVID) const;

	// Punkty nawigacyjne (WayPoints)
	void AddWayPoint(BYTE byType, DWORD dwID, float fX, float fY, const std::string& strText, DWORD dwChrVID = 0);
	void RemoveWayPoint(DWORD dwID);
	bool GetWayPoint(DWORD dwID, SWayPointInfo** ppkInfo);
	SWayPointInfo* GetWayPoint(DWORD dwID);
	const SWayPointInfo* GetWayPoint(DWORD dwID) const;
	void UpdateWayPoint(SWayPointInfo* pkInfo, int ix, int iy);
	bool UpdateWayPoint(DWORD dwID, int ix, int iy);
	void ClearWayPoints();
	const std::vector<SWayPointInfo>& GetWayPoints() const;
	std::vector<SWayPointInfo>& GetWayPoints();

	// Punkty sygnalizacyjne (SignalPoints)
	void AddSignalPoint(float fX, float fY);
	void ClearSignalPoint();
	void ClearAllSignalPoint();
	const std::vector<TSignalPoint>& GetSignalPoints() const;
	std::vector<TSignalPoint>& GetSignalPoints();

	// Transformacja wspolrzednych globalnych do wspolrzednych atlasu
	void SetGlobalToAtlasTransform(PositionTransformer transformer);

	// Aktualizacja pozycji celow na minimapie (matematyka i clamping promienia, bez renderowania Direct3D9)
	void UpdateTargetPositions(
		float fCenterX, float fCenterY,
		float fScale, float fCellScale,
		float fMiniMapWidth, float fMiniMapHeight,
		float fScreenX, float fScreenY,
		float fWhiteMarkWidth, float fWhiteMarkHeight,
		ChrPositionGetter fnGetChrPos = nullptr
	);

private:
	std::map<DWORD, SObserver>		m_kMap_dwVID_kObserver;
	std::vector<SWayPointInfo>		m_AtlasWayPointInfoVector;
	std::vector<TSignalPoint>		m_SignalPointVector;
	unsigned int					m_uNextSignalId;
	PositionTransformer				m_fnGlobalToAtlas;
};

using CMiniMapWaypointTracker = MiniMapWaypointTracker;

} // namespace Client::UI::MiniMap
