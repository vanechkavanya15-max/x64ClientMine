#include "StdAfx.h"
#include "PythonNetworkStream.h"
#include "Packet.h"

// Select Character ---------------------------------------------------------------------------
void CPythonNetworkStream::SetSelectPhase()
{
	if ("Select" == m_strPhase)
		return;

	m_phaseLeaveFunc.Run();

	m_strPhase = "Select";

	m_dwChangingPhaseTime = ELTimer_GetMSec();
	m_phaseProcessFunc.Set(this, &CPythonNetworkStream::SelectPhase);
	m_phaseLeaveFunc.Set(this, &CPythonNetworkStream::__LeaveSelectPhase);

	if (__DirectEnterMode_IsSet())
	{
		PyCallClassMemberFunc(m_poHandler, "SetLoadingPhase", Py_BuildValue("()"));
	}
	else
	{
		if (m_dwEmpireID == 0)
			m_dwEmpireID = 1;

		PyCallClassMemberFunc(m_poHandler, "SetSelectCharacterPhase", Py_BuildValue("()"));
	}
}

void CPythonNetworkStream::SelectPhase()
{
	DispatchPacket(m_selectHandlers);
}

bool CPythonNetworkStream::SendSelectEmpirePacket(DWORD dwEmpireID)
{
	Beavium::TPacketCGSelectEmpireBeavium pkt;
	pkt.header = Beavium::CG::SELECT_EMPIRE; // 0x35
	pkt.empireId = static_cast<uint8_t>(dwEmpireID);

	Tracef("[BEAVIUM] Sending SelectEmpire 0x35: empireId=%u\n", dwEmpireID);

	if (!Send(sizeof(pkt), &pkt))
	{
		Tracen("SendSelectEmpirePacket - Error");
		return false;
	}

	SetEmpireID(dwEmpireID);
	return true;
}

bool CPythonNetworkStream::SendSelectCharacterPacket(BYTE Index)
{
	if (Index >= PLAYER_PER_ACCOUNT4)
		return false;

	if (GetServerMode() != Network::ServerType::Beavium)
	{
		TPacketCGSelectCharacter SelectCharacterPacket;
		SelectCharacterPacket.header = CG::CHARACTER_SELECT;
		SelectCharacterPacket.length = sizeof(SelectCharacterPacket);
		SelectCharacterPacket.player_index = Index;

		Tracef("[OFFICIAL] Sending CharacterSelect (0x%04X, slot=%u)\n", SelectCharacterPacket.header, Index);

		if (!Send(sizeof(TPacketCGSelectCharacter), &SelectCharacterPacket))
		{
			Tracen("SendSelectCharacterPacket - Error");
			return false;
		}

		return true;
	}

	uint32_t charId = m_akSimplePlayerInfo[Index].dwID;
	uint16_t port = m_akSimplePlayerInfo[Index].wPort;
	if (port == 0)
		port = 20102;
	m_targetGamePort = port;
	uint64_t candidateToken = m_candidateTokens[Index];
	m_gameToken64 = (candidateToken != 0) ? candidateToken : m_loginKey64;
	m_dwSelectedCharacterIndex = Index;

	if ("Loading" == m_strPhase && !m_bGameServerCharListReceived)
	{
		Tracef("[BEAVIUM] SendSelectCharacterPacket during Loading before 0x1C: caching slot %u (charId=%u), deferring 0x06 until 0x1C received.\n",
			Index, charId);
		return true;
	}

	Beavium::TPacketCGCharSelectBeavium selPkt;
	memset(&selPkt, 0, sizeof(selPkt));
	selPkt.header = Beavium::CG::CHAR_SELECT; // 0x06
	selPkt.slot = Index;
	selPkt.charId = charId;
	selPkt.securityToken = 0;

	Tracef("[BEAVIUM] Sending CharSelect 0x06 (14B): slot=%u, charId=%u, targetPort=%u, gameToken=0x%016llX (phase=%s)\n",
		Index, charId, port, (unsigned long long)m_gameToken64, m_strPhase.c_str());

	if (!Send(sizeof(selPkt), &selPkt))
	{
		Tracen("SendSelectCharacterPacket - Error");
		return false;
	}

	__SendInternalBuffer();

	if (port > 0 && port != m_serverPort && "Select" == m_strPhase)
	{
		Tracef("[BEAVIUM] Transitioning to Game Server %s on port %u...\n", m_serverIP.c_str(), port);
		m_bGameServerCharListReceived = false;
		m_bEnterGameSent = false;
		Disconnect();
		Connect(m_serverIP.c_str(), port);
		SetLoadingPhase();
	}

	return true;
}

