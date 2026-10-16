#include "PhaseGameMiningBridge.h"
#include <cstring>

#ifndef TEST_MODE_DISABLE_STDAFX
class CInstanceBase {
public:
    void NEW_LookAtDestInstance(CInstanceBase& rkInstDst);
    void PushOnceMotion(uint16_t wMotion, float fBlendTime = 0.1f, float fSpeedRatio = 1.0f);
};

class IAbstractCharacterManager {
public:
    virtual ~IAbstractCharacterManager() = default;
    virtual CInstanceBase* GetInstancePtr(uint32_t dwVID) = 0;
};
#include "GameLib/RaceMotionData.h"
#endif

namespace Client::Bridge {

    EterBase::PacketResult<void> PhaseGameMiningBridge::Handle(std::span<const uint8_t> payload) {
        if (payload.size() < GetExpectedSize()) {
            return std::unexpected(EterBase::PacketError::BufferUnderflow);
        }

        TPacketGCDigMotion pack{};
        std::memcpy(&pack, payload.data(), sizeof(pack));

        CInstanceBase* pkInstMain = m_characterManager.GetInstancePtr(pack.vid);
        CInstanceBase* pkInstTarget = m_characterManager.GetInstancePtr(pack.target_vid);

        if (pkInstMain == nullptr) {
            // Ignorujemy brak aktora (zgodnie z legacy: return true)
            return {};
        }

        if (pkInstTarget != nullptr) {
            pkInstMain->NEW_LookAtDestInstance(*pkInstTarget);
        }

        for (int i = 0; i < pack.count; ++i) {
            pkInstMain->PushOnceMotion(CRaceMotionData::NAME_DIG);
        }

        return {};
    }

    uint16_t PhaseGameMiningBridge::GetExpectedSize() const {
        return sizeof(TPacketGCDigMotion);
    }

    bool PhaseGameMiningBridge::IsDynamicSize() const {
        return false;
    }

} // namespace Client::Bridge
