// Camera.cpp: implementation of the CCamera class.
//
//////////////////////////////////////////////////////////////////////

#include "StdAfx.h"
#include "EterBase/Utils.h"
#include "EterBase/Timer.h"
#include "FrameTimer.h"
#include "Camera.h"
#include <cmath>
#include <algorithm>

const float c_fDefaultResistance = 0.3f;

CCameraManager aCameraManager;	// CCameraManager Instance

float CCamera::GetDeltaTime()
{
	float dt = FrameTimer::Instance().GetDeltaTime();
	if (dt <= 0.0f || std::isnan(dt))
	{
		dt = CTimer::Instance().GetElapsedSecond();
	}
	if (dt <= 0.0001f || dt > 0.2f || std::isnan(dt))
	{
		dt = 1.0f / 60.0f;
	}
	return dt;
}

void CCamera::SetCameraMaxDistance(float fMax)
{
	CAMERA_MAX_DISTANCE = fMax;
}

float CCamera::GetTargetHeight()
{
	return m_fTarget_;
}

void CCamera::SetTargetHeight(float fTarget)
{
	m_fTarget_=fTarget;	
}


//////////////////////////////////////////////////////////////////////////
// CCamera
//////////////////////////////////////////////////////////////////////////


CCamera::CCamera() :
m_fEyeGroundHeightRatio(0.3f),
m_fTargetHeightLimitRatio(2.0f),
m_fResistance(c_fDefaultResistance),
m_isLock(false)
{
	m_fDistance						= 1.0f;
	m_eCameraState					= CAMERA_STATE_NORMAL;
	m_eCameraStatePrev				= CAMERA_STATE_NORMAL;
	m_ulNumScreenBuilding			= 0;

	m_fPitchSum = 0.0f;
	m_fRollSum = 0.0f;

	m_fTerrainCollisionRadius		= 50.0f;
	m_fObjectCollisionRadius		= 50.0f;

	m_bDrag							= false;

	m_lMousePosX					= -1;
	m_lMousePosY					= -1;

	m_fTarget_						= CAMERA_TARGET_STANDARD;

	m_v3AngularAcceleration			= D3DXVECTOR3(0.0f, 0.0f, 0.0f);
	m_v3AngularVelocity				= D3DXVECTOR3(0.0f, 0.0f, 0.0f);

	m_bProcessTerrainCollision		= true;

    SetViewParams(D3DXVECTOR3(0.0f,0.0f,1.0f), D3DXVECTOR3(0.0f,0.0f,0.0f), D3DXVECTOR3(0.0f,1.0f,0.0f));
}

CCamera::~CCamera()
{
}

void CCamera::Lock()
{
	m_isLock = true;
}

void CCamera::Unlock()
{
	m_isLock = false;
}

bool CCamera::IsLock()
{
	return m_isLock;
}

void CCamera::SetResistance(float fResistance)
{
	m_fResistance = c_fDefaultResistance * fResistance;
}

void CCamera::Wheel(int nLen)
{
	if (IsLock())
		return;

	m_v3AngularVelocity.y = (float)(nLen) * m_fResistance;
}

void CCamera::BeginDrag(int nMouseX, int nMouseY)
{
	if (IsLock())
		return;

	m_bDrag = true;
	m_lMousePosX = nMouseX;
	m_lMousePosY = nMouseY;
	m_fPitchSum = 0.0f;
	m_fRollSum = 0.0f;
}

bool CCamera::IsDraging()
{
	if (IsLock())
		return false;

	return m_bDrag;
}

bool CCamera::EndDrag()
{
	if (IsLock())
		return false;

	m_bDrag = false;

	float fSum=sqrt(m_fPitchSum*m_fPitchSum+m_fRollSum*m_fRollSum);

	m_fPitchSum = 0.0f;
	m_fRollSum = 0.0f;
	
	if (fSum<1.0f)
		return false;

	return true;
}

