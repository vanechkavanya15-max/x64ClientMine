#include "StdAfx.h"
#include "AccountConnector.h"
#include "Packet.h"
#include "PythonNetworkStream.h"
#include "BeaviumProtocol.h"

void CAccountConnector::SetHandler(PyObject* poHandler)
{
	m_poHandler = poHandler;
}

void CAccountConnector::SetLoginInfo(const char * c_szName, const char * c_szPwd, const char * c_szPin)
{
	m_strID = c_szName;
	m_strPassword = c_szPwd;
	if (c_szPin && *c_szPin)
		m_strPin = c_szPin;
	else if (m_strPin.empty())
		m_strPin = "3583";
}

void CAccountConnector::SetPin(const char * c_szPin)
{
	if (c_szPin && *c_szPin)
		m_strPin = c_szPin;
}

void CAccountConnector::ClearLoginInfo( void )
{
	m_strPassword = "";
	m_strPin = "";
}

bool CAccountConnector::Connect(const char * c_szAddr, int iPort, const char * c_szAccountAddr, int iAccountPort)
{
	m_strAddr = c_szAddr;
	m_iPort = iPort;
	__OfflineState_Set();

	return CNetworkStream::Connect(c_szAccountAddr, iAccountPort);
}

void CAccountConnector::Disconnect()
{
	CNetworkStream::Disconnect();
	__OfflineState_Set();
}

void CAccountConnector::Process()
{
	CNetworkStream::Process();

	if (!__StateProcess())
	{
		__OfflineState_Set();
		Disconnect();
	}
}

bool CAccountConnector::__StateProcess()
{
	switch (m_eState)
	{
		case STATE_HANDSHAKE:
			return __HandshakeState_Process();
			break;
		case STATE_AUTH:
			return __AuthState_Process();
			break;
	}

	return true;
}

bool CAccountConnector::__HandshakeState_Process()
{
	m_eState = STATE_AUTH;
	return __AuthState_Process();
}

bool CAccountConnector::__AuthState_Process()
{
	int bufSize = GetRecvBufferSize();
	if (bufSize <= 0)
		return true;

	uint8_t header = 0;
	if (!Peek(sizeof(uint8_t), &header))
		return true;

	if (header == Beavium::GC::PHASE) // 0xFD
	{
		return __AuthState_RecvPhase();
	}
	else if (header == Beavium::GC::AUTH_KEY) // 0x96
	{
		return __AuthState_RecvAuthSuccess();
	}
	else if (header == Beavium::GC::LOGIN_FAILURE || header == Beavium::GC::LOGIN_SUCCESS3) // 0x07 or 0x06
	{
		return __AuthState_RecvAuthFailure();
	}

	// Scan buffer for 0x96 in case any unparsed bytes arrived before it
	std::vector<uint8_t> temp(bufSize);
	Peek(bufSize, temp.data());
	for (int i = 0; i < bufSize; ++i)
	{
		if (temp[i] == Beavium::GC::AUTH_KEY && (bufSize - i) >= 10)
		{
			Tracef("[BEAVIUM-AUTH] Found 0x96 at offset %d, skipping %d bytes of intermediate data\n", i, i);
			if (i > 0)
			{
				std::vector<uint8_t> junk(i);
				Recv(i, junk.data());
			}
			return __AuthState_RecvAuthSuccess();
		}
	}

	// Discard intermediate / unhandled bytes cleanly
	uint8_t dummy;
	Recv(1, &dummy);
	return true;
}

bool CAccountConnector::__AuthState_RecvEmpty()
{
	BYTE byEmpty;
	Recv(sizeof(BYTE), &byEmpty);
	return true;
}

bool CAccountConnector::__AuthState_RecvPhase()
{
	uint8_t hdr = 0;
	uint8_t phase = 0;
	if (!Recv(1, &hdr) || !Recv(1, &phase))
		return false;

	Tracef("[BEAVIUM-AUTH] Phase packet received: 0x%02X (phase %u). Building and sending 0x67 auth frame for '%s'...\n",
		hdr, phase, m_strID.c_str());

	std::vector<uint8_t> frame;
	const std::string& pin = m_strPin.empty() ? "3583" : m_strPin;
	if (!Beavium::BuildAuthLoginFrame(m_strID, m_strPassword, pin, static_cast<uint16_t>(m_iPort), Beavium::DefaultClientKey, frame))
	{
		TraceError("[BEAVIUM-AUTH] Failed to build 0x67 auth login frame!");
		return false;
	}

	if (!Send(static_cast<int>(frame.size()), frame.data()))
	{
		TraceError("[BEAVIUM-AUTH] Failed to send 0x67 auth login frame!");
		return false;
	}

	__SendInternalBuffer();
	return true;
}

