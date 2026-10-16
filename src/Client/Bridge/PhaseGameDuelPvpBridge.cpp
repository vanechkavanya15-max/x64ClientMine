#include "PhaseGameDuelPvpBridge.h"

namespace Client::Bridge {

    EterBase::PacketResult<void> PhaseGameDuelPvpBridge::HandlePvpPacket(const TPacketGCPVP& packet)
    {
        const uint32_t sourceId = packet.dwVIDSrc;
        const uint32_t targetId = packet.dwVIDDst;

        switch (packet.bMode)
        {
            case PVP_MODE_AGREE:
                m_pythonRuntime.Invoke("character", "RemovePVPKey", sourceId, targetId);
                m_pythonRuntime.Invoke("player", "RememberChallengeInstance", sourceId, targetId);
                break;
            case PVP_MODE_REVENGE:
                m_pythonRuntime.Invoke("character", "RemovePVPKey", sourceId, targetId);
                m_pythonRuntime.Invoke("player", "RememberRevengeInstance", sourceId, targetId);
                break;
            case PVP_MODE_FIGHT:
                m_pythonRuntime.Invoke("character", "InsertPVPKey", sourceId, targetId);
                m_pythonRuntime.Invoke("player", "ForgetInstance", sourceId);
                m_pythonRuntime.Invoke("player", "ForgetInstance", targetId);
                break;
            case PVP_MODE_NONE:
                m_pythonRuntime.Invoke("character", "RemovePVPKey", sourceId, targetId);
                m_pythonRuntime.Invoke("player", "ForgetInstance", sourceId);
                m_pythonRuntime.Invoke("player", "ForgetInstance", targetId);
                break;
            default:
                return std::unexpected(EterBase::PacketError::MalformedPayload);
        }

        m_pythonRuntime.Invoke("character", "RefreshTargetBoardByVID", sourceId);
        m_pythonRuntime.Invoke("character", "RefreshTargetBoardByVID", targetId);

        return {};
    }

    EterBase::PacketResult<void> PhaseGameDuelPvpBridge::HandleDuelStartPacket(std::span<const uint32_t> duelPairs, uint32_t mainCharacterVid)
    {
        if (mainCharacterVid == 0)
        {
            return std::unexpected(EterBase::PacketError::MalformedPayload);
        }

        for (uint32_t targetVid : duelPairs)
        {
            m_pythonRuntime.Invoke("character", "InsertDUELKey", mainCharacterVid, targetVid);
        }

        if (duelPairs.empty())
        {
            m_pythonRuntime.Invoke("player", "SetDuelMode", 2); // DUEL_CANNOTATTACK
        }
        else
        {
            m_pythonRuntime.Invoke("player", "SetDuelMode", 1); // DUEL_START
        }

        m_pythonRuntime.Invoke("game", "CloseTargetBoard");
        m_pythonRuntime.Invoke("character", "RefreshAllPCTextTail");

        return {};
    }

} // namespace Client::Bridge
