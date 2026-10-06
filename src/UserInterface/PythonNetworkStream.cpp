#include "StdAfx.h"

#include "PythonNetworkStream.h"
#include "Packet.h"
#include "NetworkActorManager.h"

#include "GuildMarkDownloader.h"
#include "GuildMarkUploader.h"
#include "MarkManager.h"

#include "ProcessCRC.h"

// MARK_BUG_FIX
static DWORD gs_nextDownloadMarkTime = 0;
// END_OF_MARK_BUG_FIX

void CPythonNetworkStream::ExitApplication()
{
	if (__IsNotPing())
	{
		AbsoluteExitApplication();
	}
	else
	{
		SendChatPacket("/quit");
	}
}

void CPythonNetworkStream::ExitGame()
{
	if (__IsNotPing())
	{
		LogOutGame();
	}
	else
	{
		SendChatPacket("/phase_select");
	}
}


void CPythonNetworkStream::LogOutGame()
{
	if (__IsNotPing())
	{		
		AbsoluteExitGame();
	}	
	else
	{
		SendChatPacket("/logout");
	}
}

void CPythonNetworkStream::AbsoluteExitGame()
{
	if (!IsOnline())
		return;

	OnRemoteDisconnect();		
	Disconnect();
}

void CPythonNetworkStream::AbsoluteExitApplication()
{
	TraceError("CPythonNetworkStream::AbsoluteExitApplication called!");
	PostQuitMessage(0);
}

bool CPythonNetworkStream::__IsNotPing()
{
	// 원래는 핑이 안올때 체크이나 서버랑 정확히 맞추어야 한다.
	return false;
}

DWORD CPythonNetworkStream::GetGuildID()
{
	return m_dwGuildID;
}

UINT CPythonNetworkStream::UploadMark(const char * c_szImageFileName)
{
	// MARK_BUG_FIX
	// 길드를 만든 직후는 길드 아이디가 0이다.
	if (0 == m_dwGuildID)
		return ERROR_MARK_UPLOAD_NEED_RECONNECT;

	gs_nextDownloadMarkTime = 0;
	// END_OF_MARK_BUG_FIX

	UINT uError=ERROR_UNKNOWN;
	CGuildMarkUploader& rkGuildMarkUploader=CGuildMarkUploader::Instance();
	if (!rkGuildMarkUploader.Connect(m_kMarkAuth.m_kNetAddr, m_kMarkAuth.m_dwHandle, m_kMarkAuth.m_dwRandomKey, m_dwGuildID, c_szImageFileName, &uError))
	{
		switch (uError)
		{
			case CGuildMarkUploader::ERROR_CONNECT:
				return ERROR_CONNECT_MARK_SERVER;
				break;
			case CGuildMarkUploader::ERROR_LOAD:
				return ERROR_LOAD_MARK;
				break;
			case CGuildMarkUploader::ERROR_WIDTH:
				return ERROR_MARK_WIDTH;
				break;
			case CGuildMarkUploader::ERROR_HEIGHT:
				return ERROR_MARK_HEIGHT;
				break;
			default:
				return ERROR_UNKNOWN;
		}
	}

	// MARK_BUG_FIX	
	__DownloadMark();
	// END_OF_MARK_BUG_FIX
	
	if (CGuildMarkManager::INVALID_MARK_ID == CGuildMarkManager::Instance().GetMarkID(m_dwGuildID))
		return ERROR_MARK_CHECK_NEED_RECONNECT;

	return ERROR_NONE;
}

UINT CPythonNetworkStream::UploadSymbol(const char* c_szImageFileName)
{
	UINT uError=ERROR_UNKNOWN;
	CGuildMarkUploader& rkGuildMarkUploader=CGuildMarkUploader::Instance();
	if (!rkGuildMarkUploader.ConnectToSendSymbol(m_kMarkAuth.m_kNetAddr, m_kMarkAuth.m_dwHandle, m_kMarkAuth.m_dwRandomKey, m_dwGuildID, c_szImageFileName, &uError))
	{
		switch (uError)
		{
			case CGuildMarkUploader::ERROR_CONNECT:
				return ERROR_CONNECT_MARK_SERVER;
				break;
			case CGuildMarkUploader::ERROR_LOAD:
				return ERROR_LOAD_MARK;
				break;
			case CGuildMarkUploader::ERROR_WIDTH:
				return ERROR_MARK_WIDTH;
				break;
			case CGuildMarkUploader::ERROR_HEIGHT:
				return ERROR_MARK_HEIGHT;
				break;
			default:
				return ERROR_UNKNOWN;
		}
	}

	return ERROR_NONE;
}

