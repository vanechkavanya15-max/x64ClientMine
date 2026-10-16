/* Copyright (C) John W. Ratcliff, 2001. 
 * All rights reserved worldwide.
 *
 * This software is provided "as is" without express or implied
 * warranties. You may freely copy and compile this source into
 * applications you distribute provided that the copyright text
 * below is included in the resulting source code, for example:
 * "Portions Copyright (C) John W. Ratcliff, 2001"
 */
#pragma once

/***********************************************************************/
/** FRUSTUM.H   : Represents a clipping frustum.                       */
/**               You should replace this with your own more robust    */
/**               view frustum clipper.                                */
/**                                                                    */
/**               Written by John W. Ratcliff jratcliff@att.net        */
/***********************************************************************/

#include "vector.h"

enum ViewState
{
	VS_INSIDE,   // completely inside the frustum.
	VS_PARTIAL,  // partially inside and partially outside the frustum.
	VS_OUTSIDE   // completely outside the frustum
};

class Frustum 
{
	public:
		static constexpr float DEFAULT_HYSTERESIS_MARGIN = 50.0f;

		Frustum() : m_bUsingSphere(false), m_v3Center(0.0f, 0.0f, 0.0f), m_fRadius(0.0f), m_fHysteresisMargin(DEFAULT_HYSTERESIS_MARGIN) {}

		void SetHysteresisMargin(float fMargin) noexcept { m_fHysteresisMargin = fMargin; }
		[[nodiscard]] float GetHysteresisMargin() const noexcept { return m_fHysteresisMargin; }

		void BuildViewFrustum(D3DXMATRIX & mat);
		void BuildViewFrustum2(D3DXMATRIX & mat, float fNear, float fFar, float fFov, float fAspect, const D3DXVECTOR3 & vCamera, const D3DXVECTOR3 & vLook);
		
		[[nodiscard]] ViewState ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius) const;
		[[nodiscard]] ViewState ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin) const;

		[[nodiscard]] ViewState ViewVolumeTestAABB(const Vector3d &c_v3Min, const Vector3d &c_v3Max, const float c_fMargin = DEFAULT_HYSTERESIS_MARGIN) const;
		[[nodiscard]] bool IsBoxVisible(const Vector3d &c_v3Min, const Vector3d &c_v3Max, const float c_fMargin = DEFAULT_HYSTERESIS_MARGIN) const;
		[[nodiscard]] bool IsSphereVisible(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin = DEFAULT_HYSTERESIS_MARGIN) const;

		[[nodiscard]] const D3DXPLANE* GetPlanes() const noexcept { return m_plane; }
		[[nodiscard]] const D3DXPLANE& GetPlane(size_t index) const noexcept { return m_plane[index]; }

	private:
		bool m_bUsingSphere;
		D3DXVECTOR3 m_v3Center;
		float m_fRadius;
		float m_fHysteresisMargin;
		D3DXPLANE m_plane[6];
};
