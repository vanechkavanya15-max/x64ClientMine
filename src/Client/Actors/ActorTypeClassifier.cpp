#include "ActorTypeClassifier.h"

namespace Client::Actors {

ActorCategory ActorTypeClassifier::ClassifyByRace(uint32_t raceVnum) noexcept {
    if (raceVnum >= 0 && raceVnum <= 8) return ActorCategory::PC;
    if (raceVnum >= 8001 && raceVnum <= 8030) return ActorCategory::Stone;
    if (raceVnum >= 20000 && raceVnum < 30000) return ActorCategory::NPC;
    if (raceVnum >= 34000 && raceVnum <= 34100) return ActorCategory::Pet;
    if (raceVnum >= 20100 && raceVnum <= 20120) return ActorCategory::Mount;
    if (raceVnum >= 101 && raceVnum < 20000) return ActorCategory::Monster;
    return ActorCategory::Unknown;
}

bool ActorTypeClassifier::IsMetinStone(uint32_t raceVnum) noexcept {
    return raceVnum >= 8001 && raceVnum <= 8030;
}

bool ActorTypeClassifier::IsBossMonster(uint32_t raceVnum) noexcept {
    return (raceVnum == 1093 || raceVnum == 2291 || raceVnum == 2493);
}

bool ActorTypeClassifier::IsAttackableCategory(ActorCategory cat) noexcept {
    return (cat == ActorCategory::Monster || cat == ActorCategory::Stone || cat == ActorCategory::PC);
}

} // namespace Client::Actors