void CPythonNetworkStream::__DownloadMark()
{
	// 3분 안에는 다시 접속하지 않는다.
	DWORD curTime = ELTimer_GetMSec();

	if (curTime < gs_nextDownloadMarkTime)
		return;

	gs_nextDownloadMarkTime = curTime + 60000 * 3; // 3분

	CGuildMarkDownloader& rkGuildMarkDownloader = CGuildMarkDownloader::Instance();
	rkGuildMarkDownloader.Connect(m_kMarkAuth.m_kNetAddr, m_kMarkAuth.m_dwHandle, m_kMarkAuth.m_dwRandomKey);
}

void CPythonNetworkStream::__DownloadSymbol(const std::vector<DWORD> & c_rkVec_dwGuildID)
{
	CGuildMarkDownloader& rkGuildMarkDownloader=CGuildMarkDownloader::Instance();
	rkGuildMarkDownloader.ConnectToRecvSymbol(m_kMarkAuth.m_kNetAddr, m_kMarkAuth.m_dwHandle, m_kMarkAuth.m_dwRandomKey, c_rkVec_dwGuildID);
}

void CPythonNetworkStream::SetPhaseWindow(UINT ePhaseWnd, PyObject* poPhaseWnd)
{
	if (ePhaseWnd>=PHASE_WINDOW_NUM)
		return;

	m_apoPhaseWnd[ePhaseWnd]=poPhaseWnd;
}

void CPythonNetworkStream::ClearPhaseWindow(UINT ePhaseWnd, PyObject* poPhaseWnd)
{
	if (ePhaseWnd>=PHASE_WINDOW_NUM)
		return;

	if (poPhaseWnd != m_apoPhaseWnd[ePhaseWnd])
		return;

	m_apoPhaseWnd[ePhaseWnd]=0;
}

void CPythonNetworkStream::SetServerCommandParserWindow(PyObject* poWnd)
{
	m_poSerCommandParserWnd = poWnd;
}

bool CPythonNetworkStream::IsSelectedEmpire()
{
	if (m_dwEmpireID)
		return true;

	for (int i = 0; i < PLAYER_PER_ACCOUNT4; ++i)
	{
		if (m_akSimplePlayerInfo[i].dwID != 0)
		{
			if (m_dwEmpireID == 0)
				m_dwEmpireID = 1;
			return true;
		}
	}

	return false;
}

UINT CPythonNetworkStream::GetAccountCharacterSlotDatau(UINT iSlot, UINT eType)
{
	if (iSlot >= PLAYER_PER_ACCOUNT4)
		return 0;
		
	TSimplePlayerInformation&	rkSimplePlayerInfo=m_akSimplePlayerInfo[iSlot];
	
	switch (eType)
	{
		case ACCOUNT_CHARACTER_SLOT_ID:
			return rkSimplePlayerInfo.dwID;
		case ACCOUNT_CHARACTER_SLOT_RACE:
			return rkSimplePlayerInfo.byJob;
		case ACCOUNT_CHARACTER_SLOT_LEVEL:
			return rkSimplePlayerInfo.byLevel;
		case ACCOUNT_CHARACTER_SLOT_STR:
			return rkSimplePlayerInfo.byST;
		case ACCOUNT_CHARACTER_SLOT_DEX:
			return rkSimplePlayerInfo.byDX;
		case ACCOUNT_CHARACTER_SLOT_HTH:
			return rkSimplePlayerInfo.byHT;
		case ACCOUNT_CHARACTER_SLOT_INT:			
			return rkSimplePlayerInfo.byIQ;
		case ACCOUNT_CHARACTER_SLOT_PLAYTIME:
			return rkSimplePlayerInfo.dwPlayMinutes;
		case ACCOUNT_CHARACTER_SLOT_FORM:
//			return rkSimplePlayerInfo.wParts[CRaceData::PART_MAIN];
			return rkSimplePlayerInfo.wMainPart;
		case ACCOUNT_CHARACTER_SLOT_PORT:
			return rkSimplePlayerInfo.wPort;
		case ACCOUNT_CHARACTER_SLOT_GUILD_ID:
			return m_adwGuildID[iSlot];
			break;
		case ACCOUNT_CHARACTER_SLOT_CHANGE_NAME_FLAG:
			return rkSimplePlayerInfo.bChangeName;
			break;
		case ACCOUNT_CHARACTER_SLOT_HAIR:
			return rkSimplePlayerInfo.wHairPart;
			break;
	}
	return 0;
}

const char* CPythonNetworkStream::GetAccountCharacterSlotDataz(UINT iSlot, UINT eType)
{
	static const char* sc_szEmpty="";

	if (iSlot >= PLAYER_PER_ACCOUNT4)
		return sc_szEmpty;
		
	TSimplePlayerInformation&	rkSimplePlayerInfo=m_akSimplePlayerInfo[iSlot];
	
	switch (eType)
	{
		case ACCOUNT_CHARACTER_SLOT_ADDR:
			{				
				BYTE ip[4];

				const int LEN = 4;
				for (int i = 0; i < LEN; i++)
				{
					ip[i] = BYTE(rkSimplePlayerInfo.lAddr&0xff);
					rkSimplePlayerInfo.lAddr>>=8;
				}


				static char s_szAddr[256];
				sprintf(s_szAddr, "%d.%d.%d.%d", ip[0], ip[1], ip[2], ip[3]);
				return s_szAddr;
			}
			break;
		case ACCOUNT_CHARACTER_SLOT_NAME:
			return rkSimplePlayerInfo.szName;
			break;		
		case ACCOUNT_CHARACTER_SLOT_GUILD_NAME:
			return m_astrGuildName[iSlot].c_str();
			break;
	}
	return sc_szEmpty;
}