bool CCamera::Drag(int nMouseX, int nMouseY, LPPOINT lpReturnPoint)
{
	if (IsLock())
		return false;

	if (!m_bDrag)
	{
		m_lMousePosX = nMouseX;
		m_lMousePosY = nMouseY;
		lpReturnPoint->x = m_lMousePosX;
		lpReturnPoint->y = m_lMousePosY;
		return false;
	}
	
	long lMouseX = nMouseX;
	long lMouseY = nMouseY;
	
	float fNewPitchVelocity = (float)(lMouseY - m_lMousePosY) * m_fResistance;
	float fNewRotationVelocity = (float)(lMouseX - m_lMousePosX) * m_fResistance;

	m_fPitchSum += fNewPitchVelocity;
	m_fRollSum += fNewRotationVelocity;
	
	
	if (CAMERA_STATE_CANTGOLEFT == GetCameraState())
		fNewRotationVelocity = fMAX(0.0f, fNewRotationVelocity);
	if (CAMERA_STATE_CANTGORIGHT == GetCameraState())
		fNewRotationVelocity = fMIN(0.0f, fNewRotationVelocity);
	if (CAMERA_STATE_CANTGODOWN == GetCameraState())
		fNewPitchVelocity = fMAX(0.0f, fNewPitchVelocity);
	
	m_v3AngularVelocity.x = fNewRotationVelocity;
	m_v3AngularVelocity.z = fNewPitchVelocity;

	lpReturnPoint->x = m_lMousePosX;
	lpReturnPoint->y = m_lMousePosY;
	return true;
}

//////////////////////////////////////////////////////////////////////////
// Update

void CCamera::SetCameraState(eCameraState eNewCameraState)
{
	if (eNewCameraState == m_eCameraState)
		return;

	m_eCameraStatePrev = m_eCameraState;
	m_eCameraState = eNewCameraState;

/*
	if ((CAMERA_STATE_NORMAL == m_eCameraStatePrev))
	{
		m_fDistanceBackup = m_fDistance;
		m_fPitchBackup = m_fPitch;
		m_fRollBackup = m_fRoll;
	}
	else if ((CAMERA_STATE_CANTGODOWN == m_eCameraStatePrev) && (CAMERA_STATE_CANTGODOWN == m_eCameraState) )
	{
		m_v3EyeBackup = m_v3Eye;
	}
*/
}

void CCamera::IncreaseNumSrcreenBuilding()
{
	++m_ulNumScreenBuilding;
}

void CCamera::ResetNumScreenBuilding()
{
	m_ulNumScreenBuilding = 0;
}

//////////////////////////////////////////////////////////////////////////
// Property
void CCamera::SetViewParams( const D3DXVECTOR3 &v3Eye, const D3DXVECTOR3& v3Target, const D3DXVECTOR3& v3Up)
{
	if (IsLock())
		return;

    // Set attributes for the view matrix
    m_v3Eye = v3Eye;
    m_v3Target = v3Target;
    m_v3Up = v3Up;

	// Avoid screen flipping with nan values in the view matrix
	if (!m_v3Eye.y)
		m_v3Eye.y = 0.1f;

	SetViewMatrix();
}

void CCamera::SetEye(const D3DXVECTOR3 & v3Eye)
{
	if (IsLock())
		return;

    m_v3Eye = v3Eye;

	SetViewMatrix();
}

void CCamera::SetTarget(const D3DXVECTOR3 & v3Target)
{
	if (IsLock())
		return;

    m_v3Target = v3Target;

	SetViewMatrix();
}

void CCamera::SetUp(const D3DXVECTOR3 & v3Up)
{
	if (IsLock())
		return;

    m_v3Up = v3Up;

	SetViewMatrix();
}

bool IsNaN(const D3DXVECTOR3& v)
{
	return std::isnan(v.x) || std::isnan(v.y) || std::isnan(v.z);
}

