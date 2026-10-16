#include "StdAfx.h"
#include "PythonCharacterManager.h"
#include "PythonNonPlayer.h"
#include "PythonCharacterModule.h"

// ============================================================================
// Podsystem Instancji i Wygladu Postaci (PythonCharacterModule_Visual.cpp)
// ============================================================================

PyObject * chrCreateInstance(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	if (PyTuple_Size(poArgs) == 2)
	{
		PyObject* poDict = PyTuple_GetItem(poArgs, 1);
		if (!PyDict_Check(poDict))
			return Py_BuildException();

		CInstanceBase::SCreateData kCreateData;
		kCreateData.m_bType = CActorInstance::TYPE_PC;
		kCreateData.m_dwLevel = 0;
		kCreateData.m_dwGuildID = 0;
		kCreateData.m_dwEmpireID = 0;
		kCreateData.m_dwVID = iVirtualID;
		kCreateData.m_dwMountVnum = 0;
		kCreateData.m_dwRace = 0;
		kCreateData.m_fRot = CInstanceBase::DIR_NORTH;
		kCreateData.m_lPosX = 0;
		kCreateData.m_lPosY = 0;
		kCreateData.m_stName = "NONAME";
		kCreateData.m_dwStateFlags = 0;
		kCreateData.m_dwMovSpd = 100;
		kCreateData.m_dwAtkSpd = 100;
		kCreateData.m_sAlignment = 0;
		kCreateData.m_byPKMode = 0;
		kCreateData.m_kAffectFlags.Clear();
		kCreateData.m_dwArmor = 8;
		kCreateData.m_dwWeapon = 0;
		kCreateData.m_dwHair = 0;
		kCreateData.m_isMain = false;

		PyObject* poHorse = PyDict_GetItemString(poDict, "horse");
		if (poHorse)
			kCreateData.m_dwMountVnum = PyLong_AsLong(poHorse);

		PyObject* poX = PyDict_GetItemString(poDict, "x");
		if (poX)
			kCreateData.m_lPosX = PyLong_AsLong(poX);

		PyObject* poY = PyDict_GetItemString(poDict, "y");
		if (poY)
			kCreateData.m_lPosY = PyLong_AsLong(poY);

		CPythonCharacterManager::Instance().CreateInstance(kCreateData);
	}
	else
	{
		CPythonCharacterManager::Instance().RegisterInstance(iVirtualID);
	}

	return Py_BuildNone();
}

PyObject * chrDeleteInstance(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	CPythonCharacterManager::Instance().DeleteInstance(iVirtualID);
	return Py_BuildNone();
}

PyObject * chrDeleteInstanceByFade(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	CPythonCharacterManager::Instance().DeleteInstanceByFade(iVirtualID);
	return Py_BuildNone();
}

PyObject * chrSelectInstance(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	CPythonCharacterManager::Instance().SelectInstance(iVirtualID);
	return Py_BuildNone();
}

PyObject * chrHasInstance(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	bool bFlag = CPythonCharacterManager::Instance().GetInstancePtr(iVirtualID) ? TRUE : FALSE;
	return Py_BuildValue("i", bFlag);
}

PyObject * chrIsEnemy(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	CInstanceBase * pInstance = CPythonCharacterManager::Instance().GetInstancePtr(iVirtualID);
	if (!pInstance)
		return Py_BuildValue("i", 0);

	return Py_BuildValue("i", pInstance->IsEnemy());
}

PyObject * chrIsNPC(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	CInstanceBase * pInstance = CPythonCharacterManager::Instance().GetInstancePtr(iVirtualID);
	if (!pInstance)
		return Py_BuildValue("i", 0);

	return Py_BuildValue("i", pInstance->IsNPC());
}

PyObject * chrIsGameMaster(PyObject* poSelf, PyObject* poArgs)
{
	int iVirtualID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVirtualID))
		return Py_BuildException();

	CInstanceBase * pInstance = CPythonCharacterManager::Instance().GetInstancePtr(iVirtualID);
	if (!pInstance)
		return Py_BuildValue("i", 0);

	return Py_BuildValue("i", pInstance->IsGameMaster());
}

PyObject * chrSetArmor(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (nargs != 1)
		return Py_BadArgument();

	if (!PyLong_Check(poArgs[0]))
		return Py_BadArgument();

	int iForm = PyLong_AsLong(poArgs[0]);

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		Py_RETURN_NONE;

	pkInst->SetArmor(iForm);
	pkInst->RegisterBoundingSphere();

	Py_RETURN_NONE;
}

PyObject * chrSetWeapon(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (nargs != 1)
		return Py_BadArgument();

	if (!PyLong_Check(poArgs[0]))
		return Py_BadArgument();

	int iForm = PyLong_AsLong(poArgs[0]);

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		Py_RETURN_NONE;

	pkInst->SetWeapon(iForm);

	Py_RETURN_NONE;
}

