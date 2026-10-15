#include "PhaseGamePartyBridge.h"
#include <cstring>
#include <string>

namespace Client::Bridge {

    bool PhaseGamePartyBridge::HandlePartyInvite(const TPacketGCPartyInvite& pack, std::string_view leaderName) {
        std::span<const uint8_t> span(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
        auto res = Client::Network::PartyPacketCodec::DecodePartyInvite(span);
        if (!res.has_value()) {
            return false;
        }

        const auto& decodedPack = res.value();
        m_pythonRuntime.Invoke("game", "RecvPartyInviteQuestion", std::to_string(decodedPack.leader_pid), std::string(leaderName));
        return true;
    }

    bool PhaseGamePartyBridge::HandlePartyAdd(const TPacketGCPartyAdd& pack) {
        std::span<const uint8_t> span(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
        auto res = Client::Network::PartyPacketCodec::DecodePartyAdd(span);
        if (!res.has_value()) {
            return false;
        }

        const auto& decodedPack = res.value();
        std::string memberName(decodedPack.name, strnlen(decodedPack.name, sizeof(decodedPack.name)));
        m_pythonRuntime.Invoke("game", "AddPartyMember", std::to_string(decodedPack.pid), memberName);
        return true;
    }

    bool PhaseGamePartyBridge::HandlePartyUpdate(const TPacketGCPartyUpdate& pack) {
        std::span<const uint8_t> span(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
        auto res = Client::Network::PartyPacketCodec::DecodePartyUpdate(span);
        if (!res.has_value()) {
            return false;
        }

        const auto& decodedPack = res.value();
        m_pythonRuntime.Invoke("game", "UpdatePartyMemberInfo", std::to_string(decodedPack.pid));
        return true;
    }

    bool PhaseGamePartyBridge::HandlePartyRemove(const TPacketGCPartyRemove& pack) {
        std::span<const uint8_t> span(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
        auto res = Client::Network::PartyPacketCodec::DecodePartyRemove(span);
        if (!res.has_value()) {
            return false;
        }

        const auto& decodedPack = res.value();
        m_pythonRuntime.Invoke("game", "RemovePartyMember", std::to_string(decodedPack.pid));
        return true;
    }

    bool PhaseGamePartyBridge::HandlePartyParameter(const TPacketGCPartyParameter& pack) {
        std::span<const uint8_t> span(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
        auto res = Client::Network::PartyPacketCodec::DecodePartyParameter(span);
        if (!res.has_value()) {
            return false;
        }

        const auto& decodedPack = res.value();
        m_pythonRuntime.Invoke("game", "ChangePartyParameter", std::to_string(decodedPack.bDistributeMode));
        return true;
    }

} // namespace Client::Bridge
