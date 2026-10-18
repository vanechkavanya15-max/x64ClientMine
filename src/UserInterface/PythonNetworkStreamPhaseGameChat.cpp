#include "StdAfx.h"
#include "PythonNetworkStreamPhaseGameChat.h"
#include "PythonNetworkStream.h"
#include "PythonChat.h"
#include "PythonTextTail.h"
#include "PythonCharacterManager.h"
#include "InstanceBase.h"
#include <algorithm>
#include <cstring>
#include <cassert>
#include <string_view>
#include "Client/Network/Protocol/ProtocolDriverRegistry.h"

extern BOOL gs_bEmpireLanuageEnable;

static PyObject* BuildSafePyString(const char* str)
{
	if (!str)
		return PyUnicode_FromString("");
	PyObject* obj = PyUnicode_FromString(str);
	if (!obj)
	{
		PyErr_Clear();
		obj = PyUnicode_DecodeLatin1(str, strlen(str), "replace");
	}
	return obj;
}

bool PhaseGameChatBridge::HandleChat(CPythonNetworkStream* pStream)
{
	if (!pStream)
		return false;

	auto* pDriver = Network::Protocol::ProtocolDriverRegistry::Instance().GetActiveDriver();
	std::string_view driverName = pDriver ? pDriver->GetDriverName() : "standard_x64";

	TPacketGCChat kChat{};
	char buf[1024 + 1];
	UINT uChatSize = 0;

	if (driverName == "beavium")
	{
#pragma pack(push, 1)
		struct TPacketGCChatBeaviumHeader
		{
			uint8_t header;
			uint16_t length;
			uint8_t type;
			uint32_t dwVID;
			uint8_t bEmpire;
			uint8_t reserved[5];
		};
#pragma pack(pop)
		static_assert(sizeof(TPacketGCChatBeaviumHeader) == 14, "Beavium chat header must be 14 bytes");

		TPacketGCChatBeaviumHeader beaviumHeader{};
		if (!pStream->Peek(sizeof(beaviumHeader), &beaviumHeader))
			return false;

		if (beaviumHeader.length < sizeof(beaviumHeader))
			return false;

		if (!pStream->Peek(beaviumHeader.length))
			return false;

		if (!pStream->Recv(sizeof(beaviumHeader), &beaviumHeader))
			return false;

		kChat.header = beaviumHeader.header;
		kChat.length = beaviumHeader.length;
		kChat.type = beaviumHeader.type;
		kChat.dwVID = beaviumHeader.dwVID;
		kChat.bEmpire = beaviumHeader.bEmpire;

		uChatSize = beaviumHeader.length - sizeof(beaviumHeader);
	}
	else if (driverName == "luria" || driverName == "martysama")
	{
#pragma pack(push, 1)
		struct TPacketGCChat8BitHeader
		{
			uint8_t header;
			uint16_t length;
			uint8_t type;
			uint32_t dwVID;
			uint8_t bEmpire;
		};
#pragma pack(pop)
		static_assert(sizeof(TPacketGCChat8BitHeader) == 9, "8-bit chat header must be 9 bytes");

		TPacketGCChat8BitHeader h8{};
		if (!pStream->Peek(sizeof(h8), &h8))
			return false;

		if (h8.length < sizeof(h8))
			return false;

		if (!pStream->Peek(h8.length))
			return false;

		if (!pStream->Recv(sizeof(h8), &h8))
			return false;

		kChat.header = h8.header;
		kChat.length = h8.length;
		kChat.type = h8.type;
		kChat.dwVID = h8.dwVID;
		kChat.bEmpire = h8.bEmpire;

		uChatSize = h8.length - sizeof(h8);
	}
	else
	{
		if (!pStream->Peek(sizeof(kChat), &kChat))
			return false;

		if (kChat.length < sizeof(kChat))
			return false;

		if (!pStream->Peek(kChat.length))
			return false;

		if (!pStream->Recv(sizeof(kChat), &kChat))
			return false;

		uChatSize = kChat.length - sizeof(kChat);
	}

	if (uChatSize > 1024)
		return false;

	if (uChatSize > 0)
	{
		if (!pStream->Recv(uChatSize, buf))
			return false;
	}

	buf[uChatSize] = '\0';

	return HandleChat(pStream, kChat, buf, uChatSize);
}