void CCamera::SetViewMatrix()
{
	// Sprawdzenie integralnosci wektorow zrodlowych
	if (IsNaN(m_v3Eye) || IsNaN(m_v3Target) || IsNaN(m_v3Up))
	{
		m_v3Eye = D3DXVECTOR3(0.0f, -500.0f, 200.0f);
		m_v3Target = D3DXVECTOR3(0.0f, 0.0f, 100.0f);
		m_v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
	}

	// Obliczenie kierunku widoku
	m_v3View = m_v3Target - m_v3Eye;
	float viewLenSq = D3DXVec3LengthSq(&m_v3View);
	if (viewLenSq < 1e-6f || std::isnan(viewLenSq) || IsNaN(m_v3View))
	{
		// Eye jest zbyt blisko targetu (np. scisniecie przez kolizje z terenem)
		// Odsun eye na bezpieczna odleglosc wzdluz bezpiecznego wektora
		m_v3View = D3DXVECTOR3(0.0f, -1.0f, 0.5f);
		D3DXVec3Normalize(&m_v3View, &m_v3View);
		m_v3Eye = m_v3Target - m_v3View * 10.0f;
		viewLenSq = 100.0f;
	}

	m_fDistance = std::sqrt(viewLenSq);
	if (std::isnan(m_fDistance) || m_fDistance < 1e-4f)
	{
		m_fDistance = 10.0f;
	}
	m_v3View /= m_fDistance;

	// v3CenterRay jest odwrotnoscia wektora widoku
	D3DXVECTOR3 v3CenterRay = -m_v3View;

	// Bezpieczne obliczenie roll
	CalculateRoll();

	// Normalizacja i walidacja Up
	float upLenSq = D3DXVec3LengthSq(&m_v3Up);
	if (upLenSq < 1e-6f || std::isnan(upLenSq) || IsNaN(m_v3Up))
	{
		m_v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
	}
	else
	{
		m_v3Up /= std::sqrt(upLenSq);
	}

	// Wektor Cross (Right) = Up x View
	D3DXVec3Cross(&m_v3Cross, &m_v3Up, &m_v3View);
	float crossLenSq = D3DXVec3LengthSq(&m_v3Cross);
	if (crossLenSq < 1e-6f || std::isnan(crossLenSq) || IsNaN(m_v3Cross))
	{
		// Gimbal lock: Up i View sa niemal rownolegle (widok pionowo w dol/gore)
		D3DXVECTOR3 refAxis = (fabsf(m_v3View.z) > 0.9f) ? D3DXVECTOR3(0.0f, 1.0f, 0.0f) : D3DXVECTOR3(0.0f, 0.0f, 1.0f);
		D3DXVec3Cross(&m_v3Cross, &refAxis, &m_v3View);
		crossLenSq = D3DXVec3LengthSq(&m_v3Cross);
		if (crossLenSq < 1e-6f || std::isnan(crossLenSq))
		{
			m_v3Cross = D3DXVECTOR3(1.0f, 0.0f, 0.0f);
		}
		else
		{
			m_v3Cross /= std::sqrt(crossLenSq);
		}
	}
	else
	{
		m_v3Cross /= std::sqrt(crossLenSq);
	}

	// Rekalkulacja Up = View x Cross dla idealnej ortonormalnosci
	D3DXVec3Cross(&m_v3Up, &m_v3View, &m_v3Cross);
	upLenSq = D3DXVec3LengthSq(&m_v3Up);
	if (upLenSq > 1e-6f && !std::isnan(upLenSq))
	{
		m_v3Up /= std::sqrt(upLenSq);
	}
	else
	{
		m_v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
	}

	// Ochrona przed niepozadanym odwracaniem wektora Up
	if (m_v3Up.z < -0.1f && fabsf(m_v3View.z) < 0.999f)
	{
		m_v3Up = -m_v3Up;
		m_v3Cross = -m_v3Cross;
	}

	// Obliczenie pitch z zabezpieczeniem przed NaN
	D3DXVECTOR3 zAxis = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
	float dotUpZ = D3DXVec3Dot(&m_v3Up, &zAxis);
	dotUpZ = std::clamp(dotUpZ, -1.0f, 1.0f);
	m_fPitch = acosf(dotUpZ);
	if (std::isnan(m_fPitch))
	{
		m_fPitch = 0.0f;
	}
	m_fPitch *= (180.0f / D3DX_PI);
	if (m_v3View.z > 0.0f)
		m_fPitch = -m_fPitch;
	m_fPitch = std::clamp(m_fPitch, -89.5f, 89.5f);

	// Wyznaczenie macierzy widoku
	D3DXMatrixLookAtRH(&m_matView, &m_v3Eye, &m_v3Target, &m_v3Up);

	// Odwracalnosc macierzy
	float fDet = D3DXMatrixDeterminant(&m_matView);
	if (std::isnan(fDet) || fabsf(fDet) < 1e-6f)
	{
		D3DXMatrixIdentity(&m_matInverseView);
	}
	else
	{
		D3DXMatrixInverse(&m_matInverseView, &fDet, &m_matView);
	}

	m_matBillboard = m_matInverseView;
	m_matBillboard._41 = 0.0f;
	m_matBillboard._42 = 0.0f;
	m_matBillboard._43 = 0.0f;

	m_ViewRay.SetStartPoint(m_v3Target);
	m_ViewRay.SetDirection(v3CenterRay, m_fDistance);

	m_kCameraBottomToTerrainRay.SetStartPoint(m_v3Eye);
	m_kCameraFrontToTerrainRay.SetStartPoint(m_v3Eye);
	m_kCameraBackToTerrainRay.SetStartPoint(m_v3Eye);
	m_kCameraLeftToTerrainRay.SetStartPoint(m_v3Eye);
	m_kCameraRightToTerrainRay.SetStartPoint(m_v3Eye);
	m_kTargetToCameraBottomRay.SetStartPoint(m_v3Target);

	m_kCameraBottomToTerrainRay.SetDirection(-m_v3Up, 2.0f * m_fTerrainCollisionRadius);
	m_kCameraFrontToTerrainRay.SetDirection(m_v3View, 4.0f * m_fTerrainCollisionRadius);
	m_kCameraBackToTerrainRay.SetDirection(-m_v3View, m_fTerrainCollisionRadius);
	m_kCameraLeftToTerrainRay.SetDirection(-m_v3Cross, 3.0f * m_fTerrainCollisionRadius);
	m_kCameraRightToTerrainRay.SetDirection(m_v3Cross, 3.0f * m_fTerrainCollisionRadius);

	D3DXVECTOR3 temp = (v3CenterRay - m_fTerrainCollisionRadius * m_v3Up);
	float tempLen = D3DXVec3Length(&temp);
	if (tempLen > 1e-4f && !std::isnan(tempLen))
		m_kTargetToCameraBottomRay.SetDirection(temp, tempLen);

	m_kLeftObjectCollisionRay.SetStartPoint(m_v3Target);
	m_kTopObjectCollisionRay.SetStartPoint(m_v3Target);
	m_kRightObjectCollisionRay.SetStartPoint(m_v3Target);
	m_kBottomObjectCollisionRay.SetStartPoint(m_v3Target);

	D3DXVECTOR3 val1 = (v3CenterRay + m_fObjectCollisionRadius * m_v3Cross);
	float val1Len = D3DXVec3Length(&val1);
	if (val1Len > 1e-4f && !std::isnan(val1Len))
		m_kLeftObjectCollisionRay.SetDirection(val1, val1Len);

	D3DXVECTOR3 val2 = (v3CenterRay - m_fObjectCollisionRadius * m_v3Cross);
	float val2Len = D3DXVec3Length(&val2);
	if (val2Len > 1e-4f && !std::isnan(val2Len))
		m_kRightObjectCollisionRay.SetDirection(val2, val2Len);

	D3DXVECTOR3 val3 = (v3CenterRay + m_fObjectCollisionRadius * m_v3Up);
	float val3Len = D3DXVec3Length(&val3);
	if (val3Len > 1e-4f && !std::isnan(val3Len))
		m_kTopObjectCollisionRay.SetDirection(val3, val3Len);

	D3DXVECTOR3 val4 = (v3CenterRay - m_fObjectCollisionRadius * m_v3Up);
	float val4Len = D3DXVec3Length(&val4);
	if (val4Len > 1e-4f && !std::isnan(val4Len))
		m_kBottomObjectCollisionRay.SetDirection(val4, val4Len);
}

