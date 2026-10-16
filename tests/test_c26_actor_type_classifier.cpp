#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorTypeClassifier.h"

TEST_CASE("ActorTypeClassifier: Classification and race checking") {
    using namespace Client::Actors;
    CHECK(ActorTypeClassifier::ClassifyByRace(0) == ActorCategory::PC);
    CHECK(ActorTypeClassifier::ClassifyByRace(8001) == ActorCategory::Stone);
    CHECK(ActorTypeClassifier::IsMetinStone(8027));
    CHECK(ActorTypeClassifier::ClassifyByRace(20001) == ActorCategory::NPC);
    CHECK(ActorTypeClassifier::ClassifyByRace(101) == ActorCategory::Monster);
    CHECK(ActorTypeClassifier::IsBossMonster(1093));
    CHECK(ActorTypeClassifier::IsAttackableCategory(ActorCategory::Monster));
    CHECK_FALSE(ActorTypeClassifier::IsAttackableCategory(ActorCategory::NPC));
}
