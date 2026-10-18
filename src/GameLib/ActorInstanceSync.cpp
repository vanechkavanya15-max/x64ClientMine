#include "StdAfx.h"
#include "ActorInstance.h"
#include "RaceData.h"



void CActorInstance::__Push(int x, int y)
{
	if (IsResistFallen())
		return;

	const D3DXVECTOR3& c_rv3Src = GetPosition();
	const D3DXVECTOR3 c_v3Dst(static_cast<float>(x), static_cast<float>(-y), c_rv3Src.z);
	const D3DXVECTOR3 c_v3Delta = c_v3Dst - c_rv3Src;

	const float lenSq = c_v3Delta.x * c_v3Delta.x + c_v3Delta.y * c_v3Delta.y;
	if (lenSq < 1.0f)
		return;

	IPhysicsWorld* pWorld = IPhysicsWorld::GetPhysicsWorld();
	if (!pWorld)
		return;

	// Adaptacyjna liczba krokow (1 krok na 25 jednostek swiata, maksymalnie 32 iteracje zamiast 100)
	const float len = std::sqrt(lenSq);
	const int steps = (std::clamp)(static_cast<int>(len * (1.0f / 25.0f)), 1, 32);
	const D3DXVECTOR3 inc = c_v3Delta * (1.0f / static_cast<float>(steps));

	D3DXVECTOR3 v3Probe = c_rv3Src;
	for (int i = 0; i < steps; ++i)
	{
		if (pWorld->isPhysicalCollision(v3Probe))
		{
			ResetBlendingPosition();
			return;
		}
		v3Probe += inc;
	}

	SetBlendingPosition(c_v3Dst);

	if (!IsUsingSkill() && lenSq > (150.0f * 150.0f))
	{
		InterceptOnceMotion(CRaceMotionData::NAME_DAMAGE_FLYING);
		PushOnceMotion(CRaceMotionData::NAME_STAND_UP);
	}
}

void CActorInstance::TEMP_Push(int x, int y)
{
	__Push(x, y);
}

bool CActorInstance::__IsSyncing()
{
	if (IsDead())
		return TRUE;

	if (IsStun())
		return TRUE;

	if (IsPushing())
		return TRUE;

	return FALSE;
}

bool CActorInstance::IsPushing()
{
	return m_PhysicsObject.isBlending();
}
