#include "../StdAfx.h"
#include "HarnessNetworkAuthProvider.h"
#include "../PythonNetworkStream.h"
#include "../AccountConnector.h"
#include "Client/Network/Protocol/ProtocolDriverRegistry.h"
#include <string>

namespace UserInterface::TestHarness
{
    void HarnessNetworkAuthProvider::SetConnectionTarget(std::string_view serverIp, int authPort, int gamePort, std::string_view pin, int slot)
    {
        m_serverIp = std::string(serverIp);
        m_authPort = authPort;
        m_gamePort = gamePort;
        m_pin = std::string(pin);
        m_slot = slot;
    }

    std::expected<void, Client::IPC::AuthError> HarnessNetworkAuthProvider::SendLogin(std::string_view user, std::string_view pass)
    {
        auto* pActiveDriver = ::Network::Protocol::ProtocolDriverRegistry::Instance().GetActiveDriver();
        if (pActiveDriver && pActiveDriver->GetDriverName() == "luria")
        {
            if (m_serverIp == "127.0.0.1")
                m_serverIp = "46.105.73.82";
            if (m_authPort == 11000)
                m_authPort = 11469;
            if (m_gamePort == 11011)
                m_gamePort = 29001;
        }

        CAccountConnector& rkAccountConnector = CAccountConnector::Instance();
        CPythonNetworkStream& rkNetStream = CPythonNetworkStream::Instance();

        std::string userStr(user);
        std::string passStr(pass);

        rkAccountConnector.SetLoginInfo(userStr.c_str(), passStr.c_str(), m_pin.c_str());
        rkNetStream.SetLoginInfo(userStr.c_str(), passStr.c_str(), m_pin.c_str());
        rkNetStream.SetDirectEnterMode(m_slot);

        bool connOk = rkAccountConnector.Connect(m_serverIp.c_str(), m_gamePort, m_serverIp.c_str(), m_authPort);
        if (!connOk)
        {
            return std::unexpected(Client::IPC::AuthError::NetworkError);
        }

        return {};
    }

    std::expected<std::vector<Client::IPC::CharacterSlotInfo>, Client::IPC::AuthError> HarnessNetworkAuthProvider::RequestCharacterSlots()
    {
        CPythonNetworkStream& rkNet = CPythonNetworkStream::Instance();
        std::vector<Client::IPC::CharacterSlotInfo> slots;

        for (UINT i = 0; i < PLAYER_PER_ACCOUNT4; ++i)
        {
            const TSimplePlayerInformation* pInfo = rkNet.GetSimplePlayerInfoPtr(i);
            if (pInfo && pInfo->dwID != 0)
            {
                slots.push_back(Client::IPC::CharacterSlotInfo{
                    .slotIndex = static_cast<uint8_t>(i),
                    .name = pInfo->szName ? pInfo->szName : "",
                    .level = static_cast<uint8_t>(pInfo->byLevel),
                    .job = static_cast<uint8_t>(pInfo->byJob),
                    .playtime = pInfo->dwPlayMinutes * 60
                });
            }
        }

        return slots;
    }

    std::expected<void, Client::IPC::AuthError> HarnessNetworkAuthProvider::SendSelectCharacter(uint8_t slotIndex)
    {
        if (Py_IsInitialized())
        {
            PyGILState_STATE gstate = PyGILState_Ensure();
            CPythonNetworkStream& rkNetStream = CPythonNetworkStream::Instance();
            PyObject* poSelectWnd = rkNetStream.GetPhaseWindow(CPythonNetworkStream::PHASE_WINDOW_SELECT);
            if (poSelectWnd)
            {
                PyCallClassMemberFunc(poSelectWnd, "DirectStartGame", Py_BuildValue("(i)", static_cast<int>(slotIndex)));
            }
            else
            {
                PyObject* poHandler = rkNetStream.GetHandler();
                if (poHandler)
                {
                    PyCallClassMemberFunc(poHandler, "SetCharacterSlot", Py_BuildValue("(i)", static_cast<int>(slotIndex)));
                    PyCallClassMemberFunc(poHandler, "SetLoadingPhase", Py_BuildValue("()"));
                }
                else
                {
                    rkNetStream.SendSelectCharacterPacket(static_cast<BYTE>(slotIndex));
                }
            }
            if (PyErr_Occurred())
                PyErr_Clear();
            PyGILState_Release(gstate);
        }
        else
        {
            CPythonNetworkStream::Instance().SendSelectCharacterPacket(static_cast<BYTE>(slotIndex));
        }

        return {};
    }

    std::expected<void, Client::IPC::AuthError> HarnessNetworkAuthProvider::SendCreateCharacter(uint8_t slotIndex, std::string_view name, uint8_t job, uint8_t shape)
    {
        BYTE bJob = static_cast<BYTE>(job);
        BYTE bShape = static_cast<BYTE>(shape);
        BYTE bCon = 4, bInt = 3, bStr = 6, bDex = 3;
        if (job == 1 || job == 5) { bCon = 3; bInt = 3; bStr = 4; bDex = 6; }
        else if (job == 2 || job == 6) { bCon = 3; bInt = 5; bStr = 5; bDex = 3; }
        else if (job == 3 || job == 7) { bCon = 4; bInt = 6; bStr = 3; bDex = 3; }

        std::string nameStr(name);
        bool ok = CPythonNetworkStream::Instance().SendCreateCharacterPacket(
            static_cast<BYTE>(slotIndex),
            nameStr.c_str(),
            bJob,
            bShape,
            bCon,
            bInt,
            bStr,
            bDex
        );

        if (!ok)
        {
            return std::unexpected(Client::IPC::AuthError::NetworkError);
        }

        return {};
    }
}
