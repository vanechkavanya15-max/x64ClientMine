#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <expected>
#include <optional>
#include <unordered_map>
#include <chrono>

namespace Client::IPC {

enum class QueryError {
    PlayerNotFound,
    InventoryEmpty,
    GeneralError
};

struct PlayerState {
    uint32_t vid{0};
    int64_t hp{0};
    int64_t maxHp{0};
    float x{0.0f};
    float y{0.0f};
    uint64_t gold{0};
    uint32_t level{0};
    std::string name;
};

enum class EntityType {
    NPC,
    Monster,
    Player,
    Unknown
};

struct EntityInfo {
    uint32_t vid{0};
    uint32_t race{0};
    float x{0.0f};
    float y{0.0f};
    int64_t hp{0};
    int64_t maxHp{0};
    EntityType type{EntityType::Unknown};
    std::string name;
    std::string state;
};

struct ItemInfo {
    uint32_t vnum{0};
    uint16_t slot{0};
    uint32_t count{0};
};

struct AuditEntry {
    uint64_t timestamp{0};
    std::string event_type;
    std::string details;
};

class HarnessWorldQueryRouter {
public:
    HarnessWorldQueryRouter() = default;
    ~HarnessWorldQueryRouter() = default;

    void SetPlayerState(const PlayerState& state) { m_playerState = state; }
    void ClearPlayerState() noexcept { m_playerState.reset(); }

    void AddEntity(const EntityInfo& e) { m_entities.push_back(e); }
    void RemoveEntity(uint32_t vid);
    void ClearEntities() noexcept { m_entities.clear(); }

    void AddItem(const ItemInfo& item) { m_items.push_back(item); }
    void RemoveItem(uint16_t slot);
    void ClearItems() noexcept { m_items.clear(); }

    void AddAuditLog(const std::string& type, const std::string& details);
    void RecordAuditLog(const std::string& type, const std::string& details) { AddAuditLog(type, details); } // Alias as requested in plan

    [[nodiscard]] std::expected<std::string, QueryError> GetState() const;
    [[nodiscard]] std::expected<std::string, QueryError> GetInventory() const;
    [[nodiscard]] std::expected<std::string, QueryError> GetNearbyEntities(float radius) const;
    [[nodiscard]] std::string WorldMegaSnapshot() const;
    [[nodiscard]] std::string AuditState() const;

    [[nodiscard]] std::string SerializeWorldSnapshot(uint32_t playerVid) const;

private:
    std::optional<PlayerState> m_playerState;
    std::vector<EntityInfo> m_entities;
    std::vector<ItemInfo> m_items;
    std::vector<AuditEntry> m_auditLogs;
};

} // namespace Client::IPC