void CCamera::Move(const D3DXVECTOR3 & v3Displacement)
{
	if (IsLock())
		return;

	m_v3Eye += v3Displacement;
	m_v3Target += v3Displacement; 

	SetViewMatrix();
}

void CCamera::Zoom(float fRatio)
{
	if (IsLock())
		return;

	if (fRatio == 1.0f)
		return;

	D3DXVECTOR3 v3Temp = m_v3Eye - m_v3Target;
	v3Temp *= fRatio;
	m_v3Eye = v3Temp + m_v3Target;

	SetViewMatrix();
}

void CCamera::MoveAlongView(float fDistance, float fDeltaTime) 
{
	if (IsLock())
		return;

	if (fDeltaTime > 0.0f)
		fDistance *= (fDeltaTime * 60.0f);

	float lenSq = D3DXVec3LengthSq(&m_v3View);
	D3DXVECTOR3 v3Temp;
	if (lenSq > 1e-6f && !std::isnan(lenSq))
		v3Temp = m_v3View / std::sqrt(lenSq);
	else
		v3Temp = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
	
	m_v3Eye += v3Temp * fDistance;
	m_v3Target += v3Temp * fDistance;
	
	SetViewMatrix();
}

void CCamera::MoveAlongCross(float fDistance, float fDeltaTime) 
{
	if (IsLock())
		return;

	if (fDeltaTime > 0.0f)
		fDistance *= (fDeltaTime * 60.0f);

	float lenSq = D3DXVec3LengthSq(&m_v3Cross);
	D3DXVECTOR3 v3Temp;
	if (lenSq > 1e-6f && !std::isnan(lenSq))
		v3Temp = m_v3Cross / std::sqrt(lenSq);
	else
		v3Temp = D3DXVECTOR3(1.0f, 0.0f, 0.0f);

	m_v3Eye += v3Temp * fDistance;
	m_v3Target += v3Temp * fDistance;

	SetViewMatrix();
}