bool CAccountConnector::__AuthState_RecvAuthSuccess()
{
	if (GetRecvBufferSize() < 10)
		return true; // Wait for full 10 bytes

	uint8_t hdr = 0;
	Recv(1, &hdr); // 0x96
	uint64_t loginKey64 = 0;
	Recv(sizeof(loginKey64), &loginKey64);
	uint8_t extra = 0;
	Recv(1, &extra);

	Tracef("[BEAVIUM-AUTH] AUTH SUCCESS! LoginKey64 = 0x%016llX. Connecting to Lobby at %s:%d...\n",
		(unsigned long long)loginKey64, m_strAddr.c_str(), m_iPort);

	CPythonNetworkStream & rkNet = CPythonNetworkStream::Instance();
	rkNet.SetLoginKey64(loginKey64);
	rkNet.SetLoginInfo(m_strID.c_str(), m_strPassword.c_str());
	rkNet.Connect(m_strAddr.c_str(), m_iPort);

	Disconnect();
	__OfflineState_Set();
	return true;
}

bool CAccountConnector::__AuthState_RecvAuthFailure()
{
	uint8_t hdr = 0;
	Recv(1, &hdr);
	char szStatus[128] = { 0 };
	int avail = std::min(GetRecvBufferSize(), static_cast<int>(sizeof(szStatus) - 1));
	if (avail > 0)
		Recv(avail, szStatus);

	TraceError("[BEAVIUM-AUTH] Auth Failure: %s", szStatus);

	if (m_poHandler)
		PyCallClassMemberFunc(m_poHandler, "OnLoginFailure", Py_BuildValue("(s)", szStatus));

	Disconnect();
	__OfflineState_Set();
	return true;
}

bool CAccountConnector::__AnalyzePacket(UINT uHeader, UINT uPacketSize, bool (CAccountConnector::*pfnDispatchPacket)())
{
	uint16_t wHeader;
	if (!Peek(sizeof(wHeader), &wHeader))
		return true;

	if (wHeader != uHeader)
		return true;

	if (!Peek(uPacketSize))
		return true;

	return (this->*pfnDispatchPacket)();
}

void CAccountConnector::__OfflineState_Set()
{
	__Inialize();
}

void CAccountConnector::__HandshakeState_Set()
{
	m_eState=STATE_HANDSHAKE;
}

void CAccountConnector::__AuthState_Set()
{
	m_eState=STATE_AUTH;
}

void CAccountConnector::OnConnectFailure()
{
	if (m_poHandler)
		PyCallClassMemberFunc(m_poHandler, "OnConnectFailure", Py_BuildValue("()"));

	__OfflineState_Set();
}

void CAccountConnector::OnConnectSuccess()
{
	CNetworkStream::OnConnectSuccess();
	m_eState = STATE_AUTH;
}

void CAccountConnector::OnRemoteDisconnect()
{
	if (m_isWaitKey)
	{
		if (m_poHandler)
		{
			PyCallClassMemberFunc(m_poHandler, "OnExit", Py_BuildValue("()"));
			return;
		}
	}

	__OfflineState_Set();
}

void CAccountConnector::OnDisconnect()
{
	__OfflineState_Set();
}

void CAccountConnector::__Inialize()
{
	m_eState=STATE_OFFLINE;
	m_isWaitKey = FALSE;
	m_strPin = "3583";
}

CAccountConnector::CAccountConnector()
{
	m_isAccountConnector = true;
	m_poHandler = NULL;
	m_strAddr = "";
	m_iPort = 0;

	SetLoginInfo("", "");
	SetRecvBufferSize(1024 * 128);
	SetSendBufferSize(2048);
	__Inialize();
}

CAccountConnector::~CAccountConnector()
{
	__OfflineState_Set();
}
