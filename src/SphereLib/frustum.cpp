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
#include "EterBase/m2_client_bridge.h"

ViewState Frustum::ViewVolumeTest(const Vector3d &c_v3Center,const float c_fRadius) const
{
	int res = m2_frustum_test_sphere(c_v3Center.x, c_v3Center.y, c_v3Center.z, c_fRadius);
	if (res == -1)
		return VS_OUTSIDE;
	if (res == 0)
		return VS_PARTIAL;
	return VS_INSIDE;
}

void Frustum::BuildViewFrustum(D3DXMATRIX & mat)
{
	m_bUsingSphere = false;
	m2_frustum_build((const float*)&mat);
	m_plane[0] = D3DXPLANE(          mat._13,           mat._23,           mat._33,           mat._43);
	m_plane[1] = D3DXPLANE(mat._14 - mat._13, mat._24 - mat._23, mat._34 - mat._33, mat._44 - mat._43);
	//m_plane[0] = D3DXPLANE(mat._14 + mat._13, mat._24 + mat._23, mat._34 + mat._33, mat._44 + mat._43);
	m_plane[2] = D3DXPLANE(mat._14 + mat._11, mat._24 + mat._21, mat._34 + mat._31, mat._44 + mat._41);
	m_plane[3] = D3DXPLANE(mat._14 - mat._11, mat._24 - mat._21, mat._34 - mat._31, mat._44 - mat._41);
	m_plane[4] = D3DXPLANE(mat._14 + mat._12, mat._24 + mat._22, mat._34 + mat._32, mat._44 + mat._42);
	m_plane[5] = D3DXPLANE(mat._14 - mat._12, mat._24 - mat._22, mat._34 - mat._32, mat._44 - mat._42);

	for(int i=0;i<6;i++)
		D3DXPlaneNormalize(&m_plane[i],&m_plane[i]);
}

void Frustum::BuildViewFrustum2(D3DXMATRIX & mat, float fNear, float fFar, float fFov, float fAspect, const D3DXVECTOR3 & vCamera, const D3DXVECTOR3 & vLook)
{
	float fViewLen = fFar-fNear;
	float fH = fViewLen * tan(fFov*0.5f);
	float fW = fH*fAspect;
	D3DXVECTOR3 P(0.0f, 0.0f, fNear+fViewLen*0.5f);
	D3DXVECTOR3 Q(fW, fH, fViewLen);
	D3DXVECTOR3 PQ = P-Q;
	m_fRadius = D3DXVec3Length(&PQ);
	m_v3Center = vCamera + vLook * (fNear+fViewLen*0.5f);
	BuildViewFrustum(mat);
	m_bUsingSphere = true;
}