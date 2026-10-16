#pragma once
#include <cstdint>

namespace Client::Actors {

enum class EmpireId : uint8_t { None = 0, Shinsoo = 1, Chunjo = 2, Jinno = 3 };
enum class PkMode : uint8_t { Peace = 0, Guild = 1, Free = 2, Revenge = 3 };

struct ActorRelationContext {
    uint32_t actorVid{0};
    EmpireId empire{EmpireId::None};
    uint32_t guildId{0};
    uint32_t partyId{0};
    PkMode pkMode{PkMode::Peace};
    bool isSafeZone{false};
    bool isDuelOpponent{false};
};

class ActorTargetRuleValidator {
public:
    ActorTargetRuleValidator() = default;
    ~ActorTargetRuleValidator() = default;

    [[nodiscard]] static bool CanAttackTarget(const ActorRelationContext& attacker, const ActorRelationContext& target) noexcept;
    [[nodiscard]] static bool IsEnemy(const ActorRelationContext& me, const ActorRelationContext& other) noexcept;
    [[nodiscard]] static bool IsFriendly(const ActorRelationContext& me, const ActorRelationContext& other) noexcept;
};

} // namespace Client::Actors
