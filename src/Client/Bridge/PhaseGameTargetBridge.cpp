#include "PhaseGameTargetBridge.h"

namespace Client::Bridge {

    EterBase::PacketResult<void> PhaseGameTargetBridge::HandleTargetCreate(std::span<const uint8_t> payload) {
        auto result = Client::Network::TargetPacketCodec::DecodeTargetCreate(payload);
        if (!result) {
            return std::unexpected(result.error());
        }

        m_pythonRuntime.Invoke("minimap", "CreateTarget", result->lID, std::string(result->szTargetName));
        m_pythonRuntime.Invoke("game", "BINARY_OpenAtlasWindow");
        return {};
    }

    EterBase::PacketResult<void> PhaseGameTargetBridge::HandleTargetUpdate(std::span<const uint8_t> payload) {
        auto result = Client::Network::TargetPacketCodec::DecodeTargetUpdate(payload);
        if (!result) {
            return std::unexpected(result.error());
        }

        m_pythonRuntime.Invoke("minimap", "UpdateTarget", result->lID, result->lX, result->lY);
        m_pythonRuntime.Invoke("background", "CreateTargetEffect", result->lID, result->lX, result->lY);
        return {};
    }

    EterBase::PacketResult<void> PhaseGameTargetBridge::HandleTargetDelete(std::span<const uint8_t> payload) {
        auto result = Client::Network::TargetPacketCodec::DecodeTargetDelete(payload);
        if (!result) {
            return std::unexpected(result.error());
        }

        m_pythonRuntime.Invoke("minimap", "DeleteTarget", result->lID);
        m_pythonRuntime.Invoke("background", "DeleteTargetEffect", result->lID);
        return {};
    }

    EterBase::PacketResult<void> PhaseGameTargetBridge::HandleCreateFly(std::span<const uint8_t> payload) {
        auto result = Client::Network::TargetPacketCodec::DecodeCreateFly(payload);
        if (!result) {
            return std::unexpected(result.error());
        }

        m_pythonRuntime.Invoke("fly_manager", "CreateIndexedFly", result->bType, result->dwStartVID, result->dwEndVID);
        return {};
    }

} // namespace Client::Bridge