void CCamera::MoveAlongUp(FLOAT fDistance, float fDeltaTime) 
{
	if (IsLock())
		return;

	if (fDeltaTime > 0.0f)
		fDistance *= (fDeltaTime * 60.0f);

	float lenSq = D3DXVec3LengthSq(&m_v3Up);
	D3DXVECTOR3 v3Temp;
	if (lenSq > 1e-6f && !std::isnan(lenSq))
		v3Temp = m_v3Up / std::sqrt(lenSq);
	else
		v3Temp = D3DXVECTOR3(0.0f, 0.0f, 1.0f);

	m_v3Target += v3Temp * fDistance;
	m_v3Eye += v3Temp * fDistance;
	SetViewMatrix();
}

void CCamera::MoveLateral(float fDistance, float fDeltaTime)
{
	if (IsLock())
		return;

	MoveAlongCross(fDistance, fDeltaTime);
}

void CCamera::MoveFront(float fDistance, float fDeltaTime) 
{	
	if (IsLock())
		return;

	if (fDeltaTime > 0.0f)
		fDistance *= (fDeltaTime * 60.0f);

	D3DXVECTOR3 v3Temp = D3DXVECTOR3(m_v3View.x, m_v3View.y, 0.0f);
	float lenSq = D3DXVec3LengthSq(&v3Temp);
	if (lenSq > 1e-6f && !std::isnan(lenSq))
	{
		v3Temp /= std::sqrt(lenSq);
	}
	else
	{
		// Fallback jesli kamera patrzy pionowo w dol/gore
		v3Temp = D3DXVECTOR3(m_v3Up.x, m_v3Up.y, 0.0f);
		lenSq = D3DXVec3LengthSq(&v3Temp);
		if (lenSq > 1e-6f && !std::isnan(lenSq))
			v3Temp /= std::sqrt(lenSq);
		else
			v3Temp = D3DXVECTOR3(0.0f, 1.0f, 0.0f);
	}

	m_v3Eye += v3Temp * fDistance; 
	m_v3Target += v3Temp * fDistance;
	
	SetViewMatrix();	
}

void CCamera::MoveVertical(float fDistance, float fDeltaTime) 
{
	if (IsLock())
		return;

	if (fDeltaTime > 0.0f)
		fDistance *= (fDeltaTime * 60.0f);

	m_v3Eye.z += fDistance;
	m_v3Target.z += fDistance;

	SetViewMatrix();
}

