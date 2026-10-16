#pragma once

#include <cstdint>
#include <vector>
#include <unordered_map>
#include <memory>
#include <string_view>
#include <array>
#include <span>

#include "EterBase/Result.h"
#include "Client/Core/StrongTypes.h"

namespace Client::Bridge {

class IPythonRuntime;

// ============================================================================
// Core State Structures
// ============================================================================

enum class PointType : uint8_t {
    None = 0,
    HP,
    SP,
    STAMINA,
    MAX_HP,
    MAX_SP,
    MAX_STAMINA,
    EXP,
    NEXT_EXP,
    GOLD
};

enum class ActorActionState : uint8_t {
    Idle = 0,
    Moving,
    Attacking,
    Stunned,
    Dead
};

struct PointChangeData {
    uint8_t type{0};
    int64_t value{0};
    int64_t amount{0};
};

struct CharacterStateData {
    Client::Core::EntityVid vid;
    ActorActionState actionState{ActorActionState::Idle};
    uint8_t alignment{0};
    uint8_t empire{0};
    uint8_t guildId{0};
    uint8_t pkMode{0};
};

struct CharacterPositionData {
    Client::Core::EntityVid vid;
    Client::Core::MapCoords position;
    float rotation{0.0f};
    uint32_t destTime{0};
};

struct ActorState {
    Client::Core::EntityVid vid;
    ActorActionState actionState{ActorActionState::Idle};
    Client::Core::MapCoords currentPosition;
    float currentRotation{0.0f};
    int64_t currentHP{100};
    int64_t currentSP{100};
    int64_t maxHP{100};
    int64_t maxSP{100};
    bool isStunned{false};
    bool isDead{false};
};

// ============================================================================
// Main Bridge Class
// ============================================================================

class PhaseGameCharacterStateBridge final {
public:
    explicit PhaseGameCharacterStateBridge(std::shared_ptr<IPythonRuntime> pyRuntime = nullptr);
    ~PhaseGameCharacterStateBridge() = default;

    PhaseGameCharacterStateBridge(const PhaseGameCharacterStateBridge&) = delete;
    PhaseGameCharacterStateBridge& operator=(const PhaseGameCharacterStateBridge&) = delete;
    PhaseGameCharacterStateBridge(PhaseGameCharacterStateBridge&&) noexcept = default;
    PhaseGameCharacterStateBridge& operator=(PhaseGameCharacterStateBridge&&) noexcept = default;

    // Set Python Runtime dependency
    void SetPythonRuntime(std::shared_ptr<IPythonRuntime> pyRuntime) noexcept;

    // Incoming Packet Handlers (from Network -> Client Domain)
    EterBase::PacketResult<void> RecvPointChange(Client::Core::EntityVid vid, PointType type, int64_t value, int64_t amount);
    EterBase::PacketResult<void> RecvStun(Client::Core::EntityVid vid);
    EterBase::PacketResult<void> RecvDead(Client::Core::EntityVid vid);
    EterBase::PacketResult<void> RecvMotion(Client::Core::EntityVid vid, const Client::Core::MapCoords& destPosition, float destRotation, uint32_t timestamp);

    // Outgoing Packet Generators (from Client Domain -> Network)
    EterBase::PacketDispatchResult<std::vector<uint8_t>> SendCharacterStatePacket(const CharacterStateData& stateData);
    EterBase::PacketDispatchResult<std::vector<uint8_t>> SendCharacterPosition(const CharacterPositionData& posData);

    // Legacy test hook (preserving tests/test_c26_phase_game_char_state.cpp backwards compatibility)
    void OnPointChange(uint8_t type, int64_t value, int64_t amount);
    [[nodiscard]] const std::vector<PointChangeData>& GetHistory() const noexcept { return m_history; }

    // State Query Accessors
    [[nodiscard]] const ActorState* GetActorState(Client::Core::EntityVid vid) const noexcept;
    
    // Management
    void ClearState() noexcept;

private:
    ActorState& GetOrCreateActorState(Client::Core::EntityVid vid);
    void SerializeUint8(std::vector<uint8_t>& buffer, uint8_t value) const;
    void SerializeUint32(std::vector<uint8_t>& buffer, uint32_t value) const;
    void SerializeFloat(std::vector<uint8_t>& buffer, float value) const;

    std::shared_ptr<IPythonRuntime> m_pyRuntime;
    std::unordered_map<Client::Core::EntityVid, ActorState> m_actorStates;
    std::vector<PointChangeData> m_history; // Kept for legacy OnPointChange compatibility
};

} // namespace Client::Bridge