bool PhaseGameChatBridge::HandleChat(CPythonNetworkStream* pStream, const TPacketGCChat& kChat, const char* pChatBuf, size_t chatBufSize)
{
	if (!pStream || !pChatBuf)
		return false;

	char buf[1024 + 1];
	size_t copyLen = (chatBufSize < 1024) ? chatBufSize : 1024;
	memcpy(buf, pChatBuf, copyLen);
	buf[copyLen] = '\0';

	char line[1024 + 1];

	// Localize item names in hyperlinks for multi-language support
	pStream->__LocalizeItemLinks(buf, sizeof(buf));

	if (kChat.type >= CHAT_TYPE_MAX_NUM)
		return true;

	if (CHAT_TYPE_COMMAND == kChat.type)
	{
		pStream->ServerCommand(buf);
		return true;
	}

	if (kChat.dwVID != 0)
	{
		CPythonCharacterManager& rkChrMgr = CPythonCharacterManager::Instance();
		CInstanceBase* pkInstChatter = rkChrMgr.GetInstancePtr(kChat.dwVID);
		if (NULL == pkInstChatter)
			return true;

		switch (kChat.type)
		{
		case CHAT_TYPE_TALKING:  /* 그냥 채팅 */
		case CHAT_TYPE_PARTY:    /* 파티말 */
		case CHAT_TYPE_GUILD:    /* 길드말 */
		case CHAT_TYPE_SHOUT:	/* 외치기 */
		case CHAT_TYPE_WHISPER:	// 서버와는 연동되지 않는 Only Client Enum
			{
				char* p = strchr(buf, ':');

				if (p)
					p += 2;
				else
					p = buf;

				DWORD dwEmoticon;

				if (pStream->ParseEmoticon(p, &dwEmoticon))
				{
					pkInstChatter->SetEmoticon(dwEmoticon);
					return true;
				}
				else
				{
					if (gs_bEmpireLanuageEnable)
					{
						CInstanceBase* pkInstMain = rkChrMgr.GetMainActorPtr();
						if (pkInstMain)
							if (!pkInstMain->IsSameEmpire(*pkInstChatter))
								pStream->__ConvertEmpireText(pkInstChatter->GetEmpireID(), p);
					}

					if (pStream->m_isEnableChatInsultFilter)
					{
						if (false == pkInstChatter->IsNPC() && false == pkInstChatter->IsEnemy())
						{
							pStream->__FilterInsult(p, strlen(p));
						}
					}

					_snprintf(line, sizeof(line), "%s", p);
				}
			}
			break;
		case CHAT_TYPE_COMMAND:	/* 명령 */
		case CHAT_TYPE_INFO:     /* 정보 (아이템을 집었다, 경험치를 얻었다. 등) */
		case CHAT_TYPE_NOTICE:   /* 공지사항 */
		case CHAT_TYPE_BIG_NOTICE:
		case CHAT_TYPE_MAX_NUM:
		default:
			_snprintf(line, sizeof(line), "%s", buf);
			break;
		}

		if (CHAT_TYPE_SHOUT != kChat.type)
		{
			CPythonTextTail::Instance().RegisterChatTail(kChat.dwVID, line);
		}

		if (pkInstChatter->IsPC())
			CPythonChat::Instance().AppendChat(kChat.type, buf);
	}
	else
	{
		if (CHAT_TYPE_NOTICE == kChat.type)
		{
			PyObject* pyMsg = BuildSafePyString(buf);
			PyCallClassMemberFunc(pStream->m_apoPhaseWnd[CPythonNetworkStream::PHASE_WINDOW_GAME], "BINARY_SetTipMessage", Py_BuildValue("(N)", pyMsg));
		}
		else if (CHAT_TYPE_BIG_NOTICE == kChat.type)
		{
			PyObject* pyMsg = BuildSafePyString(buf);
			PyCallClassMemberFunc(pStream->m_apoPhaseWnd[CPythonNetworkStream::PHASE_WINDOW_GAME], "BINARY_SetBigMessage", Py_BuildValue("(N)", pyMsg));
		}
		else if (CHAT_TYPE_SHOUT == kChat.type)
		{
			char* p = strchr(buf, ':');

			if (p)
			{
				if (pStream->m_isEnableChatInsultFilter)
					pStream->__FilterInsult(p, strlen(p));
			}
		}

		CPythonChat::Instance().AppendChat(kChat.type, buf);
	}

	return true;
}

