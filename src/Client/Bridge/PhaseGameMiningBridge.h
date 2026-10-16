#pragma once

#include <cstdint>
#include <span>
#include "Client/Network/ModernPacketDispatcher.h"
#include "EterBase/Result.h"

class IAbstractCharacterManager;

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obslugę pakietów kopania rud (mining) od głównej fazy gry.
     * ZERO-CONFLICT: Niezależne rozszerzenie z obslugą błędów oparte na SRP.
     */
    class PhaseGameMiningBridge : public Client::Network::IPacketHandler {
    public:
        explicit PhaseGameMiningBridge(IAbstractCharacterManager& characterManager) noexcept
            : m_characterManager(characterManager) {}

        [[nodiscard]] EterBase::PacketResult<void> Handle(std::span<const uint8_t> payload) override;
        [[nodiscard]] uint16_t GetExpectedSize() const override;
        [[nodiscard]] bool IsDynamicSize() const override;

    private:
        IAbstractCharacterManager& m_characterManager;
    };

} // namespace Client::Bridge
