#include "StdAfx.h"
#include "InstanceBase.h"
#include "PythonBackground.h"
#include "PythonNonPlayer.h"
#include "PythonPlayer.h"
#include "PythonCharacterManager.h"
#include "AbstractPlayer.h"
#include "AbstractApplication.h"
#include "packet.h"

#include "EterLib/StateManager.h"
#include "GameLib/ItemManager.h"
#include "Core/EventBus.h"
#include "InstanceControllers/IInstanceMountHorseController.h"
#include "Client/Bridge/StranglerInstanceFacade.h"
#include "ECS/ECSWorldRegistry.h"

BOOL RIDE_HORSE_ENABLE = TRUE;
const float c_fDefaultHorseRotationSpeed = 300.0f;

//////////////////////////////////////////////////////////////////////////////////////
// CInstanceBase Mount / Horse Operations (Kompozycyjna fasada do InstanceMountComponent)

void CInstanceBase::__AttachHorseSaddle()
{
	m_mountComponent.AttachSaddle(m_GraphicThingInstance);
}

void CInstanceBase::__DetachHorseSaddle()
{
	m_mountComponent.DetachSaddle(m_GraphicThingInstance);
}

BOOL CInstanceBase::IsNewMount()
{
	if (m_mountController && m_mountController->IsMounted())
		return m_mountController->IsNewMount() ? TRUE : FALSE;

	return m_mountComponent.IsNewMount();
}

BOOL CInstanceBase::IsMountingHorse()
{
	if (m_mountController && m_mountController->IsMounted())
		return TRUE;

	return m_mountComponent.IsMounting();
}

void CInstanceBase::MountHorse(UINT eRace)
{
	if (!m_mountController)
		m_mountController = UserInterface::InstanceControllers::CreateMountBoardController();

	if (m_mountController->IsMounted())
		m_mountController->Dismount();

	const TPixelPosition& c_rkPPosCur = m_GraphicThingInstance.NEW_GetCurPixelPositionRef();
	m_mountController->UpdateMountTransform(c_rkPPosCur.x, c_rkPPosCur.y, c_rkPPosCur.z, GetRotation());
	m_mountController->Mount(EterBase::EntityId(GetVirtualID()), eRace);

	m_mountComponent.Mount(
		m_GraphicThingInstance,
		c_rkPPosCur,
		eRace,
		ms_adwCRCAffectEffect[EFFECT_HIT]
	);

	SetRotationSpeed(c_fDefaultHorseRotationSpeed);
	Refresh(CRaceMotionData::NAME_WAIT, true);

	const DWORD dwVID = GetVirtualID();
	UserInterface::Core::EventBus::GetInstance().Publish(
		UserInterface::Core::MountStateChangedEvent(dwVID, eRace, 1)
	);
}

void CInstanceBase::DismountHorse()
{
	if (m_mountController && m_mountController->IsMounted())
		m_mountController->Dismount();

	m_mountComponent.Dismount(m_GraphicThingInstance);
	__InitializeRotationSpeed();
	Refresh(CRaceMotionData::NAME_WAIT, true);

	const DWORD dwVID = GetVirtualID();
	UserInterface::Core::EventBus::GetInstance().Publish(
		UserInterface::Core::MountStateChangedEvent(dwVID, 0, 0)
	);
}

BOOL CInstanceBase::CanAttackHorseLevel()
{
	if (!IsMountingHorse())
		return FALSE;

	if (m_mountController && m_mountController->IsMounted())
		return m_mountController->CanAttack() ? TRUE : FALSE;

	return m_mountComponent.CanAttack();
}

UINT CInstanceBase::GetHorseLevel()
{
	if (!IsMountingHorse())
		return 0;

	if (m_mountController && m_mountController->IsMounted())
		return m_mountController->GetMountLevel();

	return m_mountComponent.GetLevel();
}

void CInstanceBase::UpdateHorseMotion()
{
	if (m_mountController && m_mountController->IsMounted())
	{
		TPixelPosition curPos;
		NEW_GetPixelPosition(&curPos);
		m_mountController->UpdateMountTransform(curPos.x, curPos.y, curPos.z, GetRotation());
	}

	m_mountComponent.UpdateMotion();
}

void CInstanceBase::ProcessHorseDust()
{
	if (IsMountingHorse())
	{
		__AttachEffect(EFFECT_HORSE_DUST);
	}
}

int CInstanceBase::GetHorseMotionMode(BYTE byWeaponSubType)
{
	if (m_mountController && m_mountController->IsMounted())
		return m_mountController->GetHorseMotionMode(byWeaponSubType);

	return UserInterface::InstanceComponents::InstanceMountComponent::GetHorseMotionMode(byWeaponSubType);
}
