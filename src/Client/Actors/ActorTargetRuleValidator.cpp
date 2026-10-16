#include "ActorTargetRuleValidator.h"

namespace Client::Actors {

bool ActorTargetRuleValidator::IsSameEmpire(const ActorRelationContext& a, const ActorRelationContext& b) noexcept {
    return (a.empire != EmpireId::None && a.empire == b.empire);
}

bool ActorTargetRuleValidator::IsSameGuild(const ActorRelationContext& a, const ActorRelationContext& b) noexcept {
    return (a.guildId != 0 && a.guildId == b.guildId);
}

bool ActorTargetRuleValidator::IsSameParty(const ActorRelationContext& a, const ActorRelationContext& b) noexcept {
    return (a.partyId != 0 && a.partyId == b.partyId);
}

bool ActorTargetRuleValidator::CanRevengeAttack(const ActorRelationContext& attacker, const ActorRelationContext& target) noexcept {
    // In Revenge mode, player can attack evil players from same empire, or any player from different empire
    if (target.alignmentType == AlignmentType::Evil) return true;
    if (!IsSameEmpire(attacker, target)) return true;
    return false;
}

bool ActorTargetRuleValidator::CanAttackTarget(const ActorRelationContext& attacker, const ActorRelationContext& target) noexcept {
    if (attacker.actorVid == target.actorVid) return false;

    // SafeZone check
    if (attacker.isSafeZone || target.isSafeZone) return false;

    // Duel check (overrides party/guild rules)
    if (target.isDuelOpponent) return true;

    // Level check: PvP protection under level 15
    // Note: Duel bypasses this (handled above). Empire wars might bypass this in full server, but we assume basic PK check here.
    if (attacker.level < 15 || target.level < 15) {
        // Different empires CAN attack each other even under level 15 in original game (sometimes), 
        // but PK (same empire) is blocked.
        if (IsSameEmpire(attacker, target)) {
            return false;
        }
    }

    // Party members cannot attack each other
    if (IsSameParty(attacker, target)) return false;

    // Guild War
    if (attacker.guildWarState == GuildWarState::Battle) {
        // We assume context sets guildWarState=Battle ONLY if the target is in the enemy guild of that war.
        // Or realistically, we check if they are in opposing guilds that are at war.
        // Here we simplify by relying on context fields if possible, or just saying:
        // if they are NOT same guild, and war state is battle (against each other).
        // Since we don't have target's war state against this specific guild in the simple struct, 
        // we assume the boolean context means "This target is in the guild we are at war with".
        // Let's rely on PK Mode Guild instead for general guild wars, or refine the war state.
    }

    if (attacker.pkMode == PkMode::Guild) {
        // Only attack enemies in guild war
        if (attacker.guildWarState == GuildWarState::Battle) {
            // Can attack enemy guild members during war
            return true;
        }
        // If not in battle, fall back to empire rules maybe? Or just block.
        // Usually Guild mode means you ONLY attack enemy guilds you are at war with.
        if (attacker.guildWarState != GuildWarState::Battle) {
            // Cannot attack same empire
            if (IsSameEmpire(attacker, target)) return false;
            return true; // Can attack other empires
        }
    }

    // Free Mode
    if (attacker.pkMode == PkMode::Free) {
        // Can attack anyone (except same party, which is blocked above)
        // Usually Free mode cannot attack same guild.
        if (IsSameGuild(attacker, target)) return false;
        return true;
    }

    // Revenge Mode
    if (attacker.pkMode == PkMode::Revenge) {
        if (IsSameGuild(attacker, target)) return false;
        return CanRevengeAttack(attacker, target);
    }

    // Peace Mode (Default)
    if (attacker.pkMode == PkMode::Peace) {
        if (IsSameEmpire(attacker, target)) return false;
        return true; // Can attack different empires
    }

    return false;
}

bool ActorTargetRuleValidator::IsEnemy(const ActorRelationContext& me, const ActorRelationContext& other) noexcept {
    return CanAttackTarget(me, other);
}

bool ActorTargetRuleValidator::IsFriendly(const ActorRelationContext& me, const ActorRelationContext& other) noexcept {
    if (me.actorVid == other.actorVid) return true;
    if (IsSameParty(me, other)) return true;
    if (IsSameGuild(me, other)) return true;
    if (IsSameEmpire(me, other)) return true;
    return false;
}

} // namespace Client::Actors
