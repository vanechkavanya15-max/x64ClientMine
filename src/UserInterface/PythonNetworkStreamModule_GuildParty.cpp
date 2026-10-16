#include "StdAfx.h"
#include "PythonNetworkStream.h"
#include "PythonNetworkStreamModule.h"
#include "PythonGuild.h"
#include "AbstractPlayer.h"

// ============================================================================
// Messenger Functions
// ============================================================================

PyObject* netSendMessengerAddByVIDPacket(PyObject* poSelf, PyObject* poArgs)
{
	int vid;
	if (!PyTuple_GetInteger(poArgs, 0, &vid))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendMessengerAddByVIDPacket(vid);
	
	return Py_BuildNone();
}

PyObject* netSendMessengerAddByNamePacket(PyObject* poSelf, PyObject* poArgs)
{
	char * szName;
	if (!PyTuple_GetString(poArgs, 0, &szName))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendMessengerAddByNamePacket(szName);
	
	return Py_BuildNone();
}

PyObject* netSendMessengerRemovePacket(PyObject* poSelf, PyObject* poArgs)
{
	char * szKey;
	if (!PyTuple_GetString(poArgs, 0, &szKey))
		return Py_BuildException();
	char * szName;
	if (!PyTuple_GetString(poArgs, 1, &szName))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendMessengerRemovePacket(szKey, szName);
	
	return Py_BuildNone();
}

// ============================================================================
// Party Functions
// ============================================================================

PyObject* netSendPartyInvitePacket(PyObject* poSelf, PyObject* poArgs)
{
	int vid;
	if (!PyTuple_GetInteger(poArgs, 0, &vid))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendPartyInvitePacket(vid);

	return Py_BuildNone();
}

PyObject* netSendPartyInviteAnswerPacket(PyObject* poSelf, PyObject* poArgs)
{
	int vid;
	if (!PyTuple_GetInteger(poArgs, 0, &vid))
		return Py_BuildException();
	int answer;
	if (!PyTuple_GetInteger(poArgs, 1, &answer))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendPartyInviteAnswerPacket(vid, answer);

	return Py_BuildNone();
}

PyObject* netSendPartyExitPacket(PyObject* poSelf, PyObject* poArgs)
{
	IAbstractPlayer& rPlayer=IAbstractPlayer::GetSingleton();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();

	DWORD dwVID = rPlayer.GetMainCharacterIndex();
	DWORD dwPID;
	if (rPlayer.PartyMemberVIDToPID(dwVID, &dwPID))
		rns.SendPartyRemovePacket(dwPID);

	return Py_BuildNone();
}

PyObject* netSendPartyRemovePacket(PyObject* poSelf, PyObject* poArgs)
{
	int vid;
	if (!PyTuple_GetInteger(poArgs, 0, &vid))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendPartyRemovePacket(vid);

	return Py_BuildNone();
}

PyObject* netSendPartySetStatePacket(PyObject* poSelf, PyObject* poArgs)
{
	int iVID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVID))
		return Py_BuildException();
	int iState;
	if (!PyTuple_GetInteger(poArgs, 1, &iState))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 2, &iFlag))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendPartySetStatePacket(iVID, iState, iFlag);

	return Py_BuildNone();
}

PyObject* netSendPartyUseSkillPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iSkillIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iSkillIndex))
		return Py_BuildException();
	int iVID;
	if (!PyTuple_GetInteger(poArgs, 1, &iVID))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendPartyUseSkillPacket(iSkillIndex, iVID);

	return Py_BuildNone();
}

PyObject* netSendPartyParameterPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iMode;
	if (!PyTuple_GetInteger(poArgs, 0, &iMode))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendPartyParameterPacket(iMode);

	return Py_BuildNone();
}

// ============================================================================
// Guild Functions
// ============================================================================

PyObject* netSendAnswerMakeGuildPacket(PyObject* poSelf, PyObject* poArgs)
{
	char * szName;
	if (!PyTuple_GetString(poArgs, 0, &szName))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendAnswerMakeGuildPacket(szName);

	return Py_BuildNone();
}

PyObject* netSendGuildAddMemberPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iVID;
	if (!PyTuple_GetInteger(poArgs, 0, &iVID))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildAddMemberPacket(iVID);

	return Py_BuildNone();
}

PyObject* netSendGuildRemoveMemberPacket(PyObject* poSelf, PyObject* poArgs)
{
	char * szKey;
	if (!PyTuple_GetString(poArgs, 0, &szKey))
		return Py_BuildException();

	CPythonGuild::TGuildMemberData * pGuildMemberData;
	if (!CPythonGuild::Instance().GetMemberDataPtrByName(szKey, &pGuildMemberData))
	{
		TraceError("netSendGuildRemoveMemberPacket(szKey=%s) - Can't Find Guild Member\n", szKey);
		return Py_BuildNone();
	}

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildRemoveMemberPacket(pGuildMemberData->dwPID);

	return Py_BuildNone();
}

