#pragma once

#include <unordered_map>
#include <expected>
#include <cstdint>
#include "Client/Core/StrongTypes.h"

/**
 * @file PlayerEquipmentController.h
 * @brief Kontroler zarzadzajacy ekwipunkiem gracza w izolowanym module domenowym.
 * 
 * Klasa 100% niezalezna od API systemowych (Windows/DirectX).
 * Zapewnia operacje zakladania i zdejmowania przedmiotow uzywajac typow C++23.
 */

namespace Client::Player {

/// @brief Typy slotow ekwipunku gracza.
enum class EquipmentSlotType : uint8_t {
    Weapon = 0,
    Armor,
    Helmet,
    Shield,
    Jewelry
};

/// @brief Kody bledow domenowych operacji na ekwipunku.
enum class EquipmentError : uint8_t {
    None = 0,
    SlotOccupied,
    SlotEmpty
};

/// @brief Kontroler wyposazenia gracza mapujacy sloty na VNUM przedmiotow.
class PlayerEquipmentController {
public:
    PlayerEquipmentController() = default;
    ~PlayerEquipmentController() = default;

    /// @brief Zaklada przedmiot w wybranym slocie.
    /// @param slot Typ slotu ekwipunku.
    /// @param vnum VNUM przedmiotu zdefiniowany przez StrongType.
    /// @return std::expected z błędem jeśli slot jest już zajęty.
    std::expected<void, EquipmentError> EquipItem(EquipmentSlotType slot, Client::Core::ItemVnum vnum);

    /// @brief Zdejmuje przedmiot z wybranego slotu.
    /// @param slot Typ slotu ekwipunku.
    /// @return std::expected z błędem jeśli slot jest pusty.
    std::expected<void, EquipmentError> UnequipItem(EquipmentSlotType slot);

    /// @brief Zwraca przedmiot znajdujacy sie w danym slocie.
    /// @param slot Typ slotu ekwipunku.
    /// @return std::expected z VNUM przedmiotu, albo błąd jeśli slot jest pusty.
    std::expected<Client::Core::ItemVnum, EquipmentError> GetItemInSlot(EquipmentSlotType slot) const;

    /// @brief Sprawdza czy dany slot jest pusty.
    /// @param slot Typ slotu ekwipunku.
    /// @return true jezeli slot jest pusty, false jezeli jest w nim przedmiot.
    bool IsSlotEmpty(EquipmentSlotType slot) const;

    /// @brief Cysci wszystkie sloty ekwipunku.
    void ClearAll();

private:
    std::unordered_map<EquipmentSlotType, Client::Core::ItemVnum> equippedItems_;
};

} // namespace Client::Player