bool CPythonNetworkStream::Connect(const char* c_szAddr, int port, int limitSec)
{
	if (c_szAddr && strlen(c_szAddr) > 0)
		m_serverIP = c_szAddr;
	m_serverPort = port;

	bool isBeavium = (port == 11001 || (port >= 20101 && port <= 20501) ||
		(c_szAddr && strstr(c_szAddr, "51.83.150.")) ||
		Network::MultiServerManager::Instance().IsBeavium());

	SetBeaviumMode(isBeavium);
	return CNetworkStream::Connect(c_szAddr, port, limitSec);
}

void CPythonNetworkStream::ConnectLoginServer(const char* c_szAddr, UINT uPort)
{
	Connect(c_szAddr, uPort);		
}

void CPythonNetworkStream::SetMarkServer(const char* c_szAddr, UINT uPort)
{
	m_kMarkAuth.m_kNetAddr.Set(c_szAddr, uPort);
}

void CPythonNetworkStream::ConnectGameServer(UINT iChrSlot)
{
	if (iChrSlot >= PLAYER_PER_ACCOUNT4)
		return;

	m_dwSelectedCharacterIndex = iChrSlot;
	__DirectEnterMode_Set(iChrSlot);

	SendSelectCharacterPacket(static_cast<BYTE>(iChrSlot));
}

void CPythonNetworkStream::SetLoginInfo(const char* c_szID, const char* c_szPassword)
{
	m_stID=c_szID;
	m_stPassword=c_szPassword;
}

void CPythonNetworkStream::ClearLoginInfo( void )
{
	m_stPassword = "";
}

void CPythonNetworkStream::SetLoginKey(DWORD dwLoginKey)
{
	m_dwLoginKey = dwLoginKey;
}

// Table-driven packet dispatch — replaces CheckPacket() + per-phase switch statements.
// Returns true if a packet was processed and more may follow.
// Returns false to exit the phase loop (no data, error, or exitPhase handler).
bool CPythonNetworkStream::DispatchPacket(const PacketHandlerMap& handlers)
{
	uint8_t header = 0;
	if (!Peek(sizeof(uint8_t), &header))
		return false;

	// Skip zero-padding (can occur from encryption alignment)
	while (0 == header)
	{
		if (!Recv(sizeof(uint8_t), &header))
			return false;
		if (!Peek(sizeof(uint8_t), &header))
			return false;
	}

	Beavium::BeaviumGCInfo info;
	if (!Beavium::GetGCInfo(header, info))
	{
		TraceError("[BEAVIUM] Unknown packet header: 0x%02X (recv_seq #%u), Phase: %s", header, m_dwRecvPacketSeq, m_strPhase.c_str());
		DumpRecentPackets();
		ClearRecvBuffer();
		return false;
	}

	uint32_t packetLen = 0;
	if (info.isDynamic)
	{
		if (GetRecvBufferSize() < 3)
			return false; // Need at least header (1B) + size (2B)

		char tempBuf[3];
		Peek(3, tempBuf);
		uint16_t dynSize = *reinterpret_cast<const uint16_t*>(tempBuf + 1);

		constexpr uint16_t MAX_PACKET_LENGTH = 65000;
		if (header == Beavium::GC::SYNC_POSITION)
		{
			if (dynSize > MAX_PACKET_LENGTH)
			{
				TraceError("[BEAVIUM] Invalid dynamic packet length: header 0x%02X length: %u, recv_seq #%u",
					header, dynSize, m_dwRecvPacketSeq);
				DumpRecentPackets();
				ClearRecvBuffer();
				return false;
			}
			packetLen = dynSize + 3;
		}
		else
		{
			if (dynSize < 3 || dynSize > MAX_PACKET_LENGTH)
			{
				TraceError("[BEAVIUM] Invalid dynamic packet length: header 0x%02X length: %u, recv_seq #%u",
					header, dynSize, m_dwRecvPacketSeq);
				DumpRecentPackets();
				ClearRecvBuffer();
				return false;
			}
			packetLen = dynSize;
		}
	}
	else
	{
		packetLen = info.size;
	}

	// Wait for full packet to be received
	if (GetRecvBufferSize() < static_cast<int>(packetLen))
		return false;

	// Look up handler in this phase's table
	auto it = handlers.find(header);
	if (it == handlers.end())
	{
		// Unhandled packet in this phase: safely discard packetLen bytes to prevent stream desync!
		Tracef("[BEAVIUM] Phase %s unhandled header 0x%02X (%u bytes), skipping cleanly.\n",
			m_strPhase.c_str(), header, packetLen);
		std::vector<char> skipBuf(packetLen);
		Recv(packetLen, skipBuf.data());
		return true;
	}

	// Log this packet
	LogRecvPacket(header, packetLen);

	// Call handler
	bool ret = (this->*(it->second.handler))();

	if (!ret || it->second.exitPhase)
		return false;

	return true;
}