//void CCamera::RotateUpper(float fDegree)
//{
//	D3DXMATRIX matRot;
//	D3DXMatrixRotationAxis(&matRot, &m_v3Cross, -D3DXToRadian(fDegree));
//	D3DXVec3TransformCoord(&m_v3View, &m_v3View, &matRot) ;
//    D3DXVec3Cross(&m_v3Up, &m_v3View, &m_v3Cross);
//
//	m_v3Target = m_v3Eye + m_v3View;
//
//	SetViewMatrix() ;
//}

void CCamera::RotateEyeAroundTarget(float fPitchDegree, float fRollDegree, float fDeltaTime) 
{
	if (IsLock())
		return;

	if (fDeltaTime > 0.0f)
	{
		float fTimeScale = fDeltaTime * 60.0f;
		fPitchDegree *= fTimeScale;
		fRollDegree *= fTimeScale;
	}

	// Bezpieczna plynna klamra pitch (-89.5 do +89.5 stopnia) bez ryzyka gimbal locka i odwracania Up
	constexpr float PITCH_MIN = -89.5f;
	constexpr float PITCH_MAX = 89.5f;

	float fTargetPitch = std::clamp(m_fPitch + fPitchDegree, PITCH_MIN, PITCH_MAX);
	fPitchDegree = fTargetPitch - m_fPitch;

	D3DXMATRIX matRot, matRotPitch, matRotRoll;
	D3DXMatrixRotationAxis(&matRotPitch, &m_v3Cross, D3DXToRadian(fPitchDegree));
	D3DXMatrixRotationZ(&matRotRoll, -D3DXToRadian(fRollDegree));
	matRot = matRotPitch * matRotRoll;

	D3DXVECTOR3 v3Temp = m_v3Eye - m_v3Target;
	float distSq = D3DXVec3LengthSq(&v3Temp);
	if (distSq < 1e-4f || std::isnan(distSq))
	{
		float fallbackDist = (m_fDistance > 10.0f) ? m_fDistance : 100.0f;
		v3Temp = D3DXVECTOR3(0.0f, -fallbackDist, 50.0f);
	}

	D3DXVec3TransformCoord(&m_v3Eye, &v3Temp, &matRot);
	m_v3Eye += m_v3Target;

	// Zachowaj Up wektor jako czysty wektor gora (0, 0, 1) bez odwracania
	SetUp(D3DXVECTOR3(0.0f, 0.0f, 1.0f));

	m_fRoll += fRollDegree;

	if (m_fRoll > 360.0f)
		m_fRoll -= 360.0f;
	else if (m_fRoll < -360.0f)
		m_fRoll += 360.0f;
}

void CCamera::RotateEyeAroundPoint(const D3DXVECTOR3 & v3Point, float fPitchDegree, float fRollDegree, float fDeltaTime)
{
//	if (IsLock())
//		return;

	if (fDeltaTime > 0.0f)
	{
		float fTimeScale = fDeltaTime * 60.0f;
		fPitchDegree *= fTimeScale;
		fRollDegree *= fTimeScale;
	}

	D3DXMATRIX matRot, matRotPitch, matRotRoll;

	D3DXMatrixRotationAxis(&matRotPitch, &m_v3Cross, D3DXToRadian(fPitchDegree));

	D3DXMatrixRotationZ(&matRotRoll, -D3DXToRadian(fRollDegree));
	matRot = matRotPitch * matRotRoll;
	
	D3DXVECTOR3 v3Temp = m_v3Eye - v3Point;
	D3DXVec3TransformCoord(&m_v3Eye, &v3Temp, &matRot);
	m_v3Eye += v3Point;
	
	const auto vv2 = (v3Temp + m_v3Up);
	D3DXVec3TransformCoord(&m_v3Up, &vv2, &matRot);
	m_v3Up -= (m_v3Eye - v3Point);
	
	v3Temp = m_v3Target - v3Point;
	D3DXVec3TransformCoord(&m_v3Target, &v3Temp, &matRot);
	m_v3Target += v3Point;

	SetViewMatrix();
}

