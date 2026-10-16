#pragma once

#include <memory>
#include <optional>
#include <array>
#include "EterBase/StrongTypes.h"
#include "EterBase/Result.h"
#include "Client/Gameplay/AutoPotionManager.h"
#include "Client/Contracts/IPlayerDomainContract.h"
#include "Client/Core/INetworkPort.h"

namespace Client::Player {

    enum class AutoPotionControllerError : uint8_t {
        None = 0,
        SlotNotSet,
        ManagerNotSet,
        ContractNotSet,
        NetworkNotSet,
        HealingFailed,
        NetworkError
    };

    /**
     * @class PlayerAutoPotionController
     * @brief Kontroler odpowiedzialny za uzywanie autopotow (Health/Mana).
     *
     * 100% niezalezny od Windows API / DirectX.
     * Uzywa C++23 (std::expected, std::optional).
     */
    class PlayerAutoPotionController {
    public:
        PlayerAutoPotionController(
            std::shared_ptr<Client::Gameplay::AutoPotionManager> manager,
            std::shared_ptr<Client::Contracts::IPlayerDomainContract> domainContract,
            std::shared_ptr<Client::Core::INetworkPort> networkPort
        );
        ~PlayerAutoPotionController() = default;

        /**
         * @brief Ustawia slot w ekwipunku, z ktorego kontroler ma automatycznie uzywac mikstur.
         */
        void SetPotionSlot(Client::Gameplay::AutoPotionType type, EterBase::ItemSlot slot);

        /**
         * @brief Czysci ustawiony slot mikstury danego typu.
         */
        void ClearPotionSlot(Client::Gameplay::AutoPotionType type);

        /**
         * @brief Aktualizuje stan autopotow i wysyla pakiet do serwera jesli potrzeba.
         * @param currentTimeMS Aktualny czas w milisekundach.
         */
        std::expected<void, AutoPotionControllerError> Update(uint64_t currentTimeMS);

    private:
        std::shared_ptr<Client::Gameplay::AutoPotionManager> m_manager;
        std::shared_ptr<Client::Contracts::IPlayerDomainContract> m_domainContract;
        std::shared_ptr<Client::Core::INetworkPort> m_networkPort;

        std::array<std::optional<EterBase::ItemSlot>, static_cast<size_t>(Client::Gameplay::AutoPotionType::Count)> m_potionSlots;
    };

} // namespace Client::Player
