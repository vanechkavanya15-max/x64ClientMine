#pragma once

#include "StdAfx.h"

// ============================================================================
// Domain: Guild, Party & Messenger (src/UserInterface/PythonNetworkStreamModule_GuildParty.cpp)
// ============================================================================

// Party
PyObject* netSendPartyInvitePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendPartyInviteAnswerPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendPartyExitPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendPartyRemovePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendPartySetStatePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendPartyUseSkillPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendPartyParameterPacket(PyObject* poSelf, PyObject* poArgs);

// Messenger
PyObject* netSendMessengerAddByVIDPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendMessengerAddByNamePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendMessengerRemovePacket(PyObject* poSelf, PyObject* poArgs);

// Guild
PyObject* netSendAnswerMakeGuildPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildAddMemberPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildRemoveMemberPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildChangeGradeNamePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildChangeGradeAuthorityPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildOfferPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildPostCommentPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSnedGuildPostCommentPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildDeleteCommentPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSnedGuildDeleteCommentPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildRefreshCommentsPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildChangeMemberGradePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildUseSkillPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildChangeMemberGeneralPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildInviteAnswerPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildChargeGSPPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildDepositMoneyPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendGuildWithdrawMoneyPacket(PyObject* poSelf, PyObject* poArgs);

// ============================================================================
// Domain: Trade, Shop, Safebox & Mall
// ============================================================================
#include "PythonNetworkStreamModule_TradeShop.h"