PyObject* netSendGuildChangeGradeNamePacket(PyObject* poSelf, PyObject* poArgs)
{
	int iGradeNumber;
	if (!PyTuple_GetInteger(poArgs, 0, &iGradeNumber))
		return Py_BuildException();
	char * szGradeName;
	if (!PyTuple_GetString(poArgs, 1, &szGradeName))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildChangeGradeNamePacket(iGradeNumber, szGradeName);

	return Py_BuildNone();
}

PyObject* netSendGuildChangeGradeAuthorityPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iGradeNumber;
	if (!PyTuple_GetInteger(poArgs, 0, &iGradeNumber))
		return Py_BuildException();
	int iAuthority;
	if (!PyTuple_GetInteger(poArgs, 1, &iAuthority))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildChangeGradeAuthorityPacket(iGradeNumber, iAuthority);

	return Py_BuildNone();
}

PyObject* netSendGuildOfferPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iExperience;
	if (!PyTuple_GetInteger(poArgs, 0, &iExperience))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildOfferPacket(iExperience);

	return Py_BuildNone();
}

PyObject* netSendGuildPostCommentPacket(PyObject* poSelf, PyObject* poArgs)
{
	char * szComment;
	if (!PyTuple_GetString(poArgs, 0, &szComment))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildPostCommentPacket(szComment);

	return Py_BuildNone();
}

PyObject* netSnedGuildPostCommentPacket(PyObject* poSelf, PyObject* poArgs)
{
	return netSendGuildPostCommentPacket(poSelf, poArgs);
}

PyObject* netSendGuildDeleteCommentPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iIndex))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildDeleteCommentPacket(iIndex);

	return Py_BuildNone();
}

PyObject* netSnedGuildDeleteCommentPacket(PyObject* poSelf, PyObject* poArgs)
{
	return netSendGuildDeleteCommentPacket(poSelf, poArgs);
}

PyObject* netSendGuildRefreshCommentsPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iHightestIndex;
	if (!PyTuple_GetInteger(poArgs, 0, &iHightestIndex))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildRefreshCommentsPacket(iHightestIndex);

	return Py_BuildNone();
}

PyObject* netSendGuildChangeMemberGradePacket(PyObject* poSelf, PyObject* poArgs)
{
	int iPID;
	if (!PyTuple_GetInteger(poArgs, 0, &iPID))
		return Py_BuildException();
	int iGradeNumber;
	if (!PyTuple_GetInteger(poArgs, 1, &iGradeNumber))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildChangeMemberGradePacket(iPID, iGradeNumber);

	return Py_BuildNone();
}

PyObject* netSendGuildUseSkillPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iSkillID;
	if (!PyTuple_GetInteger(poArgs, 0, &iSkillID))
		return Py_BuildException();
	int iTargetVID;
	if (!PyTuple_GetInteger(poArgs, 1, &iTargetVID))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildUseSkillPacket(iSkillID, iTargetVID);

	return Py_BuildNone();
}

PyObject* netSendGuildChangeMemberGeneralPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iPID;
	if (!PyTuple_GetInteger(poArgs, 0, &iPID))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildChangeMemberGeneralPacket(iPID, iFlag);

	return Py_BuildNone();
}

PyObject* netSendGuildInviteAnswerPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iGuildID;
	if (!PyTuple_GetInteger(poArgs, 0, &iGuildID))
		return Py_BuildException();
	int iAnswer;
	if (!PyTuple_GetInteger(poArgs, 1, &iAnswer))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildInviteAnswerPacket(iGuildID, iAnswer);

	return Py_BuildNone();
}

PyObject* netSendGuildChargeGSPPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iGSP;
	if (!PyTuple_GetInteger(poArgs, 0, &iGSP))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildChargeGSPPacket(iGSP);

	return Py_BuildNone();
}

PyObject* netSendGuildDepositMoneyPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iGSP;
	if (!PyTuple_GetInteger(poArgs, 0, &iGSP))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildDepositMoneyPacket(iGSP);

	return Py_BuildNone();
}

PyObject* netSendGuildWithdrawMoneyPacket(PyObject* poSelf, PyObject* poArgs)
{
	int iGSP;
	if (!PyTuple_GetInteger(poArgs, 0, &iGSP))
		return Py_BuildException();

	CPythonNetworkStream& rns=CPythonNetworkStream::Instance();
	rns.SendGuildWithdrawMoneyPacket(iGSP);

	return Py_BuildNone();
}
