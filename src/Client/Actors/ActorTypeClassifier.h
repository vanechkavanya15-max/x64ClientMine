#pragma once
#include <cstdint>
#include <string_view>
#include <type_traits>

namespace Client::Actors {

enum class ActorCategory : uint8_t {
    Unknown = 0,
    PC = 1,
    NPC = 2,
    Monster = 3,
    Stone = 4,
    Door = 5,
    Building = 6,
    Mount = 7,
    Pet = 8
};

enum class ActorRank : uint8_t {
    Pawn = 0,
    S_Pawn = 1,
    Knight = 2,
    S_Knight = 3,
    Boss = 4,
    King = 5
};

enum class ActorElement : uint8_t {
    None = 0,
    Fire = 1,
    Ice = 2,
    Earth = 3,
    Wind = 4,
    Lightning = 5,
    Dark = 6,
    Light = 7
};

enum class ActorRaceFlag : uint32_t {
    None     = 0,
    Animal   = 1 << 0,
    Undead   = 1 << 1,
    Devil    = 1 << 2,
    Human    = 1 << 3,
    Orc      = 1 << 4,
    Milgyo   = 1 << 5,
    Tree     = 1 << 6,
    Insect   = 1 << 7,
    Desert   = 1 << 8,
    Ice      = 1 << 9,
    Troll    = 1 << 10,
    Magic    = 1 << 11,
    Halfman  = 1 << 12,
    Beast    = 1 << 13,
    Demon    = 1 << 14,
    Zombie   = 1 << 15,
    Vampire  = 1 << 16,
    Ghost    = 1 << 17,
    Skeleton = 1 << 18,
    Golem    = 1 << 19,
    Giant    = 1 << 20,
    Elemental= 1 << 21,
    Aquatic  = 1 << 22,
    Flying   = 1 << 23,
    Dragon   = 1 << 24,
    Alien    = 1 << 25,
    Mech     = 1 << 26,
    Spirit   = 1 << 27,
    Mutant   = 1 << 28,
    Plant    = 1 << 29,
    God      = 1 << 30,
    All      = 0xFFFFFFFF
};

constexpr ActorRaceFlag operator|(ActorRaceFlag a, ActorRaceFlag b) noexcept {
    return static_cast<ActorRaceFlag>(static_cast<uint32_t>(a) | static_cast<uint32_t>(b));
}

constexpr ActorRaceFlag operator&(ActorRaceFlag a, ActorRaceFlag b) noexcept {
    return static_cast<ActorRaceFlag>(static_cast<uint32_t>(a) & static_cast<uint32_t>(b));
}

constexpr ActorRaceFlag operator^(ActorRaceFlag a, ActorRaceFlag b) noexcept {
    return static_cast<ActorRaceFlag>(static_cast<uint32_t>(a) ^ static_cast<uint32_t>(b));
}

constexpr ActorRaceFlag operator~(ActorRaceFlag a) noexcept {
    return static_cast<ActorRaceFlag>(~static_cast<uint32_t>(a));
}

constexpr ActorRaceFlag& operator|=(ActorRaceFlag& a, ActorRaceFlag b) noexcept {
    a = a | b;
    return a;
}

constexpr ActorRaceFlag& operator&=(ActorRaceFlag& a, ActorRaceFlag b) noexcept {
    a = a & b;
    return a;
}

constexpr ActorRaceFlag& operator^=(ActorRaceFlag& a, ActorRaceFlag b) noexcept {
    a = a ^ b;
    return a;
}

class ActorTypeClassifier {
public:
    ActorTypeClassifier() = delete;
    ~ActorTypeClassifier() = delete;

    [[nodiscard]] static ActorCategory ClassifyByRace(uint32_t raceVnum) noexcept;
    
    // Core categories
    [[nodiscard]] static bool IsPC(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsNPC(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsMonster(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsMetinStone(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsDoor(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsBuilding(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsMount(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsPet(uint32_t raceVnum) noexcept;
    
    [[nodiscard]] static bool IsBossMonster(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsAttackableCategory(ActorCategory cat) noexcept;

    // Advanced metadata
    [[nodiscard]] static ActorRank GetActorRank(uint32_t raceVnum) noexcept;
    [[nodiscard]] static ActorElement GetActorElement(uint32_t raceVnum) noexcept;
    [[nodiscard]] static ActorRaceFlag GetActorRaceFlags(uint32_t raceVnum) noexcept;
    
    [[nodiscard]] static bool IsHostile(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsFriendly(uint32_t raceVnum) noexcept;
    
    [[nodiscard]] static std::string_view GetRaceNameLiteral(uint32_t raceVnum) noexcept;
    
    // Behaviors
    [[nodiscard]] static bool CanHaveGuild(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsShopNPC(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsGuardNPC(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsImmuneToKnockback(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsImmuneToStun(uint32_t raceVnum) noexcept;
    
    // Utilities
    [[nodiscard]] static bool HasRaceFlag(uint32_t raceVnum, ActorRaceFlag flag) noexcept;
};

} // namespace Client::Actors
