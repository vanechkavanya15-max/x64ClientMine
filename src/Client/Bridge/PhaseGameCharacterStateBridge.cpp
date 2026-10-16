#include "PhaseGameCharacterStateBridge.h"
#include "IPythonRuntime.h"
#include <cstring>

namespace Client::Bridge {

PhaseGameCharacterStateBridge::PhaseGameCharacterStateBridge(std::shared_ptr<IPythonRuntime> pyRuntime)
    : m_pyRuntime(std::move(pyRuntime)) {
}

void PhaseGameCharacterStateBridge::SetPythonRuntime(std::shared_ptr<IPythonRuntime> pyRuntime) noexcept {
    m_pyRuntime = std::move(pyRuntime);
}

// ============================================================================
// Incoming Packet Handlers
// ============================================================================

EterBase::PacketResult<void> PhaseGameCharacterStateBridge::RecvPointChange(
    Client::Core::EntityVid vid, PointType type, int64_t value, int64_t amount)
{
    // Local state updates
    ActorState& state = GetOrCreateActorState(vid);

    switch (type) {
        case PointType::HP:
            state.currentHP = value;
            if (state.currentHP <= 0 && state.maxHP > 0) {
                state.isDead = true;
                state.actionState = ActorActionState::Dead;
                state.currentHP = 0;
            } else if (state.currentHP > state.maxHP) {
                state.currentHP = state.maxHP;
            }
            break;
        case PointType::MAX_HP:
            state.maxHP = value;
            if (state.currentHP > state.maxHP) {
                state.currentHP = state.maxHP;
            }
            break;
        case PointType::SP:
            state.currentSP = value;
            if (state.currentSP < 0) state.currentSP = 0;
            if (state.currentSP > state.maxSP) state.currentSP = state.maxSP;
            break;
        case PointType::MAX_SP:
            state.maxSP = value;
            if (state.currentSP > state.maxSP) {
                state.currentSP = state.maxSP;
            }
            break;
        case PointType::None:
            return EterBase::MakeError(EterBase::PacketError::MalformedPayload);
        default:
            // Other points (EXP, GOLD, etc.) just update without strict local clamp constraints for now.
            break;
    }

    // Legacy handler update
    OnPointChange(static_cast<uint8_t>(type), value, amount);

    // Python Integration
    if (m_pyRuntime) {
        auto res = m_pyRuntime->Invoke("player", "OnPointChange", vid.get(), static_cast<uint32_t>(type), value, amount);
        if (!res.has_value()) {
            // Non-critical, python invocation failures should not crash packet loop but can be logged.
        }
    }

    return {};
}

EterBase::PacketResult<void> PhaseGameCharacterStateBridge::RecvStun(Client::Core::EntityVid vid) {
    ActorState& state = GetOrCreateActorState(vid);
    
    if (state.isDead) {
        return EterBase::MakeError(EterBase::PacketError::SequenceMismatch); // Dead entities cannot be stunned
    }

    state.isStunned = true;
    state.actionState = ActorActionState::Stunned;

    if (m_pyRuntime) {
        m_pyRuntime->Invoke("player", "SetStatus", vid.get(), "STUN", 1);
    }

    return {};
}

EterBase::PacketResult<void> PhaseGameCharacterStateBridge::RecvDead(Client::Core::EntityVid vid) {
    ActorState& state = GetOrCreateActorState(vid);
    
    if (state.isDead) {
        // Repeated death packet, potentially benign, but we shouldn't act again.
        return {}; 
    }

    state.isDead = true;
    state.isStunned = false; // Death removes stun
    state.actionState = ActorActionState::Dead;
    state.currentHP = 0;

    if (m_pyRuntime) {
        m_pyRuntime->Invoke("player", "SetStatus", vid.get(), "DEAD", 1);
    }

    return {};
}

EterBase::PacketResult<void> PhaseGameCharacterStateBridge::RecvMotion(
    Client::Core::EntityVid vid, const Client::Core::MapCoords& destPosition, float destRotation, uint32_t timestamp)
{
    ActorState& state = GetOrCreateActorState(vid);

    // State machine check: cannot move if dead or stunned
    if (state.isDead || state.isStunned) {
        return EterBase::MakeError(EterBase::PacketError::SequenceMismatch);
    }

    state.currentPosition = destPosition;
    state.currentRotation = destRotation;
    state.actionState = ActorActionState::Moving;

    if (m_pyRuntime) {
        m_pyRuntime->Invoke("player", "SetTargetPosition", vid.get(), destPosition.x, destPosition.y, destPosition.z, destRotation, timestamp);
    }

    return {};
}

// ============================================================================
// Outgoing Packet Generators
// ============================================================================

EterBase::PacketDispatchResult<std::vector<uint8_t>> PhaseGameCharacterStateBridge::SendCharacterStatePacket(
    const CharacterStateData& stateData)
{
    if (!stateData.vid) {
         return EterBase::MakeError(EterBase::PacketDispatchError::EncodingFailed);
    }

    std::vector<uint8_t> buffer;
    buffer.reserve(32);

    // Example serialization format: 
    // Header(1 byte) + VID(4) + Action(1) + Alignment(1) + Empire(1) + Guild(1) + PKMode(1)
    
    constexpr uint8_t HEADER_CHARACTER_STATE = 0xA1;
    SerializeUint8(buffer, HEADER_CHARACTER_STATE);
    SerializeUint32(buffer, stateData.vid.get());
    SerializeUint8(buffer, static_cast<uint8_t>(stateData.actionState));
    SerializeUint8(buffer, stateData.alignment);
    SerializeUint8(buffer, stateData.empire);
    SerializeUint8(buffer, stateData.guildId);
    SerializeUint8(buffer, stateData.pkMode);

    return buffer;
}

EterBase::PacketDispatchResult<std::vector<uint8_t>> PhaseGameCharacterStateBridge::SendCharacterPosition(
    const CharacterPositionData& posData)
{
    if (!posData.vid) {
         return EterBase::MakeError(EterBase::PacketDispatchError::EncodingFailed);
    }

    std::vector<uint8_t> buffer;
    buffer.reserve(32);

    // Example serialization format: 
    // Header(1 byte) + VID(4) + X(4) + Y(4) + Z(4) + Rot(4) + DestTime(4)

    constexpr uint8_t HEADER_CHARACTER_POSITION = 0xA2;
    SerializeUint8(buffer, HEADER_CHARACTER_POSITION);
    SerializeUint32(buffer, posData.vid.get());
    SerializeFloat(buffer, posData.position.x);
    SerializeFloat(buffer, posData.position.y);
    SerializeFloat(buffer, posData.position.z);
    SerializeFloat(buffer, posData.rotation);
    SerializeUint32(buffer, posData.destTime);

    return buffer;
}

// ============================================================================
// Internal Helpers & Legacy Methods
// ============================================================================

void PhaseGameCharacterStateBridge::OnPointChange(uint8_t type, int64_t value, int64_t amount) {
    m_history.push_back({type, value, amount});
}

const ActorState* PhaseGameCharacterStateBridge::GetActorState(Client::Core::EntityVid vid) const noexcept {
    auto it = m_actorStates.find(vid);
    if (it != m_actorStates.end()) {
        return &it->second;
    }
    return nullptr;
}

void PhaseGameCharacterStateBridge::ClearState() noexcept {
    m_actorStates.clear();
    m_history.clear();
}

ActorState& PhaseGameCharacterStateBridge::GetOrCreateActorState(Client::Core::EntityVid vid) {
    auto [it, inserted] = m_actorStates.try_emplace(vid);
    if (inserted) {
        it->second.vid = vid;
    }
    return it->second;
}

void PhaseGameCharacterStateBridge::SerializeUint8(std::vector<uint8_t>& buffer, uint8_t value) const {
    buffer.push_back(value);
}

void PhaseGameCharacterStateBridge::SerializeUint32(std::vector<uint8_t>& buffer, uint32_t value) const {
    const uint8_t* p = reinterpret_cast<const uint8_t*>(&value);
    buffer.insert(buffer.end(), p, p + sizeof(uint32_t));
}

void PhaseGameCharacterStateBridge::SerializeFloat(std::vector<uint8_t>& buffer, float value) const {
    const uint8_t* p = reinterpret_cast<const uint8_t*>(&value);
    buffer.insert(buffer.end(), p, p + sizeof(float));
}

} // namespace Client::Bridge
