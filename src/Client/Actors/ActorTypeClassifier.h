#pragma once
#include <cstdint>

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

class ActorTypeClassifier {
public:
    ActorTypeClassifier() = default;
    ~ActorTypeClassifier() = default;

    [[nodiscard]] static ActorCategory ClassifyByRace(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsMetinStone(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsBossMonster(uint32_t raceVnum) noexcept;
    [[nodiscard]] static bool IsAttackableCategory(ActorCategory cat) noexcept;
};

} // namespace Client::Actors
