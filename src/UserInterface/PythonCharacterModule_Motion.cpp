#include "StdAfx.h"
#include "PythonCharacterManager.h"
#include "PythonCharacterModule.h"

// ============================================================================
// Podsystem Ruchu, Rotacji i Animacji Bojowych (PythonCharacterModule_Motion.cpp)
// ============================================================================

PyObject * chrSetMotionMode(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (nargs != 1)
		return Py_BadArgument();

	if (!PyLong_Check(poArgs[0]))
		return Py_BadArgument();

	int iMotionMode = PyLong_AsLong(poArgs[0]);

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		Py_RETURN_NONE;

	pkInst->SetMotionMode(iMotionMode);
	Py_RETURN_NONE;
}

PyObject * chrSetMotion(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	return chrSetMotionMode(poSelf, poArgs, nargs);
}

PyObject * chrSetLoopMotion(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (nargs != 1)
		return Py_BadArgument();

	if (!PyLong_Check(poArgs[0]))
		return Py_BadArgument();

	int iMotionIndex = PyLong_AsLong(poArgs[0]);

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		Py_RETURN_NONE;

	pkInst->SetLoopMotion(WORD(iMotionIndex));
	Py_RETURN_NONE;
}

PyObject * chrBlendLoopMotion(PyObject* poSelf, PyObject* poArgs)
{
	int iMotionIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iMotionIndex))
		return Py_BuildException();
	float fBlendTime;
	if (!PyTuple_GetFloat(poArgs, 1, &fBlendTime))
		return Py_BuildException();

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();
	pkInst->SetLoopMotion(WORD(iMotionIndex), fBlendTime);

	return Py_BuildNone();
}

PyObject * chrPushOnceMotion(PyObject* poSelf, PyObject* poArgs)
{
	int iMotionIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iMotionIndex))
		return Py_BuildException();

	float fBlendTime;
	if (!PyTuple_GetFloat(poArgs, 1, &fBlendTime))
	{
		fBlendTime = 0.1f;
	}

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();
	pkInst->PushOnceMotion(WORD(iMotionIndex), fBlendTime);

	return Py_BuildNone();
}

PyObject * chrPushLoopMotion(PyObject* poSelf, PyObject* poArgs)
{
	int iMotionIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iMotionIndex))
		return Py_BuildException();

	float fBlendTime;
	if (!PyTuple_GetFloat(poArgs, 1, &fBlendTime))
	{
		fBlendTime = 0.1f;
	}

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();
	pkInst->PushLoopMotion(WORD(iMotionIndex), fBlendTime);

	return Py_BuildNone();
}

PyObject * chrSetPixelPosition(PyObject* poSelf, PyObject* poArgs)
{
	int iX;
	if (!PyTuple_GetInteger(poArgs, 0, &iX))
		return Py_BuildException();
	int iY;
	if (!PyTuple_GetInteger(poArgs, 1, &iY))
		return Py_BuildException();

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	int iZ;
	if (PyTuple_GetInteger(poArgs, 2, &iZ))
	{
		pkInst->NEW_SetPixelPosition(TPixelPosition(iX, iY, iZ));
	}
	else
	{
		pkInst->SCRIPT_SetPixelPosition(iX, iY);
	}
	return Py_BuildNone();
}

PyObject * chrSetDirection(PyObject* poSelf, PyObject* poArgs)
{
	int iDirection;
	if (!PyTuple_GetInteger(poArgs, 0, &iDirection))
		return Py_BuildException();

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	pkInst->SetDirection(iDirection);
	return Py_BuildNone();
}

PyObject * chrSetRotation(PyObject* poSelf, PyObject* poArgs)
{
	float fRotation;
	if (!PyTuple_GetFloat(poArgs, 0, &fRotation))
		return Py_BuildException();

	CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();

	if (!pCharacterInstance)
		return Py_BuildNone();

	fRotation = fmod(fRotation + 180.0f, 360.0f);

	pCharacterInstance->SetRotation(fRotation);

	return Py_BuildNone();
}