bool PhaseGameChatBridge::HandleWhisper(CPythonNetworkStream* pStream)
{
	if (!pStream)
		return false;

	auto* pDriver = Network::Protocol::ProtocolDriverRegistry::Instance().GetActiveDriver();
	std::string_view driverName = pDriver ? pDriver->GetDriverName() : "standard_x64";

	TPacketGCWhisper whisperPacket{};
	char buf[512 + 1];
	UINT uWhisperSize = 0;

	if (driverName == "luria" || driverName == "martysama" || driverName == "beavium")
	{
#pragma pack(push, 1)
		struct TPacketGCWhisper8BitHeader
		{
			uint8_t header;
			uint16_t length;
			uint8_t bType;
			char szNameFrom[CHARACTER_NAME_MAX_LEN + 1];
		};
#pragma pack(pop)

		TPacketGCWhisper8BitHeader h8{};
		if (!pStream->Peek(sizeof(h8), &h8))
			return false;

		if (h8.length < sizeof(h8))
			return false;

		uWhisperSize = h8.length - sizeof(h8);
		assert(uWhisperSize < 512);
		if (uWhisperSize >= 512)
			return false;

		if (!pStream->Peek(h8.length))
			return false;

		if (!pStream->Recv(sizeof(h8), &h8))
			return false;

		whisperPacket.header = h8.header;
		whisperPacket.length = h8.length;
		whisperPacket.bType = h8.bType;
		memcpy(whisperPacket.szNameFrom, h8.szNameFrom, sizeof(whisperPacket.szNameFrom));
	}
	else
	{
		if (!pStream->Peek(sizeof(whisperPacket), &whisperPacket))
			return false;

		if (whisperPacket.length < sizeof(whisperPacket))
			return false;

		uWhisperSize = whisperPacket.length - sizeof(whisperPacket);
		assert(uWhisperSize < 512);
		if (uWhisperSize >= 512)
			return false;

		if (!pStream->Peek(whisperPacket.length))
			return false;

		if (!pStream->Recv(sizeof(whisperPacket), &whisperPacket))
			return false;
	}

	if (uWhisperSize > 0)
	{
		if (!pStream->Recv(uWhisperSize, buf))
			return false;
	}

	buf[uWhisperSize] = '\0';

	return HandleWhisper(pStream, whisperPacket, buf, uWhisperSize);
}

bool PhaseGameChatBridge::HandleWhisper(CPythonNetworkStream* pStream, const TPacketGCWhisper& whisperPacket, const char* pChatBuf, size_t chatBufSize)
{
	if (!pStream || !pChatBuf)
		return false;

	char buf[512 + 1];
	size_t copyLen = (chatBufSize < 512) ? chatBufSize : 512;
	memcpy(buf, pChatBuf, copyLen);
	buf[copyLen] = '\0';

	static char line[256];
	if (CPythonChat::WHISPER_TYPE_CHAT == whisperPacket.bType || CPythonChat::WHISPER_TYPE_GM == whisperPacket.bType)
	{		
		_snprintf(line, sizeof(line), "%s : %s", whisperPacket.szNameFrom, buf);
		PyCallClassMemberFunc(pStream->m_apoPhaseWnd[CPythonNetworkStream::PHASE_WINDOW_GAME], "OnRecvWhisper", Py_BuildValue("(iss)", (int) whisperPacket.bType, whisperPacket.szNameFrom, line));
	}
	else if (CPythonChat::WHISPER_TYPE_SYSTEM == whisperPacket.bType || CPythonChat::WHISPER_TYPE_ERROR == whisperPacket.bType)
	{
		PyCallClassMemberFunc(pStream->m_apoPhaseWnd[CPythonNetworkStream::PHASE_WINDOW_GAME], "OnRecvWhisperSystemMessage", Py_BuildValue("(iss)", (int) whisperPacket.bType, whisperPacket.szNameFrom, buf));
	}
	else
	{
		PyCallClassMemberFunc(pStream->m_apoPhaseWnd[CPythonNetworkStream::PHASE_WINDOW_GAME], "OnRecvWhisperError", Py_BuildValue("(iss)", (int) whisperPacket.bType, whisperPacket.szNameFrom, buf));
	}

	return true;
}

