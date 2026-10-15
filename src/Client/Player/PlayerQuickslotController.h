#pragma once

#include <memory>
#include <expected>
#include "../../EterBase/Result.h"
#include "../Gameplay/QuickslotDomain.h"
#include "IQuickslotNetwork.h"

namespace Client::Player {

class PlayerQuickslotController {
public:
    PlayerQuickslotController(std::shared_ptr<Client::Gameplay::QuickslotDomain> domain,
                              std::shared_ptr<IQuickslotNetwork> networkService);
    ~PlayerQuickslotController() = default;

    [[nodiscard]] std::expected<void, EterBase::InventoryError> SetQuickslot(uint32_t slotIndex, const Client::Gameplay::QuickslotItem& item);
    [[nodiscard]] std::expected<void, EterBase::InventoryError> DeleteQuickslot(uint32_t slotIndex);
    [[nodiscard]] std::expected<void, EterBase::InventoryError> SwapQuickslots(uint32_t fromIndex, uint32_t toIndex);
    [[nodiscard]] std::expected<Client::Gameplay::QuickslotItem, EterBase::InventoryError> GetQuickslot(uint32_t slotIndex) const;
    void ClearAll();

private:
    std::shared_ptr<Client::Gameplay::QuickslotDomain> m_domain;
    std::shared_ptr<IQuickslotNetwork> m_networkService;
};

} // namespace Client::Player
