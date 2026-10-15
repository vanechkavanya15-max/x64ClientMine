#include "PlayerQuickslotController.h"

namespace Client::Player {

PlayerQuickslotController::PlayerQuickslotController(
    std::shared_ptr<Client::Gameplay::QuickslotDomain> domain,
    std::shared_ptr<IQuickslotNetwork> networkService)
    : m_domain(std::move(domain)), m_networkService(std::move(networkService))
{
}

std::expected<void, EterBase::InventoryError> PlayerQuickslotController::SetQuickslot(uint32_t slotIndex, const Client::Gameplay::QuickslotItem& item)
{
    if (!m_domain || !m_networkService) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    auto result = m_domain->SetSlot(slotIndex, item);
    if (!result) return result;

    m_networkService->SendQuickSlotAddPacket(static_cast<uint8_t>(slotIndex), item.type, item.position);

    return {};
}

std::expected<void, EterBase::InventoryError> PlayerQuickslotController::DeleteQuickslot(uint32_t slotIndex)
{
    if (!m_domain || !m_networkService) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    auto result = m_domain->ClearSlot(slotIndex);
    if (!result) return result;

    m_networkService->SendQuickSlotDelPacket(static_cast<uint8_t>(slotIndex));

    return {};
}

std::expected<void, EterBase::InventoryError> PlayerQuickslotController::SwapQuickslots(uint32_t fromIndex, uint32_t toIndex)
{
    if (!m_domain || !m_networkService) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    auto result = m_domain->SwapSlots(fromIndex, toIndex);
    if (!result) return result;

    m_networkService->SendQuickSlotMovePacket(static_cast<uint8_t>(fromIndex), static_cast<uint8_t>(toIndex));

    return {};
}

std::expected<Client::Gameplay::QuickslotItem, EterBase::InventoryError> PlayerQuickslotController::GetQuickslot(uint32_t slotIndex) const
{
    if (!m_domain) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    return m_domain->GetSlot(slotIndex);
}

void PlayerQuickslotController::ClearAll()
{
    if (m_domain)
    {
        for (uint32_t i = 0; i < Client::Gameplay::QuickslotDomain::QUICKSLOT_MAX_NUM; ++i)
        {
            if (auto slot = m_domain->GetSlot(i); slot.has_value() && !slot->IsEmpty())
            {
                 [[maybe_unused]] auto _ = DeleteQuickslot(i);
            }
        }
    }
}

} // namespace Client::Player