bool PhaseGameChatBridge::SendChat(CPythonNetworkStream* pStream, const char* c_szChat, BYTE byType)
{
	if (!pStream || !c_szChat)
		return false;

	if (strlen(c_szChat) == 0)
		return true;

	if (strlen(c_szChat) >= 512)
		return true;

	if (c_szChat[0] == '/')
	{
		if (1 == strlen(c_szChat))
		{
			if (!pStream->m_strLastCommand.empty())
				c_szChat = pStream->m_strLastCommand.c_str();
		}
		else
		{
			pStream->m_strLastCommand = c_szChat;
		}
	}

	if (pStream->ClientCommand(c_szChat))
		return true;

	int iTextLen = (int)strlen(c_szChat) + 1;

	auto* pDriver = Network::Protocol::ProtocolDriverRegistry::Instance().GetActiveDriver();
	std::string_view driverName = pDriver ? pDriver->GetDriverName() : "standard_x64";

	if (driverName == "luria" || driverName == "martysama" || driverName == "beavium")
	{
#pragma pack(push, 1)
		struct TPacketCGChat8Bit
		{
			uint8_t header;
			uint16_t length;
			uint8_t type;
		};
#pragma pack(pop)
		static_assert(sizeof(TPacketCGChat8Bit) == 4, "TPacketCGChat8Bit must be 4 bytes");

		TPacketCGChat8Bit chatPacket;
		chatPacket.header = (driverName == "luria") ? 0x04 : 0x03;
		chatPacket.length = static_cast<uint16_t>(sizeof(chatPacket) + iTextLen);
		chatPacket.type = byType;

		if (!pStream->Send(sizeof(chatPacket), &chatPacket))
			return false;

		if (!pStream->Send(iTextLen, c_szChat))
			return false;

		return true;
	}
	else
	{
		TPacketCGChat ChatPacket;
		ChatPacket.header = CG::CHAT;
		ChatPacket.length = sizeof(ChatPacket) + iTextLen;
		ChatPacket.type = byType;

		if (!pStream->Send(sizeof(ChatPacket), &ChatPacket))
			return false;

		if (!pStream->Send(iTextLen, c_szChat))
			return false;

		return true;
	}
}

bool PhaseGameChatBridge::SendWhisper(CPythonNetworkStream* pStream, const char* name, const char* c_szChat)
{
	if (!pStream || !name || !c_szChat)
		return false;

	if (strlen(c_szChat) >= 255)
		return true;

	int iTextLen = (int)strlen(c_szChat) + 1;

	auto* pDriver = Network::Protocol::ProtocolDriverRegistry::Instance().GetActiveDriver();
	std::string_view driverName = pDriver ? pDriver->GetDriverName() : "standard_x64";

	if (driverName == "luria" || driverName == "martysama" || driverName == "beavium")
	{
#pragma pack(push, 1)
		struct TPacketCGWhisper8Bit
		{
			uint8_t header;
			uint16_t length;
			char szNameTo[CHARACTER_NAME_MAX_LEN + 1];
		};
#pragma pack(pop)

		TPacketCGWhisper8Bit whisperPacket;
		whisperPacket.header = (driverName == "martysama") ? 0x13 : 0x19;
		whisperPacket.length = static_cast<uint16_t>(sizeof(whisperPacket) + iTextLen);

		strncpy(whisperPacket.szNameTo, name, sizeof(whisperPacket.szNameTo) - 1);
		whisperPacket.szNameTo[sizeof(whisperPacket.szNameTo) - 1] = '\0';

		if (!pStream->Send(sizeof(whisperPacket), &whisperPacket))
			return false;

		if (!pStream->Send(iTextLen, c_szChat))
			return false;

		return true;
	}
	else
	{
		TPacketCGWhisper WhisperPacket;
		WhisperPacket.header = CG::WHISPER;
		WhisperPacket.length = sizeof(WhisperPacket) + iTextLen;

		strncpy(WhisperPacket.szNameTo, name, sizeof(WhisperPacket.szNameTo) - 1);
		WhisperPacket.szNameTo[sizeof(WhisperPacket.szNameTo) - 1] = '\0';

		if (!pStream->Send(sizeof(WhisperPacket), &WhisperPacket))
			return false;

		if (!pStream->Send(iTextLen, c_szChat))
			return false;

		return true;
	}
}
