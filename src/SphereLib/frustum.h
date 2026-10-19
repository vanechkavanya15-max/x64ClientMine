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
#include "../Client/Actors/CollisionMathClean.h"
#include "../Client/Math/PODMath.h"
#include <cstddef>

enum ViewState
{
	VS_INSIDE,   // completely inside the frustum.
	VS_PARTIAL,  // partially inside and partially outside the frustum.
	VS_OUTSIDE   // completely outside the frustum
};

enum EFrustumPlane
{
	FRUSTUM_PLANE_NEAR = 0,
	FRUSTUM_PLANE_FAR = 1,
	FRUSTUM_PLANE_LEFT = 2,
	FRUSTUM_PLANE_RIGHT = 3,
	FRUSTUM_PLANE_BOTTOM = 4,
	FRUSTUM_PLANE_TOP = 5,
	FRUSTUM_PLANE_COUNT = 6
};

struct CleanPlane { float a, b, c, d; };

class Frustum 
{
	public:
		static constexpr float DEFAULT_HYSTERESIS_MARGIN = 50.0f;

		Frustum() : m_bUsingSphere(false), m_v3Center(0.0f, 0.0f, 0.0f), m_fRadius(0.0f), m_fHysteresisMargin(DEFAULT_HYSTERESIS_MARGIN) {}

		void SetHysteresisMargin(float fMargin) noexcept { m_fHysteresisMargin = fMargin; }
		[[nodiscard]] float GetHysteresisMargin() const noexcept { return m_fHysteresisMargin; }

		// Zunifikowana ekstrakcja 6 plaszczyzn z macierzy widoku i projekcji (row-major DirectX).
		// Parametr bNegatePlaneY pozwala na dostosowanie plaszczyzn do konwencji ze zwrocona osia Y.
		static void ExtractPlanes(const Client::Math::Mat4 & mat, CleanPlane * pPlanesOut, bool bNegatePlaneY = false);

		void BuildViewFrustum(const Client::Math::Mat4 & mat, bool bNegatePlaneY = false);
		void BuildViewFrustum(Client::Math::Mat4 & mat) { BuildViewFrustum(static_cast<const Client::Math::Mat4&>(mat), false); }
		void BuildViewFrustum2(const Client::Math::Mat4 & mat, float fNear, float fFar, float fFov, float fAspect, const Client::Collision::CleanVec3 & vCamera, const Client::Collision::CleanVec3 & vLook);
		void BuildViewFrustum2(Client::Math::Mat4 & mat, float fNear, float fFar, float fFov, float fAspect, const Client::Collision::CleanVec3 & vCamera, const Client::Collision::CleanVec3 & vLook)
		{
			BuildViewFrustum2(static_cast<const Client::Math::Mat4&>(mat), fNear, fFar, fFov, fAspect, vCamera, vLook);
		}
		
		[[nodiscard]] ViewState ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius) const;
		[[nodiscard]] ViewState ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin) const;
		[[nodiscard]] ViewState ViewVolumeTest(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin, bool bNegateY) const;
		[[nodiscard]] ViewState ViewVolumeTestTerrain(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin = 0.0f) const;

		[[nodiscard]] ViewState ViewVolumeTestAABB(const Vector3d &c_v3Min, const Vector3d &c_v3Max, const float c_fMargin = DEFAULT_HYSTERESIS_MARGIN, bool bNegateY = false) const;
		[[nodiscard]] bool IsBoxVisible(const Vector3d &c_v3Min, const Vector3d &c_v3Max, const float c_fMargin = DEFAULT_HYSTERESIS_MARGIN, bool bNegateY = false) const;
		[[nodiscard]] bool IsSphereVisible(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin = DEFAULT_HYSTERESIS_MARGIN, bool bNegateY = false) const;

		[[nodiscard]] const CleanPlane* GetPlanes() const noexcept { return m_plane; }
		[[nodiscard]] const CleanPlane& GetPlane(size_t index) const noexcept { return m_plane[index]; }

		[[nodiscard]] const Client::Collision::CleanVec3& GetCenter() const noexcept { return m_v3Center; }
		[[nodiscard]] float GetRadius() const noexcept { return m_fRadius; }
		[[nodiscard]] bool IsUsingSphere() const noexcept { return m_bUsingSphere; }

	private:
		bool m_bUsingSphere;
		Client::Collision::CleanVec3 m_v3Center;
		float m_fRadius;
		float m_fHysteresisMargin;
		CleanPlane m_plane[6];
};

using CFrustum = Frustum;