bool CPythonNetworkStream::RecvErrorPacket(int header)
{
	TraceError("Phase %s does not handle header 0x%04X (recv_seq #%u)", m_strPhase.c_str(), header, m_dwRecvPacketSeq - 1);
	DumpRecentPackets();

	ClearRecvBuffer();
	return true;
}

bool CPythonNetworkStream::RecvPhasePacket()
{
	uint8_t header = 0;
	uint8_t phase = 0;

	if (!Recv(sizeof(uint8_t), &header))
		return false;
	if (!Recv(sizeof(uint8_t), &phase))
		return false;

	Tracef("[BEAVIUM] RecvPhasePacket: header 0x%02X, phase %u (%s)\n", header, phase, m_strPhase.c_str());

	switch (phase)
	{
		case PHASE_CLOSE:
			ClosePhase();
			break;

		case PHASE_HANDSHAKE:
			SetHandShakePhase();
			break;

		case PHASE_LOGIN:
			if ("Login" != m_strPhase && "Loading" != m_strPhase)
				SetLoginPhase();
			break;

		case PHASE_SELECT:
			if ("Loading" != m_strPhase)
			{
				if ("Select" != m_strPhase)
					SetSelectPhase();
#if defined(ENABLE_DISCORD_RPC)
				Discord_Update(false);
#endif
				BuildProcessCRC();
				__DownloadMark();
			}
			break;

		case PHASE_LOADING:
			if ("Loading" != m_strPhase)
				SetLoadingPhase();
			break;

		case PHASE_GAME:
			if ("Game" != m_strPhase)
				SetGamePhase();
#if defined(ENABLE_DISCORD_RPC)
			Discord_Update(true);
#endif
			break;

		case PHASE_DEAD:
			break;
	}

	return true;
}

bool CPythonNetworkStream::RecvPingPacket()
{
	TPacketGCPing kPacketPing;

	if (!Recv(sizeof(TPacketGCPing), &kPacketPing))
		return false;

	m_dwLastGamePingTime = ELTimer_GetMSec();

	// Sync server time from ping
	ELTimer_SetServerMSec(kPacketPing.server_time);

	TPacketCGPong kPacketPong;
	kPacketPong.header = CG::PONG;
	kPacketPong.length = sizeof(kPacketPong);

	if (!Send(sizeof(TPacketCGPong), &kPacketPong))
		return false;

	return true;
}

bool CPythonNetworkStream::OnProcess()
{
	if (m_isStartGame)
	{
		m_isStartGame = FALSE;
		PyCallClassMemberFunc(m_poHandler, "SetGamePhase", Py_BuildValue("()"));
	}

	m_rokNetActorMgr->Update();

	if (m_phaseProcessFunc.IsEmpty())
		return true;

	m_phaseProcessFunc.Run();

	return true;
}


// Set
void CPythonNetworkStream::SetOffLinePhase()
{
	if ("OffLine" != m_strPhase)
		m_phaseLeaveFunc.Run();

	Tracef("[PHASE] Entering phase: OffLine\n");
	m_strPhase = "OffLine";

	Tracen("");
	Tracen("## Network - OffLine Phase ##");	
	Tracen("");
	
#if defined(ENABLE_DISCORD_RPC)
	Discord_Update(false);
#endif

	m_dwChangingPhaseTime = ELTimer_GetMSec();
	m_phaseProcessFunc.Set(this, &CPythonNetworkStream::OffLinePhase);
	m_phaseLeaveFunc.Set(this, &CPythonNetworkStream::__LeaveOfflinePhase);

	SetGameOffline();

	m_dwSelectedCharacterIndex = 0;
	m_bGameServerCharListReceived = false;
	m_bEnterGameSent = false;

	__DirectEnterMode_Initialize();
	__BettingGuildWar_Initialize();
}


void CPythonNetworkStream::ClosePhase()
{
	PyCallClassMemberFunc(m_poHandler, "SetLoginPhase", Py_BuildValue("()"));
}

// Game Online
void CPythonNetworkStream::SetGameOnline()
{
	m_isGameOnline = TRUE;
}

void CPythonNetworkStream::SetGameOffline()
{
	m_isGameOnline = FALSE;
}

BOOL CPythonNetworkStream::IsGameOnline()
{
	return m_isGameOnline;
}

// Handler
void CPythonNetworkStream::SetHandler(PyObject* poHandler)
{
	m_poHandler = poHandler;
}

