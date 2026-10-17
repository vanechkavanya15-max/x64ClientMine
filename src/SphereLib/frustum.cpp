/* Copyright (C) John W. Ratcliff, 2001. 
 * All rights reserved worldwide.
 *
 * This software is provided "as is" without express or implied
 * warranties. You may freely copy and compile this source into
 * applications you distribute provided that the copyright text
 * below is included in the resulting source code, for example:
 * "Portions Copyright (C) John W. Ratcliff, 2001"
 */

/* Copyright (C) John W. Ratcliff, 2001. 
 * All rights reserved worldwide.
 *
 * This software is provided "as is" without express or implied
 * warranties. You may freely copy and compile this source into
 * applications you distribute provided that the copyright text
 * below is included in the resulting source code, for example:
 * "Portions Copyright (C) John W. Ratcliff, 2001"
 */

#include "Stdafx.h"
#include "frustum.h"
#include <cmath>
#include <algorithm>

namespace
{
	// Ujednolicona, szybka funkcja normalizujaca plaszczyzne z pelna odpornoscia na dzielenie przez zero
	inline void NormalizePlaneUnified(D3DXPLANE& plane) noexcept
	{
		const float magSq = plane.a * plane.a + plane.b * plane.b + plane.c * plane.c;
		if (magSq > 1e-12f)
		{
			D3DXPlaneNormalize(&plane, &plane);
		}
	}
}

void Frustum::ExtractPlanes(const D3DXMATRIX & mat, D3DXPLANE * pPlanesOut, bool bNegatePlaneY)
{
	if (!pPlanesOut)
		return;

	// Zoptymalizowana ekstrakcja 6 plaszczyzn bryl widzenia (DirectX row-major view-projection)
	// Near:   Z >= 0
	pPlanesOut[FRUSTUM_PLANE_NEAR] = D3DXPLANE(mat._13, mat._23, mat._33, mat._43);

	// Far:    Z <= W (W - Z >= 0)
	pPlanesOut[FRUSTUM_PLANE_FAR] = D3DXPLANE(mat._14 - mat._13, mat._24 - mat._23, mat._34 - mat._33, mat._44 - mat._43);

	// Left:   X >= -W (W + X >= 0)
	pPlanesOut[FRUSTUM_PLANE_LEFT] = D3DXPLANE(mat._14 + mat._11, mat._24 + mat._21, mat._34 + mat._31, mat._44 + mat._41);

	// Right:  X <= W (W - X >= 0)
	pPlanesOut[FRUSTUM_PLANE_RIGHT] = D3DXPLANE(mat._14 - mat._11, mat._24 - mat._21, mat._34 - mat._31, mat._44 - mat._41);

	// Bottom: Y >= -W (W + Y >= 0)
	pPlanesOut[FRUSTUM_PLANE_BOTTOM] = D3DXPLANE(mat._14 + mat._12, mat._24 + mat._22, mat._34 + mat._32, mat._44 + mat._42);

	// Top:    Y <= W (W - Y >= 0)
	pPlanesOut[FRUSTUM_PLANE_TOP] = D3DXPLANE(mat._14 - mat._12, mat._24 - mat._22, mat._34 - mat._32, mat._44 - mat._42);

	// Ujednolicona normalizacja plaszczyzn
	for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
	{
		NormalizePlaneUnified(pPlanesOut[i]);
		if (bNegatePlaneY)
		{
			pPlanesOut[i].b = -pPlanesOut[i].b;
		}
	}
}

void Frustum::BuildViewFrustum(const D3DXMATRIX & mat, bool bNegatePlaneY)
{
	m_bUsingSphere = false;
	ExtractPlanes(mat, m_plane, bNegatePlaneY);
}

void Frustum::BuildViewFrustum2(const D3DXMATRIX & mat, float fNear, float fFar, float fFov, float fAspect, const D3DXVECTOR3 & vCamera, const D3DXVECTOR3 & vLook)
{
	float fViewLen = fFar - fNear;
	float fH = fViewLen * tan(fFov * 0.5f);
	float fW = fH * fAspect;
	D3DXVECTOR3 P(0.0f, 0.0f, fNear + fViewLen * 0.5f);
	D3DXVECTOR3 Q(fW, fH, fViewLen);
	D3DXVECTOR3 PQ = P - Q;
	m_fRadius = D3DXVec3Length(&PQ);
	m_v3Center = vCamera + vLook * (fNear + fViewLen * 0.5f);
	BuildViewFrustum(mat, false);
	m_bUsingSphere = true;
}

