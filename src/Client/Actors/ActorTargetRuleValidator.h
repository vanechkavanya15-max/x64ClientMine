#pragma once
#include <cstdint>

namespace Client::Actors {

enum class EmpireId : uint8_t { None = 0, Shinsoo = 1, Chunjo = 2, Jinno = 3 };
enum class PkMode : uint8_t { Peace = 0, Guild = 1, Free = 2, Revenge = 3 };
enum class AlignmentType : uint8_t { Good = 0, Neutral = 1, Evil = 2 };
enum class GuildWarState : uint8_t { None = 0, Declared = 1, Battle = 2, End = 3 };

struct ActorRelationContext {
    uint32_t actorVid{0};
    uint8_t level{0};
    EmpireId empire{EmpireId::None};
    uint32_t guildId{0};
    uint32_t partyId{0};
    PkMode pkMode{PkMode::Peace};
    int32_t alignment{0};
    AlignmentType alignmentType{AlignmentType::Neutral};
    bool isSafeZone{false};
    bool isDuelOpponent{false};
    GuildWarState guildWarState{GuildWarState::None};
};

class ActorTargetRuleValidator {
public:
    ActorTargetRuleValidator() = default;
    ~ActorTargetRuleValidator() = default;

    [[nodiscard]] static bool CanAttackTarget(const ActorRelationContext& attacker, const ActorRelationContext& target) noexcept;
    [[nodiscard]] static bool IsEnemy(const ActorRelationContext& me, const ActorRelationContext& other) noexcept;
    [[nodiscard]] static bool IsFriendly(const ActorRelationContext& me, const ActorRelationContext& other) noexcept;

private:
    [[nodiscard]] static bool IsSameEmpire(const ActorRelationContext& a, const ActorRelationContext& b) noexcept;
    [[nodiscard]] static bool IsSameGuild(const ActorRelationContext& a, const ActorRelationContext& b) noexcept;
    [[nodiscard]] static bool IsSameParty(const ActorRelationContext& a, const ActorRelationContext& b) noexcept;
    [[nodiscard]] static bool CanRevengeAttack(const ActorRelationContext& attacker, const ActorRelationContext& target) noexcept;
};

} // namespace Client::Actors
