#pragma once

#include <optional>
#include <expected>
#include <cstdint>
#include <utility>
#include "Client/Core/StrongTypes.h"
#include "Client/Contracts/DomainDTOs.h"

namespace Client::Gameplay {

    enum class InventorySlotError {
        AlreadyOwned,
        NotOwned
    };

    class InventorySlotOwner {
    public:
        InventorySlotOwner() = default;

        // Disable copy
        InventorySlotOwner(const InventorySlotOwner&) = delete;
        InventorySlotOwner& operator=(const InventorySlotOwner&) = delete;

        // Enable move
        InventorySlotOwner(InventorySlotOwner&& other) noexcept : m_slot(std::move(other.m_slot)) {
            other.m_slot.reset();
        }
        InventorySlotOwner& operator=(InventorySlotOwner&& other) noexcept {
            if (this != &other) {
                m_slot = std::move(other.m_slot);
                other.m_slot.reset();
            }
            return *this;
        }

        std::expected<void, InventorySlotError> Assign(Client::Contracts::ItemSlotDTO item) {
            if (m_slot.has_value()) {
                return std::unexpected(InventorySlotError::AlreadyOwned);
            }
            m_slot = item;
            return {};
        }

        std::expected<Client::Contracts::ItemSlotDTO, InventorySlotError> Release() {
            if (!m_slot.has_value()) {
                return std::unexpected(InventorySlotError::NotOwned);
            }
            auto item = *m_slot;
            m_slot.reset();
            return item;
        }

        [[nodiscard]] bool HasItem() const {
            return m_slot.has_value();
        }

        [[nodiscard]] const Client::Contracts::ItemSlotDTO* GetItem() const {
            return m_slot.has_value() ? &(*m_slot) : nullptr;
        }

    private:
        std::optional<Client::Contracts::ItemSlotDTO> m_slot;
    };

}