// ETC
DWORD CPythonNetworkStream::GetMainActorVID()
{
	return m_dwMainActorVID;
}

DWORD CPythonNetworkStream::GetMainActorRace()
{
	return m_dwMainActorRace;
}

DWORD CPythonNetworkStream::GetMainActorEmpire()
{
	return m_dwMainActorEmpire;
}

DWORD CPythonNetworkStream::GetMainActorSkillGroup()
{
	return m_dwMainActorSkillGroup;
}

void CPythonNetworkStream::SetEmpireID(DWORD dwEmpireID)
{
	m_dwEmpireID = dwEmpireID;
}

DWORD CPythonNetworkStream::GetEmpireID()
{
	return m_dwEmpireID;
}

void CPythonNetworkStream::__ClearSelectCharacterData()
{
	NANOBEGIN
	memset(&m_akSimplePlayerInfo, 0, sizeof(m_akSimplePlayerInfo));

	for (int i = 0; i < PLAYER_PER_ACCOUNT4; ++i)
	{
		m_adwGuildID[i] = 0;
		m_astrGuildName[i] = "";
	}
	NANOEND
}

void CPythonNetworkStream::__DirectEnterMode_Initialize()
{
	m_kDirectEnterMode.m_isSet=false;
	m_kDirectEnterMode.m_dwChrSlotIndex=0;	
}

void CPythonNetworkStream::__DirectEnterMode_Set(UINT uChrSlotIndex)
{
	m_kDirectEnterMode.m_isSet=true;
	m_kDirectEnterMode.m_dwChrSlotIndex=uChrSlotIndex;
}

bool CPythonNetworkStream::__DirectEnterMode_IsSet()
{
	return m_kDirectEnterMode.m_isSet;
}

void CPythonNetworkStream::__InitializeMarkAuth()
{
	m_kMarkAuth.m_dwHandle=0;
	m_kMarkAuth.m_dwRandomKey=0;
}

void CPythonNetworkStream::__BettingGuildWar_Initialize()
{
	m_kBettingGuildWar.m_dwBettingMoney=0;
	m_kBettingGuildWar.m_dwObserverCount=0;
}

void CPythonNetworkStream::__BettingGuildWar_SetObserverCount(UINT uObserverCount)
{
	m_kBettingGuildWar.m_dwObserverCount=uObserverCount;
}

void CPythonNetworkStream::__BettingGuildWar_SetBettingMoney(UINT uBettingMoney)
{
	m_kBettingGuildWar.m_dwBettingMoney=uBettingMoney;
}

DWORD CPythonNetworkStream::EXPORT_GetBettingGuildWarValue(const char* c_szValueName)
{
	if (stricmp(c_szValueName, "OBSERVER_COUNT") == 0)
		return m_kBettingGuildWar.m_dwObserverCount;

	if (stricmp(c_szValueName, "BETTING_MONEY") == 0)
		return m_kBettingGuildWar.m_dwBettingMoney;

	return 0;
}

void CPythonNetworkStream::__ServerTimeSync_Initialize()
{
	m_kServerTimeSync.m_dwChangeClientTime=0;
	m_kServerTimeSync.m_dwChangeServerTime=0;
}

void CPythonNetworkStream::SetWaitFlag()
{
	m_isWaitLoginKey = TRUE;
}

void CPythonNetworkStream::SendEmoticon(UINT eEmoticon)
{
	if(eEmoticon < m_EmoticonStringVector.size())
		SendChatPacket(m_EmoticonStringVector[eEmoticon].c_str());
	else
		assert(false && "SendEmoticon Error");
}

CPythonNetworkStream::CPythonNetworkStream()
{
	m_rokNetActorMgr=new CNetworkActorManager;

	memset(m_akSimplePlayerInfo, 0, sizeof(m_akSimplePlayerInfo));

	// Increase network buffer sizes for better throughput
	SetRecvBufferSize(65536);  // 64KB recv buffer
	SetSendBufferSize(65536);  // 64KB send buffer

	m_phaseProcessFunc.Clear();

	m_dwEmpireID = 0;
	m_dwGuildID = 0;

	m_dwMainActorVID = 0;
	m_dwMainActorRace = 0;
	m_dwMainActorEmpire = 0;
	m_dwMainActorSkillGroup = 0;
	m_poHandler = NULL;

	m_dwLastGamePingTime = 0;

	m_dwLoginKey = 0;
	m_isWaitLoginKey = FALSE;
	m_isStartGame = FALSE;
	m_isEnableChatInsultFilter = FALSE;
	m_bComboSkillFlag = FALSE;
	m_strPhase = "OffLine";
	
	__InitializeGamePhase();
	__InitializeMarkAuth();

	__DirectEnterMode_Initialize();
	__BettingGuildWar_Initialize();

	std::fill(m_apoPhaseWnd, m_apoPhaseWnd+PHASE_WINDOW_NUM, (PyObject*)NULL);
	m_poSerCommandParserWnd = NULL;

	// Multi-Server Profile Setup: Register handlers based on active profile (Official vs Beavium)
	SetServerMode(Network::MultiServerManager::Instance().GetServerType());

	SetOffLinePhase();
}

