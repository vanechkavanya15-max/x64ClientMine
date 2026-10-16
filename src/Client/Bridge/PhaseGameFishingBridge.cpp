#include "PhaseGameFishingBridge.h"
#include "Client/Network/WorldPacketCodec.h"
#include "EterBase/ModernLogger.h"

namespace Client::Bridge {

    uint16_t PhaseGameFishingBridge::GetExpectedSize() const {
        return sizeof(TPacketGCFishing);
    }

    bool PhaseGameFishingBridge::IsDynamicSize() const {
        return false;
    }

    EterBase::PacketResult<void> PhaseGameFishingBridge::Handle(std::span<const uint8_t> payload) {
        auto res = Client::Network::WorldPacketCodec::DecodeFishing(payload);
        if (!res.has_value()) {
            return std::unexpected(res.error());
        }

        const auto& packet = res.value();

        bool isMainActor = false;
        
        // This simulates CPythonCharacterManager::Instance().GetInstancePtr(packet.info) checking
        // if the target info is main actor vs another actor. We delegate this to python:
        
        switch (packet.subheader) {
            case FishingSub::GC::START:
                m_pythonRuntime.Invoke("character", "StartFishing", packet.info, static_cast<float>(packet.dir) * 5.0f);
                break;
            case FishingSub::GC::STOP:
                m_pythonRuntime.Invoke("character", "StopFishing", packet.info);
                break;
            case FishingSub::GC::REACT:
                m_pythonRuntime.Invoke("character", "ReactFishing", packet.info);
                break;
            case FishingSub::GC::SUCCESS:
                m_pythonRuntime.Invoke("character", "CatchSuccess", packet.info);
                break;
            case FishingSub::GC::FAIL:
                m_pythonRuntime.Invoke("character", "CatchFail", packet.info);
                m_pythonRuntime.Invoke("game", "OnFishingFailure");
                break;
            case FishingSub::GC::FISH:
                if (packet.info == 0) {
                    m_pythonRuntime.Invoke("game", "OnFishingNotifyUnknown");
                } else {
                    // For subheader FISH, the original code used Python API:
                    // isMainActor was checked. If true, OnFishingNotify/OnFishingSuccess.
                    // This is done cleanly via python runtime script delegation.
                    m_pythonRuntime.Invoke("game", "OnFishingNotifyOrSuccess", packet.info);
                }
                break;
            default:
                EterBase::ModernLogger::Error("PhaseGameFishingBridge: unknown subheader {}", packet.subheader);
                return std::unexpected(EterBase::PacketError::InvalidHeader);
        }

        return {};
    }

} // namespace Client::Bridge
