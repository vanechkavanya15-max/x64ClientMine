#include "PlayerQuickslotController.h"

namespace Client::Player {

PlayerQuickslotController::PlayerQuickslotController(
    std::shared_ptr<Client::Gameplay::QuickslotDomain> domain,
    std::shared_ptr<IQuickslotNetwork> networkService)
    : m_domain(std::move(domain)), m_networkService(std::move(networkService))
{
}

std::expected<void, EterBase::InventoryError> PlayerQuickslotController::SetQuickslot(Client::Core::SlotIndex slotIndex, const Client::Gameplay::QuickslotItem& item)
{
    if (!m_domain || !m_networkService) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    auto result = m_domain->SetSlot(slotIndex, item);
    if (!result) return result;

    m_networkService->SendQuickSlotAddPacket(static_cast<uint8_t>(slotIndex.get()), item.type, item.position);

    return {};
}

std::expected<void, EterBase::InventoryError> PlayerQuickslotController::DeleteQuickslot(Client::Core::SlotIndex slotIndex)
{
    if (!m_domain || !m_networkService) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    auto result = m_domain->ClearSlot(slotIndex);
    if (!result) return result;

    m_networkService->SendQuickSlotDelPacket(static_cast<uint8_t>(slotIndex.get()));

    return {};
}

std::expected<void, EterBase::InventoryError> PlayerQuickslotController::SwapQuickslots(Client::Core::SlotIndex fromIndex, Client::Core::SlotIndex toIndex)
{
    if (!m_domain || !m_networkService) return std::unexpected(EterBase::InventoryError::SlotOutOfRange);

    auto result = m_domain->SwapSlots(fromIndex, toIndex);
    if (!result) return result;

    m_networkService->SendQuickSlotMovePacket(static_cast<uint8_t>(fromIndex.get()), static_cast<uint8_t>(toIndex.get()));

    return {};
}

std::expected<Client::Gameplay::QuickslotItem, EterBase::InventoryError> PlayerQuickslotController::GetQuickslot(Client::Core::SlotIndex slotIndex) const
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
            if (auto slot = m_domain->GetSlot(Client::Core::SlotIndex(static_cast<uint16_t>(i))); slot.has_value() && !slot->IsEmpty())
            {
                 [[maybe_unused]] auto _ = DeleteQuickslot(Client::Core::SlotIndex(static_cast<uint16_t>(i)));
            }
        }
    }
}

} // namespace Client::Player