bool CPythonNetworkStream::__RecvCharacterListBeavium()
{
	uint8_t buffer[465];
	if (!Recv(465, buffer))
		return false;

	Tracef("[BEAVIUM] Received Character List (0x1C, 465B) in phase '%s'!\n", m_strPhase.c_str());

	for (int s = 0; s < PLAYER_PER_ACCOUNT4 && s < 5; ++s)
	{
		size_t s_off = 1 + s * 89;
		uint32_t cid = *reinterpret_cast<const uint32_t*>(buffer + s_off);
		if (cid != 0)
		{
			char name[33] = { 0 };
			memcpy(name, buffer + s_off + 4, 32);
			name[32] = '\0';
			for (int n = 31; n >= 0 && (name[n] == ' ' || name[n] == '\0'); --n)
				name[n] = '\0';

			uint8_t job = buffer[s_off + 37];
			uint8_t lvl = buffer[s_off + 38];
			uint8_t bChangeName = buffer[s_off + 39];
			uint32_t playMinutes = *reinterpret_cast<const uint32_t*>(buffer + s_off + 40);
			uint8_t st = buffer[s_off + 44];
			uint8_t ht = buffer[s_off + 45];
			uint8_t dx = buffer[s_off + 46];
			uint8_t iq = buffer[s_off + 47];
			uint16_t mainPart = *reinterpret_cast<const uint16_t*>(buffer + s_off + 48);
			uint16_t hairPart = *reinterpret_cast<const uint16_t*>(buffer + s_off + 50);
			int32_t posX = *reinterpret_cast<const int32_t*>(buffer + s_off + 73);
			int32_t posY = *reinterpret_cast<const int32_t*>(buffer + s_off + 77);
			uint32_t ipAddr = *reinterpret_cast<const uint32_t*>(buffer + s_off + 81);
			uint16_t port = *reinterpret_cast<const uint16_t*>(buffer + s_off + 85);
			if (port == 0)
				port = 20102;

			m_akSimplePlayerInfo[s].dwID = cid;
			strncpy(m_akSimplePlayerInfo[s].szName, name, sizeof(m_akSimplePlayerInfo[s].szName) - 1);
			m_akSimplePlayerInfo[s].byJob = job;
			m_akSimplePlayerInfo[s].byLevel = lvl;
			m_akSimplePlayerInfo[s].bChangeName = bChangeName;
			m_akSimplePlayerInfo[s].dwPlayMinutes = playMinutes;
			m_akSimplePlayerInfo[s].byST = st;
			m_akSimplePlayerInfo[s].byHT = ht;
			m_akSimplePlayerInfo[s].byDX = dx;
			m_akSimplePlayerInfo[s].byIQ = iq;
			m_akSimplePlayerInfo[s].wMainPart = mainPart;
			m_akSimplePlayerInfo[s].wHairPart = hairPart;
			m_akSimplePlayerInfo[s].lAddr = ipAddr;
			m_akSimplePlayerInfo[s].wPort = port;

			Tracef("[BEAVIUM] Slot %d: '%s' (ID: %u, Level: %u, Job: %u, PlayMin: %u, Stats: ST=%u HT=%u DX=%u IQ=%u, GamePort: %u)\n",
				s, name, cid, lvl, job, playMinutes, st, ht, dx, iq, port);
		}
		else
		{
			memset(&m_akSimplePlayerInfo[s], 0, sizeof(m_akSimplePlayerInfo[s]));
		}
	}

	for (int s = 0; s < 4; ++s)
	{
		size_t tok_off = 433 + s * 8;
		if (tok_off + 8 <= 465)
		{
			m_candidateTokens[s] = *reinterpret_cast<const uint64_t*>(buffer + tok_off);
		}
	}

	m_bSimplePlayerInfo = true;

	if ("Loading" == m_strPhase)
	{
		m_bGameServerCharListReceived = true;
		Tracef("[BEAVIUM] In Loading phase: 0x1C received from Game Server! Now sending CharSelect 0x06 for slot %u\n",
			m_dwSelectedCharacterIndex);
		SendSelectCharacterPacket(static_cast<BYTE>(m_dwSelectedCharacterIndex));
		return true;
	}

	if (__DirectEnterMode_IsSet())
	{
		SendSelectCharacterPacket(static_cast<BYTE>(m_kDirectEnterMode.m_dwChrSlotIndex));
	}
	else
	{
		if ("Select" != m_strPhase)
			SetSelectPhase();
		PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_SELECT], "Refresh", Py_BuildValue("()"));
	}

	return true;
}

bool CPythonNetworkStream::SendDestroyCharacterPacket(BYTE index, const char * szPrivateCode)
{
	m_lastDeleteSlot = index;

	Beavium::TPacketCGCharacterDestroyBeavium pkt;
	memset(&pkt, 0, sizeof(pkt));
	pkt.header = Beavium::CG::CHARACTER_DESTROY; // 0x05
	pkt.slot = index;
	strncpy(pkt.privateCode, szPrivateCode ? szPrivateCode : "1234567", sizeof(pkt.privateCode));

	Tracef("[BEAVIUM] Sending CharacterDestroy 0x05 (10B): slot=%u, code='%s'\n", index, pkt.privateCode);

	if (!Send(sizeof(pkt), &pkt))
	{
		Tracen("SendDestroyCharacterPacket");
		return false;
	}

	return true;
}