void CCamera::Pitch(const float fPitchDelta)
{
//	if (IsLock())
//		return;

	RotateEyeAroundTarget(fPitchDelta, 0.0f);
}

void CCamera::Roll(const float fRollDelta)
{
//	if (IsLock())
//		return;

	RotateEyeAroundTarget(0.0f, fRollDelta);
}

void CCamera::SetDistance(const float fdistance)
{
//	if (IsLock())
//		return;

	Zoom(fdistance/m_fDistance);
}

void CCamera::CalculateRoll()
{
	D3DXVECTOR2 v2ViewXY;
	v2ViewXY.x = m_v3View.x;
	v2ViewXY.y = m_v3View.y;
	float lenSq = D3DXVec2LengthSq(&v2ViewXY);
	if (lenSq < 1e-6f || std::isnan(lenSq))
	{
		// Gdy kamera patrzy pionowo wzdluz osi Z, roll nie ulega zmianie
		return;
	}
	v2ViewXY /= std::sqrt(lenSq);

	const auto vv = D3DXVECTOR2(0.0f, 1.0f);
	float fDot = D3DXVec2Dot(&v2ViewXY, &vv);
	fDot = std::clamp(fDot, -1.0f, 1.0f);
	fDot = acosf(fDot);
	if (std::isnan(fDot))
	{
		fDot = 0.0f;
	}
	fDot *= (180.0f / D3DX_PI);
	float fCross = D3DXVec2CCW(&v2ViewXY, &vv);
	if (0.0f > fCross)
	{
		fDot = -fDot;
	}

	m_fRoll = fDot;
}

//////////////////////////////////////////////////////////////////////////
// CCameraMananger
//////////////////////////////////////////////////////////////////////////

CCameraManager::CCameraManager() :
m_pCurrentCamera(NULL),
m_pPreviousCamera(NULL)
{
	AddCamera(DEFAULT_PERSPECTIVE_CAMERA);
	AddCamera(DEFAULT_ORTHO_CAMERA);

	SetCurrentCamera(DEFAULT_PERSPECTIVE_CAMERA);
}

CCameraManager::~CCameraManager()
{
	for (TCameraMap::iterator itor = m_CameraMap.begin(); itor != m_CameraMap.end(); ++itor)
	{
		delete (*itor).second;
	}
	m_CameraMap.clear();
}

CCamera * CCameraManager::GetCurrentCamera()
{
	if (!m_pCurrentCamera)
		assert(false);
	return m_pCurrentCamera;
}

void CCameraManager::SetCurrentCamera(unsigned char ucCameraNum)
{
	if (m_pCurrentCamera != m_CameraMap[ucCameraNum])
		m_pPreviousCamera = m_pCurrentCamera;

	m_pCurrentCamera = m_CameraMap[ucCameraNum];
}

bool CCameraManager::isCurrentCamera(unsigned char ucCameraNum)
{
	if (m_CameraMap[ucCameraNum] == m_pCurrentCamera)
		return true;
	return false;
}

// 잡스러운 함수들...
bool CCameraManager::AddCamera(unsigned char ucCameraNum)
{
	if(m_CameraMap.end() != m_CameraMap.find(ucCameraNum))
		return false;
	m_CameraMap.insert(TCameraMap::value_type(ucCameraNum, new CCamera));
	return true;
}

unsigned char CCameraManager::GetCurrentCameraNum()
{
	if (!m_pCurrentCamera)
		return NO_CURRENT_CAMERA;
	for (TCameraMap::iterator itor = m_CameraMap.begin(); itor != m_CameraMap.end(); ++itor)
		if(m_pCurrentCamera == (*itor).second)
			return (*itor).first;
	return NO_CURRENT_CAMERA;
}

bool CCameraManager::isTerrainCollisionEnable()
{
	return m_pCurrentCamera->isTerrainCollisionEnable();
}

void CCameraManager::SetTerrainCollision(bool bEnable)
{
	m_pCurrentCamera->SetTerrainCollision(bEnable);
}