CPythonNetworkStream::~CPythonNetworkStream()
{
	Tracen("PythonNetworkMainStream Clear");
}

// ---------------------------------------------------------------------------
// Phase 5: Packet handler registration tables
// Each phase registers {header → handler, minSize, exitPhase}
// ---------------------------------------------------------------------------

void CPythonNetworkStream::RegisterOfflineHandlers()
{
	auto& h = m_offlineHandlers;
	h[Beavium::GC::PHASE] = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
}

void CPythonNetworkStream::RegisterHandshakeHandlers()
{
	auto& h = m_handshakeHandlers;
	h[Beavium::GC::PHASE]     = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	h[Beavium::GC::TIME_SYNC] = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	h[0x2C]                   = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
}

void CPythonNetworkStream::RegisterLoginHandlers()
{
	auto& h = m_loginHandlers;
	h[Beavium::GC::PHASE]         = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	h[Beavium::GC::TIME_SYNC]     = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	h[Beavium::GC::CHAR_LIST]     = { &CPythonNetworkStream::__RecvCharacterListBeavium, 465, true };
	h[Beavium::GC::LOGIN_FAILURE] = { &CPythonNetworkStream::__RecvLoginFailurePacket, sizeof(TPacketGCLoginFailure), false };
	h[Beavium::GC::AUTH_KEY]      = { &CPythonNetworkStream::__RecvAuthSuccessBeavium, 10, false };
	h[0x42]                       = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };
	h[0x2C]                       = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
}

void CPythonNetworkStream::RegisterSelectHandlers()
{
	auto& h = m_selectHandlers;
	h[Beavium::GC::PHASE]                 = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	h[Beavium::GC::CHAR_LIST]             = { &CPythonNetworkStream::__RecvCharacterListBeavium, 465, false };
	h[Beavium::GC::TIME_SYNC]             = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	h[0x2C]                               = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
	h[Beavium::GC::CHAR_CREATE_SUCCESS]   = { &CPythonNetworkStream::__RecvPlayerCreateSuccessPacket, 2, false };
	h[Beavium::GC::CHAR_CREATE_FAILURE]   = { &CPythonNetworkStream::__RecvPlayerCreateFailurePacket, 1, false };
	h[Beavium::GC::CHAR_DELETE_SUCCESS]   = { &CPythonNetworkStream::__RecvPlayerDestroySuccessPacket, 1, false };
	h[Beavium::GC::CHAR_DELETE_WRONG_PIN] = { &CPythonNetworkStream::__RecvPlayerDestroyFailurePacket, 5, false };
	h[0x42]                               = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };
}

void CPythonNetworkStream::RegisterLoadingHandlers()
{
	// Inherit game handlers so world packets received during loading are processed
	m_loadingHandlers = m_gameHandlers;

	auto& h = m_loadingHandlers;
	h[Beavium::GC::PHASE]              = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	h[Beavium::GC::TIME_SYNC]          = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	h[Beavium::GC::CHAR_LIST]          = { &CPythonNetworkStream::__RecvCharacterListBeavium, 465, false };
	h[Beavium::GC::MAIN_CHARACTER_BGM] = { &CPythonNetworkStream::__RecvMainCharacterBeavium, 341, false };
	h[0x42]                            = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };
	h[0x2C]                            = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
}

