#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorTypeClassifier.h"

TEST_CASE("ActorTypeClassifier: Core Categories") {
    using namespace Client::Actors;
    
    // PC
    CHECK(ActorTypeClassifier::IsPC(0));
    CHECK(ActorTypeClassifier::IsPC(8));
    CHECK_FALSE(ActorTypeClassifier::IsPC(9));
    CHECK(ActorTypeClassifier::ClassifyByRace(0) == ActorCategory::PC);

    // NPC
    CHECK(ActorTypeClassifier::IsNPC(9001));
    CHECK(ActorTypeClassifier::IsNPC(20016));
    CHECK(ActorTypeClassifier::ClassifyByRace(9001) == ActorCategory::NPC);

    // Monster
    CHECK(ActorTypeClassifier::IsMonster(101)); // Wild Dog
    CHECK(ActorTypeClassifier::IsMonster(1093)); // Spider Boss
    CHECK(ActorTypeClassifier::ClassifyByRace(101) == ActorCategory::Monster);

    // Metin Stone
    CHECK(ActorTypeClassifier::IsMetinStone(8001));
    CHECK(ActorTypeClassifier::IsMetinStone(8027));
    CHECK(ActorTypeClassifier::ClassifyByRace(8027) == ActorCategory::Stone);

    // Door
    CHECK(ActorTypeClassifier::IsDoor(13000));
    CHECK(ActorTypeClassifier::ClassifyByRace(13000) == ActorCategory::Door);

    // Building
    CHECK(ActorTypeClassifier::IsBuilding(14000));
    CHECK(ActorTypeClassifier::ClassifyByRace(14000) == ActorCategory::Building);

    // Mount
    CHECK(ActorTypeClassifier::IsMount(20100));
    CHECK(ActorTypeClassifier::ClassifyByRace(20100) == ActorCategory::Mount);

    // Pet
    CHECK(ActorTypeClassifier::IsPet(34000));
    CHECK(ActorTypeClassifier::ClassifyByRace(34000) == ActorCategory::Pet);
    
    // Unknown
    CHECK(ActorTypeClassifier::ClassifyByRace(999999) == ActorCategory::Unknown);
}

TEST_CASE("ActorTypeClassifier: Bosses and Attackable") {
    using namespace Client::Actors;
    
    CHECK(ActorTypeClassifier::IsBossMonster(191)); // Bera
    CHECK(ActorTypeClassifier::IsBossMonster(1093)); // Spider Queen
    CHECK(ActorTypeClassifier::IsBossMonster(2291)); // Red Dragon
    CHECK_FALSE(ActorTypeClassifier::IsBossMonster(101)); // Wild Dog

    CHECK(ActorTypeClassifier::IsAttackableCategory(ActorCategory::Monster));
    CHECK(ActorTypeClassifier::IsAttackableCategory(ActorCategory::Stone));
    CHECK(ActorTypeClassifier::IsAttackableCategory(ActorCategory::PC));
    CHECK_FALSE(ActorTypeClassifier::IsAttackableCategory(ActorCategory::NPC));
}

TEST_CASE("ActorTypeClassifier: Ranks and Elements") {
    using namespace Client::Actors;
    
    // Ranks
    CHECK(ActorTypeClassifier::GetActorRank(8001) == ActorRank::King); // Metin
    CHECK(ActorTypeClassifier::GetActorRank(1093) == ActorRank::Boss); // Spider Queen
    CHECK(ActorTypeClassifier::GetActorRank(101) == ActorRank::Pawn); // Last digit 1
    CHECK(ActorTypeClassifier::GetActorRank(102) == ActorRank::S_Pawn); // Last digit 2
    CHECK(ActorTypeClassifier::GetActorRank(104) == ActorRank::Knight); // Last digit 4
    CHECK(ActorTypeClassifier::GetActorRank(106) == ActorRank::S_Knight); // Last digit 6
    
    // Elements
    CHECK(ActorTypeClassifier::GetActorElement(2201) == ActorElement::Fire);
    CHECK(ActorTypeClassifier::GetActorElement(1901) == ActorElement::Ice);
    CHECK(ActorTypeClassifier::GetActorElement(2001) == ActorElement::Earth);
    CHECK(ActorTypeClassifier::GetActorElement(2101) == ActorElement::Wind);
    CHECK(ActorTypeClassifier::GetActorElement(2401) == ActorElement::Lightning);
    CHECK(ActorTypeClassifier::GetActorElement(2501) == ActorElement::Dark);
    CHECK(ActorTypeClassifier::GetActorElement(2601) == ActorElement::Light);
    CHECK(ActorTypeClassifier::GetActorElement(101) == ActorElement::None);
}

TEST_CASE("ActorTypeClassifier: Race Flags") {
    using namespace Client::Actors;
    
    CHECK(ActorTypeClassifier::HasRaceFlag(0, ActorRaceFlag::Human));
    CHECK(ActorTypeClassifier::HasRaceFlag(101, ActorRaceFlag::Animal)); // Dog
    CHECK(ActorTypeClassifier::HasRaceFlag(601, ActorRaceFlag::Orc)); // Orc
    CHECK(ActorTypeClassifier::HasRaceFlag(701, ActorRaceFlag::Milgyo)); // Esoteric
    CHECK(ActorTypeClassifier::HasRaceFlag(1001, ActorRaceFlag::Devil | ActorRaceFlag::Demon)); // Demon Tower
    CHECK(ActorTypeClassifier::HasRaceFlag(2501, ActorRaceFlag::Undead | ActorRaceFlag::Zombie)); // Zombie
    CHECK(ActorTypeClassifier::HasRaceFlag(2291, ActorRaceFlag::Dragon)); // Red Dragon
}

TEST_CASE("ActorTypeClassifier: Behaviors and Metadata") {
    using namespace Client::Actors;

    // Hostile / Friendly
    CHECK(ActorTypeClassifier::IsHostile(1093)); // Boss
    CHECK(ActorTypeClassifier::IsHostile(601)); // Orcs are hostile
    CHECK_FALSE(ActorTypeClassifier::IsHostile(101)); // Dogs are passive
    CHECK(ActorTypeClassifier::IsFriendly(9001)); // NPC
    CHECK(ActorTypeClassifier::IsFriendly(0)); // PC
    
    // Names
    CHECK(ActorTypeClassifier::GetRaceNameLiteral(0) == "Warrior Male");
    CHECK(ActorTypeClassifier::GetRaceNameLiteral(8001) == "Metin Stone");
    CHECK(ActorTypeClassifier::GetRaceNameLiteral(9001) == "NPC");
    
    // Guild
    CHECK(ActorTypeClassifier::CanHaveGuild(0));
    CHECK_FALSE(ActorTypeClassifier::CanHaveGuild(101));
    
    // Shops and Guards
    CHECK(ActorTypeClassifier::IsShopNPC(9001));
    CHECK(ActorTypeClassifier::IsGuardNPC(11000));
    
    // Immunities
    CHECK(ActorTypeClassifier::IsImmuneToKnockback(1093)); // Boss
    CHECK(ActorTypeClassifier::IsImmuneToKnockback(8001)); // Metin
    CHECK(ActorTypeClassifier::IsImmuneToKnockback(13000)); // Door
    CHECK_FALSE(ActorTypeClassifier::IsImmuneToKnockback(101)); // Dog
    
    CHECK(ActorTypeClassifier::IsImmuneToStun(1093)); // Boss
    CHECK(ActorTypeClassifier::IsImmuneToStun(8001)); // Metin
    CHECK_FALSE(ActorTypeClassifier::IsImmuneToStun(101)); // Dog
}
