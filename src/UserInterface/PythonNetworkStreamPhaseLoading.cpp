#include "StdAfx.h"
#include "PythonNetworkStream.h"
#include "Packet.h"
#include "PythonApplication.h"
#include "NetworkActorManager.h"
#include "AbstractPlayer.h"
#include "PackLib/PackManager.h"

void CPythonNetworkStream::EnableChatInsultFilter(bool isEnable)
{
	m_isEnableChatInsultFilter=isEnable;
}

void CPythonNetworkStream::__FilterInsult(char* szLine, UINT uLineLen)
{
	m_kInsultChecker.FilterInsult(szLine, uLineLen);
}

bool CPythonNetworkStream::IsChatInsultIn(const char* c_szMsg)
{
	if (m_isEnableChatInsultFilter)
		return false;

	return IsInsultIn(c_szMsg);
}

bool CPythonNetworkStream::IsInsultIn(const char* c_szMsg)
{
	return m_kInsultChecker.IsInsultIn(c_szMsg, strlen(c_szMsg));
}

bool CPythonNetworkStream::LoadInsultList(const char* c_szInsultListFileName)
{
	TPackFile file;
	if (!CPackManager::Instance().GetFile(c_szInsultListFileName, file))
		return false;

	CMemoryTextFileLoader kMemTextFileLoader;
	kMemTextFileLoader.Bind(file.size(), file.data());

	m_kInsultChecker.Clear();
	for (DWORD dwLineIndex=0; dwLineIndex<kMemTextFileLoader.GetLineCount(); ++dwLineIndex)
	{
		const std::string& c_rstLine=kMemTextFileLoader.GetLineString(dwLineIndex);		
		m_kInsultChecker.AppendInsult(c_rstLine);
	}
	return true;
}

bool CPythonNetworkStream::LoadConvertTable(DWORD dwEmpireID, const char* c_szFileName)
{
	if (dwEmpireID<1 || dwEmpireID>=4)
		return false;

	TPackFile file;
	if (!CPackManager::Instance().GetFile(c_szFileName, file))
		return false;

	DWORD dwEngCount=26;
	DWORD dwHanCount=(0xc8-0xb0+1)*(0xfe-0xa1+1);
	DWORD dwHanSize=dwHanCount*2;
	DWORD dwFileSize=dwEngCount*2+dwHanSize;

	if (file.size()<dwFileSize)
		return false;

	char* pcData=(char*)file.data();

	STextConvertTable& rkTextConvTable=m_aTextConvTable[dwEmpireID-1];		
	memcpy(rkTextConvTable.acUpper, pcData, dwEngCount);pcData+=dwEngCount;
	memcpy(rkTextConvTable.acLower, pcData, dwEngCount);pcData+=dwEngCount;
	memcpy(rkTextConvTable.aacHan, pcData, dwHanSize);

	return true;
}

// Loading ---------------------------------------------------------------------------
void CPythonNetworkStream::LoadingPhase()
{
	while (DispatchPacket(m_loadingHandlers))
		;
}

void CPythonNetworkStream::SetLoadingPhase()
{
	if ("Loading" == m_strPhase)
		return;

	m_phaseLeaveFunc.Run();

	Tracef("[PHASE] Entering phase: Loading\n");
	Tracen("");
	Tracen("## Network - Loading Phase ##");
	Tracen("");

	m_strPhase = "Loading";

	m_dwChangingPhaseTime = ELTimer_GetMSec();
	m_phaseProcessFunc.Set(this, &CPythonNetworkStream::LoadingPhase);
	m_phaseLeaveFunc.Set(this, &CPythonNetworkStream::__LeaveLoadingPhase);

	CPythonPlayer& rkPlayer=CPythonPlayer::Instance();
	rkPlayer.Clear();

	CFlyingManager::Instance().DeleteAllInstances();
	CEffectManager::Instance().DeleteAllInstances();

	PyCallClassMemberFunc(m_poHandler, "SetLoadingPhase", Py_BuildValue("()"));
}