void CPythonNetworkStream::RegisterGameHandlers()
{
	auto& h = m_gameHandlers;

	// Phase / Control / Ping
	h[Beavium::GC::PHASE]              = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	h[Beavium::GC::TIME_SYNC]          = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	h[0x2C]                            = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
	h[Beavium::GC::MAIN_CHARACTER_BGM] = { &CPythonNetworkStream::__RecvMainCharacterBeavium, 341, false };

	// Actors & World
	h[Beavium::GC::CHARACTER_ADD]      = { &CPythonNetworkStream::__RecvCharacterAddBeavium, 45, false };
	h[Beavium::GC::CHARACTER_DEL]      = { &CPythonNetworkStream::__RecvCharacterDelBeavium, 6, false };
	h[Beavium::GC::CHARACTER_MOVE]     = { &CPythonNetworkStream::__RecvCharacterMoveBeavium, 31, false };
	h[Beavium::GC::CHAT]               = { &CPythonNetworkStream::__RecvChatBeavium, 14, false };
	h[Beavium::GC::SYNC_POSITION]      = { &CPythonNetworkStream::__RecvSyncPositionBeavium, 3, false };
	h[Beavium::GC::ATTACK]             = { &CPythonNetworkStream::__RecvAttackBeavium, 12, false };
	h[Beavium::GC::DEAD]               = { &CPythonNetworkStream::__RecvDeadBeavium, 26, false };
	h[Beavium::GC::CHAR_ADDITIONAL_INFO] = { &CPythonNetworkStream::__RecvCharAdditionalInfoBeavium, 129, false };
	h[0x6C]                            = { &CPythonNetworkStream::__RecvTargetHPBeavium, 13, false };

	// Items
	h[Beavium::GC::CHARACTER_UPDATE]   = { &CPythonNetworkStream::__RecvItemSetBeavium, 82, false };
	h[Beavium::GC::ITEM_USE]           = { &CPythonNetworkStream::RecvItemUsePacket, sizeof(TPacketGCItemUse), false };
	h[Beavium::GC::ITEM_UPDATE]        = { &CPythonNetworkStream::__RecvItemUpdateBeavium, 281, false };
	h[Beavium::GC::ITEM_GROUND_ADD]    = { &CPythonNetworkStream::__RecvItemGroundAddBeavium, 17, false };
	h[Beavium::GC::QUICKSLOT_ADD]      = { &CPythonNetworkStream::RecvQuickSlotAddPacket, sizeof(TPacketGCQuickSlotAdd), false };
	h[Beavium::GC::QUICKSLOT_DEL]      = { &CPythonNetworkStream::RecvQuickSlotDelPacket, sizeof(TPacketGCQuickSlotDel), false };
	h[Beavium::GC::QUICKSLOT_SWAP]     = { &CPythonNetworkStream::RecvQuickSlotMovePacket, sizeof(TPacketGCQuickSlotSwap), false };

	// Social / NPC / Shop
	h[Beavium::GC::SHOP]               = { &CPythonNetworkStream::RecvShopPacket, sizeof(TPacketGCShop), false };
	h[Beavium::GC::EXCHANGE]           = { &CPythonNetworkStream::RecvExchangePacket, sizeof(TPacketGCExchange), false };
	h[Beavium::GC::MESSENGER]          = { &CPythonNetworkStream::RecvMessenger, sizeof(TPacketGCMessenger), false };
	h[Beavium::GC::GUILD]              = { &CPythonNetworkStream::RecvGuild, sizeof(TPacketGCGuild), false };
	h[Beavium::GC::NPC_POSITION]       = { &CPythonNetworkStream::RecvNPCList, sizeof(TPacketGCNPCPosition), false };

	// Stats / Skills / Quests
	h[Beavium::GC::CHARACTER_POINTS]   = { &CPythonNetworkStream::__RecvPlayerPoints, 70, false };
	h[Beavium::GC::POINT_CHANGE]       = { &CPythonNetworkStream::__RecvPointChangeBeavium, 3, false };
	h[Beavium::GC::CHANGE_SPEED]       = { &CPythonNetworkStream::RecvChangeSpeedPacket, sizeof(TPacketGCChangeSpeed), false };
	h[Beavium::GC::SKILL_LEVEL]        = { &CPythonNetworkStream::RecvSkillLevel, sizeof(TPacketGCSkillLevel), false };
	h[Beavium::GC::SKILL_COOLTIME_END] = { &CPythonNetworkStream::RecvSkillCoolTimeEnd, sizeof(TPacketGCSkillCoolTimeEnd), false };
	h[Beavium::GC::SCRIPT]             = { &CPythonNetworkStream::__RecvScriptBeavium, 6, false };
	h[Beavium::GC::QUEST_INFO]         = { &CPythonNetworkStream::RecvQuestInfoPacket, sizeof(TPacketGCQuestInfo), false };
	h[Beavium::GC::SAFEBOX_SET]        = { &CPythonNetworkStream::RecvSafeBoxSetPacket, sizeof(TPacketGCItemSet), false };
	h[Beavium::GC::SAFEBOX_SIZE]       = { &CPythonNetworkStream::RecvSafeBoxSizePacket, sizeof(TPacketGCSafeboxSize), false };
	h[Beavium::GC::AFFECT_ADD]         = { &CPythonNetworkStream::RecvAffectAddPacket, sizeof(TPacketGCAffectAdd), false };
	h[Beavium::GC::AFFECT_REMOVE]      = { &CPythonNetworkStream::RecvAffectRemovePacket, sizeof(TPacketGCAffectRemove), false };
}

