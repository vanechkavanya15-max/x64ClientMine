#include "PlayerInventoryController.h"

namespace Client::Player {

    PlayerInventoryController::PlayerInventoryController(std::shared_ptr<Client::Contracts::IInventoryDomainContract> domainContract) noexcept
        : domainContract_(std::move(domainContract)) {
    }

    bool PlayerInventoryController::IsValidWindowType(uint8_t windowType) const noexcept {
        return windowType == 1 || windowType == 2 || windowType == 10 || windowType == 11;
    }

    Client::Core::ExpectedResult<InventoryItemView, InventoryError> PlayerInventoryController::GetItemInfo(uint8_t windowType, uint16_t cellIndex) const noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(windowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        Client::Contracts::ItemCellAddress addr{windowType, cellIndex};
        auto optItem = domainContract_->GetItem(addr);
        if (!optItem) {
            return Client::Core::MakeUnexpected(InventoryError::ItemNotFound);
        }

        InventoryItemView view{
            .windowType = optItem->pos.windowType,
            .cellIndex = optItem->pos.cellIndex,
            .vnum = optItem->vnum,
            .count = optItem->count,
            .isEquipped = optItem->isEquipped,
            .name = optItem->localizedName
        };

        return view;
    }

    Client::Core::ExpectedResult<std::vector<InventoryItemView>, InventoryError> PlayerInventoryController::GetAllItems(uint8_t windowType) const noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(windowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        auto items = domainContract_->GetAllItems(windowType);
        std::vector<InventoryItemView> views;
        views.reserve(items.size());
        for (const auto& item : items) {
            views.push_back({
                .windowType = item.pos.windowType,
                .cellIndex = item.pos.cellIndex,
                .vnum = item.vnum,
                .count = item.count,
                .isEquipped = item.isEquipped,
                .name = item.localizedName
            });
        }

        return views;
    }

    Client::Core::ExpectedResult<bool, InventoryError> PlayerInventoryController::IsCellEmpty(uint8_t windowType, uint16_t cellIndex) const noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(windowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        Client::Contracts::ItemCellAddress addr{windowType, cellIndex};
        return domainContract_->IsCellEmpty(addr);
    }

    Client::Core::ExpectedResult<size_t, InventoryError> PlayerInventoryController::GetFreeSlotCount(uint8_t windowType) const noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(windowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        return domainContract_->GetFreeSlotCount(windowType);
    }

    Client::Core::ExpectedResult<void, InventoryError> PlayerInventoryController::RequestUseItem(uint8_t windowType, uint16_t cellIndex) noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(windowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        Client::Contracts::ItemCellAddress addr{windowType, cellIndex};
        if (domainContract_->RequestUseItem(addr)) {
            return {};
        } else {
            return Client::Core::MakeUnexpected(InventoryError::ActionFailed);
        }
    }

    Client::Core::ExpectedResult<void, InventoryError> PlayerInventoryController::RequestMoveItem(uint8_t srcWindowType, uint16_t srcCellIndex, uint8_t dstWindowType, uint16_t dstCellIndex, uint32_t count) noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(srcWindowType) || !IsValidWindowType(dstWindowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        Client::Contracts::ItemCellAddress srcAddr{srcWindowType, srcCellIndex};
        Client::Contracts::ItemCellAddress dstAddr{dstWindowType, dstCellIndex};

        if (domainContract_->RequestMoveItem(srcAddr, dstAddr, count)) {
            return {};
        } else {
            return Client::Core::MakeUnexpected(InventoryError::ActionFailed);
        }
    }

    Client::Core::ExpectedResult<void, InventoryError> PlayerInventoryController::RequestDropItem(uint8_t windowType, uint16_t cellIndex, uint32_t count) noexcept {
        if (!domainContract_) {
            return Client::Core::MakeUnexpected(InventoryError::DomainContractNull);
        }
        if (!IsValidWindowType(windowType)) {
            return Client::Core::MakeUnexpected(InventoryError::InvalidWindowType);
        }

        Client::Contracts::ItemCellAddress addr{windowType, cellIndex};
        if (domainContract_->RequestDropItem(addr, count)) {
            return {};
        } else {
            return Client::Core::MakeUnexpected(InventoryError::ActionFailed);
        }
    }

} // namespace Client::Player