PyObject * chrSetRotationAll(PyObject* poSelf, PyObject* poArgs)
{
	float fRotX;
	if (!PyTuple_GetFloat(poArgs, 0, &fRotX))
		return Py_BuildException();
	float fRotY;
	if (!PyTuple_GetFloat(poArgs, 1, &fRotY))
		return Py_BuildException();
	float fRotZ;
	if (!PyTuple_GetFloat(poArgs, 2, &fRotZ))
		return Py_BuildException();

	CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pCharacterInstance)
		return Py_BuildNone();

	pCharacterInstance->GetGraphicThingInstanceRef().SetXYRotation(fRotX, fRotY);
	pCharacterInstance->GetGraphicThingInstanceRef().SetRotation(fRotZ);
	return Py_BuildNone();
}

PyObject * chrBlendRotation(PyObject* poSelf, PyObject* poArgs)
{
	float fRotation;
	if (!PyTuple_GetFloat(poArgs, 0, &fRotation))
		return Py_BuildException();
	float fBlendTime;
	if (!PyTuple_GetFloat(poArgs, 1, &fBlendTime))
		return Py_BuildException();

	fRotation = fmod(720.0f - fRotation, 360.0f);

	CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pCharacterInstance)
		return Py_BuildNone();
	pCharacterInstance->BlendRotation(fRotation, fBlendTime);

	return Py_BuildNone();
}

PyObject * chrGetRotation(PyObject* poSelf, PyObject* poArgs)
{
	CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();

	if (!pCharacterInstance)
		return Py_BuildValue("f", 0.0f);

	float fRotation = pCharacterInstance->GetRotation();

	return Py_BuildValue("f", 360.0f - fRotation);
}

PyObject * chrtestSetComboType(PyObject* poSelf, PyObject* poArgs)
{
	int iComboType;
	if (!PyTuple_GetInteger(poArgs, 0, &iComboType))
		return Py_BuildException();

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	pkInst->GetGraphicThingInstanceRef().SetComboType(iComboType);

	return Py_BuildNone();
}

PyObject * chrSetComboType(PyObject* poSelf, PyObject* poArgs)
{
	return chrtestSetComboType(poSelf, poArgs);
}

PyObject * chrFaintTest(PyObject* poSelf, PyObject* poArgs)
{
	CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (pCharacterInstance)
	{
		if (pCharacterInstance->GetGraphicThingInstanceRef().IsFaint())
		{
			pCharacterInstance->GetGraphicThingInstanceRef().SetFaint(false);
		}
		else
		{
			pCharacterInstance->GetGraphicThingInstanceRef().InterceptOnceMotion(CRaceMotionData::NAME_DAMAGE_FLYING);
			pCharacterInstance->GetGraphicThingInstanceRef().PushOnceMotion(CRaceMotionData::NAME_STAND_UP);
			pCharacterInstance->GetGraphicThingInstanceRef().PushLoopMotion(CRaceMotionData::NAME_WAIT);
			pCharacterInstance->GetGraphicThingInstanceRef().SetFaint(true);
		}
	}

	return Py_BuildNone();
}

PyObject * chrSetMoveSpeed(PyObject* poSelf, PyObject* poArgs)
{
	//int iSpeed;
	//if (!PyTuple_GetInteger(poArgs, 0, &iSpeed))
	//	return Py_BadArgument();

	//CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();

	//if (!pCharacterInstance)
	//	return Py_BuildValue("i", 0);

	//pCharacterInstance->SetMoveSpeed(iSpeed);

	return Py_BuildNone();
}

PyObject * chrSetAttackSpeed(PyObject* poSelf, PyObject* poArgs)
{
	//int iSpeed;
	//if (!PyTuple_GetInteger(poArgs, 0, &iSpeed))
	//	return Py_BadArgument();

	//CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetSelectedInstancePtr();

	//if (!pCharacterInstance)
	//	return Py_BuildValue("i", 0);

	//pCharacterInstance->SetAttackSpeed(iSpeed);

	return Py_BuildNone();
}

PyObject * chrMoveToDestPosition(PyObject* poSelf, PyObject* poArgs)
{
	int iVID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVID))
		return Py_BadArgument();

	int ix;
	if (!PyTuple_GetInteger(poArgs, 1, &ix))
		return Py_BadArgument();
	int iy;
	if (!PyTuple_GetInteger(poArgs, 2, &iy))
		return Py_BadArgument();

	CInstanceBase * pCharacterInstance = CPythonCharacterManager::Instance().GetInstancePtr(iVID);
	if (!pCharacterInstance)
		return Py_BuildNone();

	pCharacterInstance->NEW_MoveToDestPixelPositionDirection(TPixelPosition(ix, iy, 0.0f));

	return Py_BuildNone();
}