void CPythonNetworkStream::OnConnectSuccess()
{
	CNetworkStream::OnConnectSuccess();

	if (GetServerMode() == Network::ServerType::Beavium)
	{
		Tracef("[BEAVIUM] CPythonNetworkStream::OnConnectSuccess (phase=%s, loginKey64=0x%016llX, gameToken64=0x%016llX)\n",
			m_strPhase.c_str(), (unsigned long long)m_loginKey64, (unsigned long long)m_gameToken64);

		if ("Loading" == m_strPhase)
		{
			Beavium::TPacketCGLogin2Beavium loginPacket;
			memset(&loginPacket, 0, sizeof(loginPacket));
			loginPacket.header = Beavium::CG::LOGIN2; // 0x68
			strncpy(loginPacket.username, m_stID.c_str(), sizeof(loginPacket.username) - 1);
			loginPacket.nullByte = 0;
			loginPacket.loginKey = m_gameToken64 != 0 ? m_gameToken64 : m_loginKey64;
			memcpy(loginPacket.clientKey, Beavium::DefaultClientKey, 16);

			Tracef("[BEAVIUM] Sending Login2 (0x68, 56B) to Game Server with token=0x%016llX\n",
				(unsigned long long)loginPacket.loginKey);
			Send(sizeof(loginPacket), &loginPacket);
			__SendInternalBuffer();
		}
		else if ("Login" == m_strPhase || m_strPhase.empty() || "OffLine" == m_strPhase)
		{
			SetLoginPhase();
		}
	}
	else
	{
		Tracef("[OFFICIAL] CPythonNetworkStream::OnConnectSuccess (phase=%s)\n", m_strPhase.c_str());
		SetHandShakePhase();
	}
}

bool CPythonNetworkStream::__RecvTimeSyncBeavium()
{
	uint8_t hdr = 0;
	Recv(1, &hdr);
	uint32_t token = 0;
	uint32_t srvTime = 0;
	uint32_t delta = 0;
	Recv(4, &token);
	Recv(4, &srvTime);
	Recv(4, &delta);

	uint32_t sig1 = 0;
	uint32_t sig2 = 0;
	Beavium::CalculateHandshakeSignatures(token, srvTime, sig1, sig2);

	Beavium::TPacketCGHandshakeAckBeavium ack;
	ack.header = Beavium::CG::HANDSHAKE_ACK; // 0xFF
	ack.token = token;
	ack.sig1 = sig1;
	ack.sig2 = sig2;

	Tracef("[BEAVIUM] Recv 0xFF TimeSync (srvTime=%u). Sending 0xFF ACK...\n", srvTime);
	Send(sizeof(ack), &ack);
	__SendInternalBuffer();
	return true;
}

bool CPythonNetworkStream::__RecvPingBeavium()
{
	uint8_t pingHdr = 0;
	Recv(1, &pingHdr);

	uint8_t pong = Beavium::CG::PONG; // 0xFC
	Send(1, &pong);
	__SendInternalBuffer();
	return true;
}

bool CPythonNetworkStream::__RecvRefineInfoDummy()
{
	uint8_t dummy[2];
	return Recv(2, dummy);
}

bool CPythonNetworkStream::SendLogin2Beavium()
{
	return SendLoginPacketNew(m_stID.c_str(), m_stPassword.c_str());
}

bool CPythonNetworkStream::__RecvAuthSuccessBeavium()
{
	uint8_t hdr = 0;
	Recv(1, &hdr);
	uint64_t key = 0;
	Recv(sizeof(key), &key);
	uint8_t extra = 0;
	Recv(1, &extra);

	Tracef("[BEAVIUM] __RecvAuthSuccessBeavium: loginKey64=0x%016llX\n", (unsigned long long)key);
	SetLoginKey64(key);
	return true;
}

// --- Packet sequence tracking ---

void CPythonNetworkStream::LogRecvPacket(uint16_t header, uint16_t length)
{
	auto& e = m_aRecentRecvPackets[m_dwRecvPacketSeq % PACKET_LOG_SIZE];
	e.seq = m_dwRecvPacketSeq;
	e.header = header;
	e.length = length;
	m_dwRecvPacketSeq++;
}

void CPythonNetworkStream::DumpRecentPackets() const
{
	const uint32_t recvCount = std::min(m_dwRecvPacketSeq, (uint32_t)PACKET_LOG_SIZE);
	const uint32_t sentCount = std::min(m_dwSentPacketSeq, (uint32_t)SENT_PACKET_LOG_SIZE);

	TraceError("=== Recent RECV packets (last %u of %u total) ===", recvCount, m_dwRecvPacketSeq);

	for (uint32_t i = 0; i < recvCount; i++)
	{
		uint32_t idx = (m_dwRecvPacketSeq > PACKET_LOG_SIZE)
			? (m_dwRecvPacketSeq - PACKET_LOG_SIZE + i)
			: i;
		const auto& e = m_aRecentRecvPackets[idx % PACKET_LOG_SIZE];
		TraceError("  RECV #%u: header=0x%04X len=%u", e.seq, e.header, e.length);
	}

	TraceError("=== Recent SENT packets (last %u of %u total) ===", sentCount, m_dwSentPacketSeq);

	for (uint32_t i = 0; i < sentCount; i++)
	{
		uint32_t idx = (m_dwSentPacketSeq > SENT_PACKET_LOG_SIZE)
			? (m_dwSentPacketSeq - SENT_PACKET_LOG_SIZE + i)
			: i;
		const auto& e = m_aSentPacketLog[idx % SENT_PACKET_LOG_SIZE];
		TraceError("  SENT #%u: header=0x%04X len=%u", e.seq, e.header, e.length);
	}
}
