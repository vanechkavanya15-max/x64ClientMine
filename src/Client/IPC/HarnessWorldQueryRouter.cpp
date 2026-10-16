#include "HarnessWorldQueryRouter.h"
#include <sstream>
#include <cmath>
#include <algorithm>
#include <chrono>
#include <rapidjson/document.h>
#include <rapidjson/writer.h>
#include <rapidjson/stringbuffer.h>

namespace Client::IPC {

static std::string EntityTypeToString(EntityType type) {
    switch (type) {
        case EntityType::NPC: return "NPC";
        case EntityType::Monster: return "Monster";
        case EntityType::Player: return "Player";
        default: return "Unknown";
    }
}

void HarnessWorldQueryRouter::RemoveEntity(uint32_t vid) {
    std::erase_if(m_entities, [vid](const EntityInfo& e) { return e.vid == vid; });
}

void HarnessWorldQueryRouter::RemoveItem(uint16_t slot) {
    std::erase_if(m_items, [slot](const ItemInfo& i) { return i.slot == slot; });
}

void HarnessWorldQueryRouter::AddAuditLog(const std::string& type, const std::string& details) {
    auto now = std::chrono::system_clock::now();
    uint64_t ms = std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();
    m_auditLogs.push_back({ms, type, details});
}

std::expected<std::string, QueryError> HarnessWorldQueryRouter::GetState() const {
    if (!m_playerState) {
        return std::unexpected(QueryError::PlayerNotFound);
    }

    rapidjson::StringBuffer s;
    rapidjson::Writer<rapidjson::StringBuffer> writer(s);

    writer.StartObject();
    writer.Key("vid"); writer.Uint(m_playerState->vid);
    writer.Key("name"); writer.String(m_playerState->name.c_str());
    writer.Key("hp"); writer.Int64(m_playerState->hp);
    writer.Key("max_hp"); writer.Int64(m_playerState->maxHp);
    writer.Key("x"); writer.Double(m_playerState->x);
    writer.Key("y"); writer.Double(m_playerState->y);
    writer.Key("gold"); writer.Uint64(m_playerState->gold);
    writer.Key("level"); writer.Uint(m_playerState->level);
    writer.EndObject();

    return s.GetString();
}

std::expected<std::string, QueryError> HarnessWorldQueryRouter::GetInventory() const {
    if (m_items.empty()) {
        return std::unexpected(QueryError::InventoryEmpty);
    }

    rapidjson::StringBuffer s;
    rapidjson::Writer<rapidjson::StringBuffer> writer(s);

    writer.StartArray();
    for (const auto& item : m_items) {
        writer.StartObject();
        writer.Key("vnum"); writer.Uint(item.vnum);
        writer.Key("slot"); writer.Uint(item.slot);
        writer.Key("count"); writer.Uint(item.count);
        writer.EndObject();
    }
    writer.EndArray();

    return s.GetString();
}

std::expected<std::string, QueryError> HarnessWorldQueryRouter::GetNearbyEntities(float radius) const {
    if (!m_playerState) {
        return std::unexpected(QueryError::PlayerNotFound);
    }

    struct DistEntity {
        float distance;
        const EntityInfo* entity;
    };
    std::vector<DistEntity> nearby;

    float px = m_playerState->x;
    float py = m_playerState->y;

    for (const auto& e : m_entities) {
        float dx = e.x - px;
        float dy = e.y - py;
        float dist = std::sqrt(dx * dx + dy * dy);
        if (dist <= radius) {
            nearby.push_back({dist, &e});
        }
    }

    std::sort(nearby.begin(), nearby.end(), [](const DistEntity& a, const DistEntity& b) {
        return a.distance < b.distance;
    });

    rapidjson::StringBuffer s;
    rapidjson::Writer<rapidjson::StringBuffer> writer(s);

    writer.StartArray();
    for (const auto& de : nearby) {
        writer.StartObject();
        writer.Key("vid"); writer.Uint(de.entity->vid);
        writer.Key("race"); writer.Uint(de.entity->race);
        writer.Key("type"); writer.String(EntityTypeToString(de.entity->type).c_str());
        writer.Key("name"); writer.String(de.entity->name.c_str());
        writer.Key("hp"); writer.Int64(de.entity->hp);
        writer.Key("max_hp"); writer.Int64(de.entity->maxHp);
        writer.Key("x"); writer.Double(de.entity->x);
        writer.Key("y"); writer.Double(de.entity->y);
        writer.Key("state"); writer.String(de.entity->state.c_str());
        writer.Key("distance"); writer.Double(de.distance);
        writer.EndObject();
    }
    writer.EndArray();

    return s.GetString();
}

std::string HarnessWorldQueryRouter::WorldMegaSnapshot() const {
    rapidjson::StringBuffer s;
    rapidjson::Writer<rapidjson::StringBuffer> writer(s);

    writer.StartObject();

    writer.Key("player");
    if (m_playerState) {
        writer.StartObject();
        writer.Key("vid"); writer.Uint(m_playerState->vid);
        writer.Key("name"); writer.String(m_playerState->name.c_str());
        writer.Key("hp"); writer.Int64(m_playerState->hp);
        writer.Key("max_hp"); writer.Int64(m_playerState->maxHp);
        writer.Key("x"); writer.Double(m_playerState->x);
        writer.Key("y"); writer.Double(m_playerState->y);
        writer.Key("gold"); writer.Uint64(m_playerState->gold);
        writer.Key("level"); writer.Uint(m_playerState->level);
        writer.EndObject();
    } else {
        writer.Null();
    }

    writer.Key("entities");
    writer.StartArray();
    for (const auto& e : m_entities) {
        writer.StartObject();
        writer.Key("vid"); writer.Uint(e.vid);
        writer.Key("race"); writer.Uint(e.race);
        writer.Key("type"); writer.String(EntityTypeToString(e.type).c_str());
        writer.Key("name"); writer.String(e.name.c_str());
        writer.Key("hp"); writer.Int64(e.hp);
        writer.Key("max_hp"); writer.Int64(e.maxHp);
        writer.Key("x"); writer.Double(e.x);
        writer.Key("y"); writer.Double(e.y);
        writer.Key("state"); writer.String(e.state.c_str());
        writer.EndObject();
    }
    writer.EndArray();

    writer.Key("inventory");
    writer.StartArray();
    for (const auto& item : m_items) {
        writer.StartObject();
        writer.Key("vnum"); writer.Uint(item.vnum);
        writer.Key("slot"); writer.Uint(item.slot);
        writer.Key("count"); writer.Uint(item.count);
        writer.EndObject();
    }
    writer.EndArray();

    writer.EndObject();

    return s.GetString();
}

std::string HarnessWorldQueryRouter::AuditState() const {
    rapidjson::StringBuffer s;
    rapidjson::Writer<rapidjson::StringBuffer> writer(s);

    writer.StartArray();
    for (const auto& log : m_auditLogs) {
        writer.StartObject();
        writer.Key("timestamp"); writer.Uint64(log.timestamp);
        writer.Key("event_type"); writer.String(log.event_type.c_str());
        writer.Key("details"); writer.String(log.details.c_str());
        writer.EndObject();
    }
    writer.EndArray();

    return s.GetString();
}

std::string HarnessWorldQueryRouter::SerializeWorldSnapshot(uint32_t playerVid) const {
    std::ostringstream ss;
    ss << "{\"player_vid\":" << playerVid << ",\"entity_count\":" << m_entities.size() << ",\"entities\":[";
    for (size_t i = 0; i < m_entities.size(); ++i) {
        const auto& e = m_entities[i];
        ss << "{\"vid\":" << e.vid << ",\"race\":" << e.race << ",\"x\":" << e.x << ",\"y\":" << e.y << "}";
        if (i + 1 < m_entities.size()) ss << ",";
    }
    ss << "]}";
    return ss.str();
}

} // namespace Client::IPC
