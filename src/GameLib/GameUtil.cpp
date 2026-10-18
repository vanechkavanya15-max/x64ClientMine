#include "StdAfx.h"
#include "EterLib/lineintersect_utils.h"
#include "Client/Actors/CollisionMathClean.h"

#include "GameUtil.h"

bool DetectCollisionDynamicZCylinderVSDynamicZCylinder(const CDynamicSphereInstance & c_rSphere1, const CDynamicSphereInstance & c_rSphere2)
{
	Client::Collision::CleanDynamicSphere cyl1{
		{c_rSphere1.v3LastPosition.x, c_rSphere1.v3LastPosition.y, c_rSphere1.v3LastPosition.z},
		{c_rSphere1.v3Position.x, c_rSphere1.v3Position.y, c_rSphere1.v3Position.z},
		c_rSphere1.fRadius
	};
	Client::Collision::CleanDynamicSphere cyl2{
		{c_rSphere2.v3LastPosition.x, c_rSphere2.v3LastPosition.y, c_rSphere2.v3LastPosition.z},
		{c_rSphere2.v3Position.x, c_rSphere2.v3Position.y, c_rSphere2.v3Position.z},
		c_rSphere2.fRadius
	};

	return Client::Collision::CleanDetectCollisionDynamicZCylinder(cyl1, cyl2);
}

bool DetectCollisionDynamicSphereVSDynamicSphere(const CDynamicSphereInstance & c_rSphere1, const CDynamicSphereInstance & c_rSphere2)
{
	Client::Collision::CleanDynamicSphere s1{
		{c_rSphere1.v3LastPosition.x, c_rSphere1.v3LastPosition.y, c_rSphere1.v3LastPosition.z},
		{c_rSphere1.v3Position.x, c_rSphere1.v3Position.y, c_rSphere1.v3Position.z},
		c_rSphere1.fRadius
	};
	Client::Collision::CleanDynamicSphere s2{
		{c_rSphere2.v3LastPosition.x, c_rSphere2.v3LastPosition.y, c_rSphere2.v3LastPosition.z},
		{c_rSphere2.v3Position.x, c_rSphere2.v3Position.y, c_rSphere2.v3Position.z},
		c_rSphere2.fRadius
	};

	return Client::Collision::CleanDetectCollisionDynamicSphere(s1, s2);
}

///////////////////////////////////////////////////////////////////////////////////////////////////

bool IsCWAcuteAngle(float begin, float end)
{
	// 360 - src + dest		// 시계 반대 방향
	// src - dest			// 시계 방향
	return ((360.0f - begin + end) > (begin - end));
}

bool IsCCWAcuteAngle(float begin, float end)
{
	// abs(360 - dest + src) 	// 시계 방향
	// dest - src				// 시계 반대 방향
	int fValue = abs((int) (360.0f - end + begin));
	return fValue >= (end - begin) ? true : false;
}

bool IsCWRotation(float begin, float end)
{
	return !IsCCWRotation(begin, end);
}

bool IsCCWRotation(float begin, float end)
{
	//     180
	// 270      90
	//      0
	//
	// 시계 반대
	return (begin - end < 0);
}

float GetInterpolatedRotation(float begin, float end, float curRate)
{
	if (IsCCWRotation(begin, end))
	{
		if (IsCCWAcuteAngle(begin, end))
			return GetLinearInterpolation(begin, end, curRate);

		return (360.0f + GetLinearInterpolation(begin, end - 360.0f, curRate));
	}

	if (IsCWAcuteAngle(begin, end))
		return GetLinearInterpolation(begin, end, curRate);

	return GetLinearInterpolation(begin, end + 360.0f, curRate);								
}

float GetDegreeFromPosition(float x, float y)
{
	D3DXVECTOR3 vtDir(floor(x), floor(y), 0.0f);
	D3DXVec3Normalize(&vtDir, &vtDir);

	D3DXVECTOR3 vtStan(0, -1, 0);
	float ret = D3DXToDegree(acosf(D3DXVec3Dot(&vtDir, &vtStan)));

	if (vtDir.x < 0.0f)
		ret = 360.0f - ret;

	return ret;
}

float GetDegreeFromPosition2(float sx, float sy, float ex, float ey)
{
	return GetDegreeFromPosition(ex - sx, ey - sy);
}

float GetDegreeDifference(float fSource, float fTarget)
{
	if (fSource < 180.0f)
	{
		if (fTarget < fSource)
			return fSource - fTarget;

		else if(fTarget - fSource > 180.0f)
			return (360.0f - (fTarget - fSource));

		return fTarget - fSource;
	}
	else
	{
		if (fTarget > fSource)
			return fTarget - fSource;

		else if (fSource - fTarget > 180.0f)
		{
			return (360.0f - (fSource - fTarget));
		}

		return fSource - fTarget;
	}
}
int GetRotatingDirection(float fSource, float fTarget)
{
	if (fSource < 180.0f)
	{
		if (fTarget < fSource)
			return DEGREE_DIRECTION_RIGHT;

		else if((360.0f - fTarget) + fSource < 180.0f)
			return DEGREE_DIRECTION_RIGHT;

		return DEGREE_DIRECTION_LEFT;
	}
	else
	{
		if (fTarget > fSource)
		{
			return DEGREE_DIRECTION_LEFT;
		}
		else if ((360.0f - fSource) + fTarget < 180.0f)
		{
			return DEGREE_DIRECTION_LEFT;
		}

		return DEGREE_DIRECTION_RIGHT;
	}
}

///////////////////////////////////////////////////////////////////////////////////////////////////

float CameraRotationToCharacterRotation(float fCameraRotation)
{
	return fmod((540.0f - fCameraRotation), 360.0f);
}

float CharacterRotationToCameraRotation(float fCharacterRotation)
{
	return fmod((540.0f - fCharacterRotation), 360.0f);
}

