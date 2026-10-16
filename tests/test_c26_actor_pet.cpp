#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorPetComponent.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_CASE("ActorPetComponent - Poprawne przywolanie zwierzaka") {
    ActorPetComponent petComponent;

    EntityVid testVid(123);
    ItemVnum testVnum(53001);

    auto result = petComponent.Summon(testVid, testVnum);
    
    CHECK(result.has_value());
    CHECK(petComponent.IsSummoned() == true);
    
    auto petVid = petComponent.GetPetVid();
    REQUIRE(petVid.has_value());
    CHECK(petVid.value() == testVid);

    auto petVnum = petComponent.GetPetVnum();
    REQUIRE(petVnum.has_value());
    CHECK(petVnum.value() == testVnum);
}

TEST_CASE("ActorPetComponent - Blad AlreadySummoned przy ponownym przywolaniu") {
    ActorPetComponent petComponent;

    EntityVid testVid(123);
    ItemVnum testVnum(53001);

    auto resultFirst = petComponent.Summon(testVid, testVnum);
    REQUIRE(resultFirst.has_value());

    EntityVid newVid(124);
    ItemVnum newVnum(53002);
    auto resultSecond = petComponent.Summon(newVid, newVnum);

    REQUIRE_FALSE(resultSecond.has_value());
    CHECK(resultSecond.error() == PetError::AlreadySummoned);
}

TEST_CASE("ActorPetComponent - Poprawne odwolanie zwierzaka") {
    ActorPetComponent petComponent;

    EntityVid testVid(123);
    ItemVnum testVnum(53001);

    petComponent.Summon(testVid, testVnum);
    REQUIRE(petComponent.IsSummoned() == true);

    auto result = petComponent.Unsummon();
    CHECK(result.has_value());
    CHECK(petComponent.IsSummoned() == false);
    CHECK_FALSE(petComponent.GetPetVid().has_value());
    CHECK_FALSE(petComponent.GetPetVnum().has_value());
}

TEST_CASE("ActorPetComponent - Blad NotSummoned przy probie odwolania bez zwierzaka") {
    ActorPetComponent petComponent;

    REQUIRE(petComponent.IsSummoned() == false);

    auto result = petComponent.Unsummon();

    REQUIRE_FALSE(result.has_value());
    CHECK(result.error() == PetError::NotSummoned);
}