bool CPythonNetworkStream::__RecvMainCharacterBeavium()
{
	uint8_t buffer[341];
	if (!Recv(341, buffer))
		return false;

	uint32_t realVid = *reinterpret_cast<const uint32_t*>(buffer + 1);
	uint16_t raceNum = *reinterpret_cast<const uint16_t*>(buffer + 5);
	char cName[25] = { 0 };
	memcpy(cName, buffer + 7, 24);
	cName[24] = '\0';
	uint8_t empire = buffer[31];
	uint8_t skillGroup = buffer[32];
	int32_t posX = *reinterpret_cast<const int32_t*>(buffer + 69);
	int32_t posY = *reinterpret_cast<const int32_t*>(buffer + 73);
	int32_t posZ = *reinterpret_cast<const int32_t*>(buffer + 77);

	uint8_t playerLvl = m_akSimplePlayerInfo[m_dwSelectedCharacterIndex].byLevel;
	if (playerLvl == 0) playerLvl = 1;

	Tracef("[BEAVIUM] Main Character (0x66, 341B): VID=0x%08X (%u), Name='%s', Pos=(%d, %d, %d), Level=%u, Race=%u, Empire=%u\n",
		realVid, realVid, cName, posX, posY, posZ, playerLvl, raceNum, empire);

	if (empire == 0)
		empire = (m_dwEmpireID != 0) ? static_cast<uint8_t>(m_dwEmpireID) : 1;

	m_dwMainActorVID = realVid;
	m_dwMainActorRace = raceNum;
	m_dwMainActorEmpire = empire;
	m_dwMainActorSkillGroup = skillGroup;

	m_rokNetActorMgr->SetMainActorVID(m_dwMainActorVID);

	CPythonPlayer& rkPlayer = CPythonPlayer::Instance();
	rkPlayer.SetName(cName);
	rkPlayer.SetMainCharacterIndex(GetMainActorVID());
	rkPlayer.SetStatus(POINT_LEVEL, playerLvl);

	if (m_apoPhaseWnd[PHASE_WINDOW_LOAD])
	{
		PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_LOAD], "LoadData", Py_BuildValue("(ii)", posX, posY));
	}
	else
	{
		Tracef("[BEAVIUM] Warning: PHASE_WINDOW_LOAD is NULL in MainCharacter! Calling handler SetLoadingPhase directly.\n");
		PyCallClassMemberFunc(m_poHandler, "SetLoadingPhase", Py_BuildValue("()"));
		if (m_apoPhaseWnd[PHASE_WINDOW_LOAD])
			PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_LOAD], "LoadData", Py_BuildValue("(ii)", posX, posY));
	}

	// Send HEADER_CG_ENTERGAME (0x0A, 1 byte) to Game Server
	if (!m_bEnterGameSent)
	{
		Beavium::TPacketCGEnterGameBeavium egPkt;
		egPkt.header = Beavium::CG::ENTERGAME; // 0x0A
		Tracef("[BEAVIUM] Sending EnterGame (0x0A, 1B) to Game Server!\n");
		Send(sizeof(egPkt), &egPkt);
		__SendInternalBuffer();
		m_bEnterGameSent = true;
	}

	return true;
}

bool CPythonNetworkStream::RecvMainCharacter()
{
	return __RecvMainCharacterBeavium();
}


static std::string gs_fieldMusic_fileName;
static float gs_fieldMusic_volume = 1.0f / 5.0f * 0.1f;

void CPythonNetworkStream::__SetFieldMusicFileName(const char* musicName)
{
	gs_fieldMusic_fileName = musicName;
}

void CPythonNetworkStream::__SetFieldMusicFileInfo(const char* musicName, float vol)
{
	gs_fieldMusic_fileName = musicName;
	gs_fieldMusic_volume = vol;
}

const char* CPythonNetworkStream::GetFieldMusicFileName()
{
	return gs_fieldMusic_fileName.c_str();	
}

float CPythonNetworkStream::GetFieldMusicVolume()
{
	return gs_fieldMusic_volume;
}
// END_OF_SUPPORT_BGM


bool CPythonNetworkStream::__RecvPlayerPoints()
{
	uint8_t buffer[70];
	if (!Recv(70, buffer))
		return false;

	PyCallClassMemberFunc(m_apoPhaseWnd[PHASE_WINDOW_GAME], "RefreshStatus", Py_BuildValue("()"));
	return true;
}

void CPythonNetworkStream::StartGame()
{
	m_isStartGame=TRUE;
}

bool CPythonNetworkStream::SendEnterGame()
{
	if (m_bEnterGameSent)
	{
		Tracef("[BEAVIUM] SendEnterGame: 0x0A already sent, skipping duplicate.\n");
		return true;
	}

	Beavium::TPacketCGEnterGameBeavium egPkt;
	egPkt.header = Beavium::CG::ENTERGAME; // 0x0A
	Tracef("[BEAVIUM] SendEnterGame: sending 0x0A (1B) to Game Server\n");
	if (!Send(sizeof(egPkt), &egPkt))
		return false;

	__SendInternalBuffer();
	m_bEnterGameSent = true;
	return true;
}
