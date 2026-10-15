#pragma once

#include <memory>
#include <optional>
#include <cstdint>
#include <vector>
#include <string>

#include "Client/Contracts/IInventoryDomainContract.h"
#include "Client/Core/ExpectedResult.h"

namespace Client::Player {

    enum class InventoryError {
        DomainContractNull,
        InvalidWindowType,
        ItemNotFound,
        ActionFailed
    };

    struct InventoryItemView {
        uint8_t windowType;
        uint16_t cellIndex;
        uint32_t vnum;
        uint32_t count;
        bool isEquipped;
        std::string name;
    };

    class PlayerInventoryController {
    public:
        explicit PlayerInventoryController(std::shared_ptr<Client::Contracts::IInventoryDomainContract> domainContract) noexcept;

        [[nodiscard]] Client::Core::ExpectedResult<InventoryItemView, InventoryError> GetItemInfo(uint8_t windowType, uint16_t cellIndex) const noexcept;
        [[nodiscard]] Client::Core::ExpectedResult<std::vector<InventoryItemView>, InventoryError> GetAllItems(uint8_t windowType) const noexcept;
        
        [[nodiscard]] Client::Core::ExpectedResult<bool, InventoryError> IsCellEmpty(uint8_t windowType, uint16_t cellIndex) const noexcept;
        [[nodiscard]] Client::Core::ExpectedResult<size_t, InventoryError> GetFreeSlotCount(uint8_t windowType) const noexcept;

        Client::Core::ExpectedResult<void, InventoryError> RequestUseItem(uint8_t windowType, uint16_t cellIndex) noexcept;
        Client::Core::ExpectedResult<void, InventoryError> RequestMoveItem(uint8_t srcWindowType, uint16_t srcCellIndex, uint8_t dstWindowType, uint16_t dstCellIndex, uint32_t count) noexcept;
        Client::Core::ExpectedResult<void, InventoryError> RequestDropItem(uint8_t windowType, uint16_t cellIndex, uint32_t count) noexcept;

    private:
        std::shared_ptr<Client::Contracts::IInventoryDomainContract> domainContract_;

        [[nodiscard]] bool IsValidWindowType(uint8_t windowType) const noexcept;
    };

} // namespace Client::Player
