#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <optional>

namespace Bot {

struct Vector2D {
    float x{ 0.0f };
    float y{ 0.0f };
};

struct EntityTarget {
    uint32_t vid{ 0 };
    uint32_t vnum{ 0 };
    std::string name;
    Vector2D position;
    float distance{ 0.0f };
    uint32_t currentHp{ 0 };
    uint32_t maxHp{ 0 };
    bool isDead{ false };
};

struct GroundItem {
    uint32_t vid{ 0 };
    uint32_t vnum{ 0 };
    std::string name;
    Vector2D position;
    float distance{ 0.0f };
};

struct ItemSlotInfo {
    uint16_t slotIndex{ 0 };
    uint32_t vnum{ 0 };
    uint32_t count{ 0 };
    std::string name;
};

class IBotSessionApi {
public:
    virtual ~IBotSessionApi() = default;

    // Stan konta i sesji
    virtual bool IsConnected() const = 0;
    virtual bool IsInGame() const = 0;
    virtual uint32_t GetSessionId() const = 0;

    // Pozycja i ruch
    virtual Vector2D GetMyPosition() const = 0;
    virtual void MoveTo(float targetX, float targetY) = 0;
    virtual void StopMovement() = 0;

    // Walka i interakcja
    virtual void AttackVid(uint32_t vid) = 0;
    virtual void UseSkill(uint32_t skillIndex, uint32_t targetVid = 0) = 0;
    virtual void ClickNpc(uint32_t npcVid) = 0;

    // Otoczenie (skanowanie mobow i dropu)
    virtual std::vector<EntityTarget> GetNearbyMonsters(float radius) const = 0;
    virtual std::vector<EntityTarget> GetNearbyMetins(float radius) const = 0;
    virtual std::vector<GroundItem> GetNearbyGroundItems(float radius) const = 0;
    virtual void PickupItem(uint32_t itemVid) = 0;

    // Ekwipunek
    virtual std::vector<ItemSlotInfo> GetInventoryItems() const = 0;
    virtual void UseItemBySlot(uint16_t slotIndex) = 0;
    virtual void DropItemBySlot(uint16_t slotIndex, uint32_t count = 1) = 0;

    // Multi-Server Profile (0 = Official x64, 1 = Beavium x64)
    virtual void SetServerMode(uint8_t mode) = 0;
    virtual uint8_t GetServerMode() const = 0;
};

// Globalny dostep do instancji API bota
IBotSessionApi& GetApi();

} // namespace Bot

