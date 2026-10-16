#include "ActorTargetRuleValidator.h"

namespace Client::Actors {

bool ActorTargetRuleValidator::CanAttackTarget(const ActorRelationContext& attacker, const ActorRelationContext& target) noexcept {
    if (attacker.actorVid == target.actorVid) return false;
    if (attacker.isSafeZone || target.isSafeZone) return false;
    if (target.isDuelOpponent) return true;
    if (attacker.partyId != 0 && attacker.partyId == target.partyId) return false;
    if (attacker.guildId != 0 && attacker.guildId == target.guildId) return false;
    if (attacker.empire != EmpireId::None && target.empire != EmpireId::None && attacker.empire != target.empire) {
        return true; // Rozne krolestwa moga sie atakowac
    }
    if (attacker.pkMode == PkMode::Free) return true;
    return false;
}

bool ActorTargetRuleValidator::IsEnemy(const ActorRelationContext& me, const ActorRelationContext& other) noexcept {
    return CanAttackTarget(me, other);
}

bool ActorTargetRuleValidator::IsFriendly(const ActorRelationContext& me, const ActorRelationContext& other) noexcept {
    if (me.actorVid == other.actorVid) return true;
    if (me.partyId != 0 && me.partyId == other.partyId) return true;
    if (me.guildId != 0 && me.guildId == other.guildId) return true;
    if (me.empire != EmpireId::None && me.empire == other.empire) return true;
    return false;
}

} // namespace Client::Actors