PyObject * chrSetHair(PyObject* poSelf, PyObject* poArgs)
{
	int iRace;
	if (!PyTuple_GetInteger(poArgs, 0, &iRace))
		return Py_BuildException();

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	pkInst->SetHair(iRace);

	return Py_BuildNone();
}

PyObject * chrChangeHair(PyObject* poSelf, PyObject* poArgs)
{
	int iHair;

	if (!PyTuple_GetInteger(poArgs, 0, &iHair))
		return Py_BuildException();
	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();
	pkInst->ChangeHair(iHair);
	return Py_BuildNone();
}

PyObject * chrSetAcce(PyObject* poSelf, PyObject* poArgs)
{
	int iAcce = 0;
	if (PyTuple_Check(poArgs))
	{
		if (!PyTuple_GetInteger(poArgs, 0, &iAcce))
			return Py_BuildException();
	}
	else if (PyLong_Check(poArgs))
	{
		iAcce = PyLong_AsLong(poArgs);
	}
	else
	{
		return Py_BuildException();
	}

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	pkInst->SetSash(static_cast<DWORD>(iAcce));
	return Py_BuildNone();
}

PyObject * chrChangeShape(PyObject* poSelf, PyObject* poArgs)
{
	int iForm;
	if (!PyTuple_GetInteger(poArgs, 0, &iForm))
		return Py_BuildException();

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	pkInst->ChangeArmor(iForm);

	return Py_BuildNone();
}

PyObject * chrShowEffect(PyObject* poSelf, PyObject* poArgs)
{
	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	if (PyTuple_Size(poArgs) < 1)
		return Py_BuildException();

	PyObject * poItem = PyTuple_GetItem(poArgs, 0);
	if (PyLong_Check(poItem))
	{
		int iEffectID = PyLong_AsLong(poItem);
		pkInst->AttachSpecialEffect(static_cast<DWORD>(iEffectID));
	}
	else if (PyUnicode_Check(poItem) || PyBytes_Check(poItem))
	{
		char * szEffectName = NULL;
		if (PyTuple_GetString(poArgs, 0, &szEffectName) && szEffectName)
		{
			// String effect name attachment not supported on instance directly
		}
	}

	return Py_BuildNone();
}

PyObject * chrClearEffect(PyObject* poSelf, PyObject* poArgs)
{
	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();
	if (!pkInst)
		return Py_BuildNone();

	if (PyTuple_Size(poArgs) >= 1)
	{
		int iEffectID = 0;
		if (PyTuple_GetInteger(poArgs, 0, &iEffectID))
		{
			return Py_BuildNone();
		}
	}

	pkInst->__ClearAffects();
	return Py_BuildNone();
}

PyObject * chrAttachEffectByName(PyObject* poSelf, PyObject* poArgs)
{
	int iParentPartIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iParentPartIndex))
		return Py_BuildException();

	char * pszBoneName;
	if (!PyTuple_GetString(poArgs, 1, &pszBoneName))
	{
		pszBoneName = NULL;
	}

	char * pszEffectName;
	if (!PyTuple_GetString(poArgs, 2, &pszEffectName))
		return Py_BuildException();

	int iLife = CActorInstance::EFFECT_LIFE_INFINITE;

	if (PyTuple_Size(poArgs) == 4)
	{
		if (!PyTuple_GetInteger(poArgs, 3, &iLife))
			return Py_BuildException();
	}

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();

	if (!pkInst)
		return Py_BuildNone();

	//pkInst->AttachEffectByName(iParentPartIndex, pszBoneName, pszEffectName, iLife);
	return Py_BuildNone();
}

PyObject * chrAttachEffectByID(PyObject* poSelf, PyObject* poArgs)
{
	int iParentPartIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iParentPartIndex))
		return Py_BuildException();

	char * pszBoneName;
	if (!PyTuple_GetString(poArgs, 1, &pszBoneName))
	{
		pszBoneName = NULL;
	}

	int iEffectID;
	if (!PyTuple_GetInteger(poArgs, 2, &iEffectID))
		return Py_BuildException();

	int iLife = CActorInstance::EFFECT_LIFE_INFINITE;

	if (PyTuple_Size(poArgs) == 4)
	{
		if (!PyTuple_GetInteger(poArgs, 3, &iLife))
			return Py_BuildException();
	}

	CInstanceBase * pkInst = CPythonCharacterManager::Instance().GetSelectedInstancePtr();

	if (!pkInst)
		return Py_BuildNone();

	//pkInst->AttachEffectByID(iParentPartIndex, pszBoneName, iEffectID, iLife);
	return Py_BuildNone();
}