ViewState Frustum::ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius) const
{
	return ViewVolumeTest(c_v3Center, c_fRadius, m_fHysteresisMargin, false);
}

ViewState Frustum::ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin) const
{
	return ViewVolumeTest(c_v3Center, c_fRadius, c_fMargin, false);
}

ViewState Frustum::ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin, bool bNegateY) const
{
	const float effectiveRadius = c_fRadius + c_fMargin;
	D3DXVECTOR3 center = c_v3Center;
	if (bNegateY)
	{
		center.y = -center.y;
	}

	if (m_bUsingSphere && !bNegateY)
	{
		D3DXVECTOR3 v(
			center.x - m_v3Center.x,
			center.y - m_v3Center.y,
			center.z - m_v3Center.z);

		const float maxDist = effectiveRadius + m_fRadius;
		if (maxDist * maxDist < D3DXVec3LengthSq(&v))
		{
			return VS_OUTSIDE;
		}
	}	
	
	constexpr int count = 6;

	float distance[count];
	for (int i = 0; i < count; ++i)
	{
		distance[i] = D3DXPlaneDotCoord(&m_plane[i], &center);
		// Margines histerezy: odrzucenie tylko gdy kula przekroczy bufor bezpieczenstwa (brak poppingu)
		if (distance[i] <= -effectiveRadius) 
			return VS_OUTSIDE;
	}

	for (int i = 0; i < count; ++i)
	{
		if (distance[i] <= c_fRadius) 
			return VS_PARTIAL;
	}
	
	return VS_INSIDE;
}

ViewState Frustum::ViewVolumeTestTerrain(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin) const
{
	return ViewVolumeTest(c_v3Center, c_fRadius, c_fMargin, true);
}

bool Frustum::IsBoxVisible(const Vector3d &c_v3Min, const Vector3d &c_v3Max, const float c_fMargin, bool bNegateY) const
{
	const float yMin = bNegateY ? -c_v3Max.y : c_v3Min.y;
	const float yMax = bNegateY ? -c_v3Min.y : c_v3Max.y;

	for (int i = 0; i < 6; ++i)
	{
		// Wierzcholek AABB najbardziej wysuniety w kierunku normalnej plaszczyzny
		D3DXVECTOR3 p;
		p.x = (m_plane[i].a >= 0.0f) ? c_v3Max.x : c_v3Min.x;
		p.y = (m_plane[i].b >= 0.0f) ? yMax : yMin;
		p.z = (m_plane[i].c >= 0.0f) ? c_v3Max.z : c_v3Min.z;

		if (D3DXPlaneDotCoord(&m_plane[i], &p) + c_fMargin < 0.0f)
		{
			return false;
		}
	}
	return true;
}

ViewState Frustum::ViewVolumeTestAABB(const Vector3d &c_v3Min, const Vector3d &c_v3Max, const float c_fMargin, bool bNegateY) const
{
	const float yMin = bNegateY ? -c_v3Max.y : c_v3Min.y;
	const float yMax = bNegateY ? -c_v3Min.y : c_v3Max.y;

	bool isPartial = false;
	for (int i = 0; i < 6; ++i)
	{
		D3DXVECTOR3 pMax(
			(m_plane[i].a >= 0.0f) ? c_v3Max.x : c_v3Min.x,
			(m_plane[i].b >= 0.0f) ? yMax : yMin,
			(m_plane[i].c >= 0.0f) ? c_v3Max.z : c_v3Min.z
		);

		float distMax = D3DXPlaneDotCoord(&m_plane[i], &pMax);
		if (distMax + c_fMargin < 0.0f)
		{
			return VS_OUTSIDE;
		}

		D3DXVECTOR3 pMin(
			(m_plane[i].a >= 0.0f) ? c_v3Min.x : c_v3Max.x,
			(m_plane[i].b >= 0.0f) ? yMin : yMax,
			(m_plane[i].c >= 0.0f) ? c_v3Min.z : c_v3Max.z
		);

		float distMin = D3DXPlaneDotCoord(&m_plane[i], &pMin);
		if (distMin - c_fMargin < 0.0f)
		{
			isPartial = true;
		}
	}

	return isPartial ? VS_PARTIAL : VS_INSIDE;
}

bool Frustum::IsSphereVisible(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin, bool bNegateY) const
{
	return ViewVolumeTest(c_v3Center, c_fRadius, c_fMargin, bNegateY) != VS_OUTSIDE;
}