bool CPythonNetworkStream::SendCreateCharacterPacket(BYTE index, const char *name, BYTE job, BYTE shape, BYTE byCON, BYTE byINT, BYTE bySTR, BYTE byDEX)
{
	Beavium::TPacketCGCharacterCreateBeavium pkt;
	memset(&pkt, 0, sizeof(pkt));
	pkt.header = Beavium::CG::CHARACTER_CREATE; // 0x04
	pkt.slot = index;
	strncpy(pkt.name, name, sizeof(pkt.name) - 1);
	pkt.job = job;
	pkt.shape = shape;
	pkt.str = bySTR;
	pkt.dex = byDEX;
	pkt.intelligence = byINT;
	pkt.con = byCON;

	Tracef("[BEAVIUM] Sending CharacterCreate 0x04 (42B): slot=%u, name='%s', job=%u, shape=%u\n",
		index, name, job, shape);

	if (!Send(sizeof(pkt), &pkt))
	{
		Tracen("Failed to SendCreateCharacterPacket");
		return false;
	}

	return true;
}

bool CPythonNetworkStream::SendChangeNamePacket(BYTE index, const char *name)
{
	TPacketCGChangeName ChangeNamePacket;
	ChangeNamePacket.header = CG::CHANGE_NAME;
	ChangeNamePacket.length = sizeof(ChangeNamePacket);
	ChangeNamePacket.index = index;
	strncpy(ChangeNamePacket.name, name, CHARACTER_NAME_MAX_LEN);

	if (!Send(sizeof(TPacketCGChangeName), &ChangeNamePacket))
	{
		Tracen("Failed to SendChangeNamePacket");
		return false;
	}

	return true;
}

bool CPythonNetworkStream::__RecvPlayerCreateSuccessPacket()
{
	Beavium::TPacketGCCharCreateSuccessBeavium pkt;
	if (!Recv(sizeof(pkt), &pkt))
		return false;

	Tracef("[BEAVIUM] Character created successfully on slot %u!\n", pkt.slot);

	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_CREATE], "OnCreateSuccess", Py_BuildValue("()"));
	return true;
}

bool CPythonNetworkStream::__RecvPlayerCreateFailurePacket()
{
	Beavium::TPacketGCCharCreateFailureBeavium pkt;
	if (!Recv(sizeof(pkt), &pkt))
		return false;

	Tracef("[BEAVIUM] Character creation failed!\n");

	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_CREATE], "OnCreateFailure", Py_BuildValue("(i)", 1));
	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_SELECT], "OnCreateFailure", Py_BuildValue("(i)", 1));
	return true;
}

bool CPythonNetworkStream::__RecvPlayerDestroySuccessPacket()
{
	Beavium::TPacketGCCharDeleteSuccessBeavium pkt;
	if (!Recv(sizeof(pkt), &pkt))
		return false;

	uint8_t slot = m_lastDeleteSlot;
	Tracef("[BEAVIUM] Character delete success for slot %u!\n", slot);

	if (slot < PLAYER_PER_ACCOUNT4)
	{
		memset(&m_akSimplePlayerInfo[slot], 0, sizeof(m_akSimplePlayerInfo[slot]));
		m_adwGuildID[slot] = 0;
		m_astrGuildName[slot] = "";
	}

	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_SELECT], "OnDeleteSuccess", Py_BuildValue("(i)", slot));
	return true;
}

bool CPythonNetworkStream::__RecvPlayerDestroyFailurePacket()
{
	Beavium::TPacketGCCharDeleteWrongPinBeavium pkt;
	if (!Recv(sizeof(pkt), &pkt))
		return false;

	Tracef("[BEAVIUM] Character delete failure (wrong pin/social id)!\n");

	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_SELECT], "OnDeleteFailure", Py_BuildValue("()"));
	return true;
}

bool CPythonNetworkStream::__RecvChangeName()
{
	TPacketGCChangeName ChangeNamePacket;
	if (!Recv(sizeof(TPacketGCChangeName), &ChangeNamePacket))
		return false;

	for (int i = 0; i < PLAYER_PER_ACCOUNT4; ++i)
	{
		if (ChangeNamePacket.pid == m_akSimplePlayerInfo[i].dwID)
		{
			m_akSimplePlayerInfo[i].bChangeName = FALSE;
			strncpy(m_akSimplePlayerInfo[i].szName, ChangeNamePacket.name, CHARACTER_NAME_MAX_LEN);

			PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_SELECT], "OnChangeName", Py_BuildValue("(is)", i, ChangeNamePacket.name));
			return true;
		}
	}

	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_SELECT], "OnCreateFailure", Py_BuildValue("(i)", 100));
	return true;
}
