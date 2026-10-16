#include "PlayerAutoPotionController.h"
#include "Client/Network/InventoryCommandEncoder.h"

namespace Client::Player {

    PlayerAutoPotionController::PlayerAutoPotionController(
        std::shared_ptr<Client::Gameplay::AutoPotionManager> manager,
        std::shared_ptr<Client::Contracts::IPlayerDomainContract> domainContract,
        std::shared_ptr<Client::Core::INetworkPort> networkPort)
        : m_manager(std::move(manager)),
          m_domainContract(std::move(domainContract)),
          m_networkPort(std::move(networkPort))
    {
    }

    void PlayerAutoPotionController::SetPotionSlot(Client::Gameplay::AutoPotionType type, EterBase::ItemSlot slot) {
        if (type < Client::Gameplay::AutoPotionType::Count) {
            m_potionSlots[static_cast<size_t>(type)] = slot;
        }
    }

    void PlayerAutoPotionController::ClearPotionSlot(Client::Gameplay::AutoPotionType type) {
        if (type < Client::Gameplay::AutoPotionType::Count) {
            m_potionSlots[static_cast<size_t>(type)] = std::nullopt;
        }
    }

    std::expected<void, AutoPotionControllerError> PlayerAutoPotionController::Update(uint64_t currentTimeMS) {
        if (!m_manager) return std::unexpected(AutoPotionControllerError::ManagerNotSet);
        if (!m_domainContract) return std::unexpected(AutoPotionControllerError::ContractNotSet);
        if (!m_networkPort) return std::unexpected(AutoPotionControllerError::NetworkNotSet);

        // Process Health
        if (m_manager->IsActive(Client::Gameplay::AutoPotionType::Health)) {
            const auto& hpSlot = m_potionSlots[static_cast<size_t>(Client::Gameplay::AutoPotionType::Health)];
            if (hpSlot.has_value()) {
                uint32_t currentHp = static_cast<uint32_t>(m_domainContract->GetCurrentHP());
                uint32_t maxHp = static_cast<uint32_t>(m_domainContract->GetMaxHP());
                
                auto healResult = m_manager->Process(Client::Gameplay::AutoPotionType::Health, currentHp, maxHp, currentTimeMS);
                if (healResult.has_value() && healResult.value() > 0) {
                    auto encodedPacketResult = Client::Network::InventoryCommandEncoder::EncodeUseItem(hpSlot.value());
                    if (encodedPacketResult.has_value() && !encodedPacketResult.value().empty()) {
                        uint8_t opcode = encodedPacketResult.value()[0];
                        auto sendResult = m_networkPort->SendRaw(opcode, std::span<const uint8_t>(encodedPacketResult.value()));
                        if (!sendResult.has_value()) {
                             return std::unexpected(AutoPotionControllerError::NetworkError);
                        }
                    }
                }
            }
        }

        // Process Mana
        if (m_manager->IsActive(Client::Gameplay::AutoPotionType::Mana)) {
            const auto& mpSlot = m_potionSlots[static_cast<size_t>(Client::Gameplay::AutoPotionType::Mana)];
            if (mpSlot.has_value()) {
                uint32_t currentSp = static_cast<uint32_t>(m_domainContract->GetCurrentSP());
                uint32_t maxSp = static_cast<uint32_t>(m_domainContract->GetMaxSP());
                
                auto manaResult = m_manager->Process(Client::Gameplay::AutoPotionType::Mana, currentSp, maxSp, currentTimeMS);
                if (manaResult.has_value() && manaResult.value() > 0) {
                    auto encodedPacketResult = Client::Network::InventoryCommandEncoder::EncodeUseItem(mpSlot.value());
                    if (encodedPacketResult.has_value() && !encodedPacketResult.value().empty()) {
                        uint8_t opcode = encodedPacketResult.value()[0];
                        auto sendResult = m_networkPort->SendRaw(opcode, std::span<const uint8_t>(encodedPacketResult.value()));
                        if (!sendResult.has_value()) {
                             return std::unexpected(AutoPotionControllerError::NetworkError);
                        }
                    }
                }
            }
        }

        return {};
    }

} // namespace Client::Player
